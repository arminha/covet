use anyhow::Result;
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use headers::HeaderMapExt;
use hyper::{
    header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE},
    StatusCode,
};
use jiff::Timestamp;
use log::{error, info};
use serde::Deserialize;
use tokio::runtime::Runtime;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use std::net::SocketAddr;
use std::sync::{Arc, LazyLock};

use crate::cli::Source;
use crate::message::scan_job::{ColorSpace, Format};
use crate::scanner::{self, Scanner, ScannerError};
use crate::util::scan_to_stream;
use crate::web::static_content::StaticContent;

mod static_content;

const ERROR_TEMPLATE: &str = include_str!("../resources/error.html");

const TEXT_HTML: &str = "text/html";
const TEXT_CSS: &str = "text/css";

static INDEX_HTML: LazyLock<StaticContent> =
    LazyLock::new(|| StaticContent::new(include_str!("../resources/index.html"), TEXT_HTML));
static STYLE_CSS: LazyLock<StaticContent> =
    LazyLock::new(|| StaticContent::new(include_str!("../resources/style.css"), TEXT_CSS));

pub fn run_server(
    scanner_host: &str,
    listen_addr: &str,
    listen_port: u16,
    use_tls: bool,
) -> Result<()> {
    let addr = SocketAddr::new(listen_addr.parse()?, listen_port);
    println!("Running on http://{listen_addr}:{listen_port}/");
    let scanner = Scanner::new(scanner_host, use_tls);
    let rt = Runtime::new()?;
    rt.block_on(run_server_async(addr, scanner))
}

async fn run_server_async(addr: SocketAddr, scanner: Scanner) -> Result<()> {
    let scanner = Arc::new(scanner);

    let app = Router::new()
        .route("/", get(index))
        .route("/style.css", get(style_css))
        .route("/scan", post(handle_scan_form))
        .layer(DefaultBodyLimit::max(1024 * 32))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(scanner);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index(headers: HeaderMap) -> impl IntoResponse {
    INDEX_HTML.get_request(headers.typed_get())
}

async fn style_css(headers: HeaderMap) -> impl IntoResponse {
    STYLE_CSS.get_request(headers.typed_get())
}

#[derive(Deserialize, Debug)]
struct ScanInput {
    format: Option<Format>,
    colorspace: Option<ColorSpace>,
    source: Option<Source>,
    quality: Option<QualityProfile>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum QualityProfile {
    Base,
    High,
    Best,
}

impl QualityProfile {
    fn resolution(&self) -> u32 {
        match self {
            Self::Base => 300,
            Self::High => 600,
            Self::Best => 600,
        }
    }

    fn quality(&self) -> u32 {
        match self {
            Self::Base => 25,
            Self::High => 25,
            Self::Best => 1,
        }
    }
}

async fn handle_scan_form(
    State(scanner): State<Arc<Scanner>>,
    Form(input): Form<ScanInput>,
) -> impl IntoResponse {
    let format = input.format.unwrap_or(Format::Pdf);
    let color = input.colorspace.unwrap_or(ColorSpace::Color);
    let source = input.source.unwrap_or(Source::Auto);
    let quality = input.quality.unwrap_or(QualityProfile::Base);
    let resolution = quality.resolution();
    let quality = quality.quality();
    info!(
        "Scan parameters: format={format:?}, color={color:?}, source={source:?}, resolution={resolution}, quality={quality}"
    );
    let stream = match scan_to_stream(&scanner, format, color, source, resolution, quality).await {
        Ok(s) => s,
        Err(e) => return render_error(&e),
    };
    let mut response = Response::new(Body::from_stream(stream));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, content_type(format));
    let filename = scanner::output_file_name(format, &Timestamp::now());
    response
        .headers_mut()
        .insert(CONTENT_DISPOSITION, content_disposition(&filename));
    response
}

fn content_type(format: Format) -> HeaderValue {
    match format {
        Format::Pdf => HeaderValue::from_static("application/pdf"),
        Format::Jpeg => HeaderValue::from_static("image/jpeg"),
    }
}

fn content_disposition(filename: &str) -> HeaderValue {
    let mut value = "attachment; ".to_owned();
    if filename.is_ascii() {
        value += "filename=";
    } else {
        value += "filename*=utf-8''";
    }
    value += filename;
    HeaderValue::from_str(&value).expect("valid header value")
}

fn render_error(error: &ScannerError) -> Response<Body> {
    match *error {
        ScannerError::AdfEmpty => error_page("ADF is empty"),
        ScannerError::Busy => error_page("Scanner is busy"),
        ScannerError::NotAvailable { ref source } => {
            error_page(format!("{error}<p>Cause: {source}</p>").as_str())
        }
        ScannerError::Parse {
            ref source,
            data: _,
        } => error_page(format!("{error}<p>Cause: {source}</p>").as_str()),
        ScannerError::Canceled => error_page("Scan cancelled"),
        _ => {
            error!("InternalServerError: Failed to scan. {error:?}");
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    }
}

fn error_page(error_message: &str) -> Response<Body> {
    #[allow(clippy::all)]
    let page = ERROR_TEMPLATE.replace("{error_message}", error_message);
    let mut response = Response::new(Body::from(page));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static(TEXT_HTML));
    response
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn generate_content_disposition() {
        assert_eq!(
            "attachment; filename=test.txt",
            content_disposition("test.txt").to_str().unwrap()
        );
        assert_eq!(
            "attachment; filename*=utf-8''äöü.txt",
            String::from_utf8_lossy(content_disposition("äöü.txt").as_bytes())
        );
    }
}
