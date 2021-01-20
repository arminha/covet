use anyhow::Result;
use base64::{self, URL_SAFE_NO_PAD};
use headers::{ETag, HeaderMapExt, IfNoneMatch};
use hyper::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION, CONTENT_TYPE};
use hyper::{Body, Response, StatusCode};
use log::{error, info};
use sha2::{Digest, Sha512Trunc256};
use time::OffsetDateTime;
use tokio::runtime::Runtime;
use warp::Filter;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::cli::Source;
use crate::message::scan_job::{ColorSpace, Format};
use crate::scanner::{self, Scanner, ScannerError};
use crate::util::scan_to_stream;

const INDEX_HTML: &[u8] = include_bytes!("resources/index.html");
const STYLE_CSS: &[u8] = include_bytes!("resources/style.css");
const ERROR_TEMPLATE: &str = include_str!("resources/error.html");

const TEXT_HTML: &str = "text/html";
const TEXT_CSS: &str = "text/css";

pub fn run_server(
    scanner_host: &str,
    listen_addr: &str,
    listen_port: u16,
    use_tls: bool,
) -> Result<()> {
    let addr = SocketAddr::new(listen_addr.parse()?, listen_port);
    println!("Running on http://{}:{}/", listen_addr, listen_port);
    let scanner = Scanner::new(scanner_host, use_tls);
    let rt = Runtime::new()?;
    rt.block_on(run_server_async(addr, scanner))
}

async fn run_server_async(addr: SocketAddr, scanner: Scanner) -> Result<()> {
    let scanner = Arc::new(scanner);
    let index = warp::get()
        .and(warp::path::end())
        .and(static_content(INDEX_HTML, TEXT_HTML));
    let css = warp::get()
        .and(warp::path("style.css"))
        .and(static_content(STYLE_CSS, TEXT_CSS));
    let scan = warp::post()
        .and(warp::path("scan"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and_then(move |params| handle_scan_form(scanner.clone(), params));

    let log = warp::log("covet::web");
    let routes = index.or(css).or(scan).with(log);
    warp::serve(routes).run(addr).await;
    Ok(())
}

fn compute_etag(content: &[u8]) -> String {
    let hash = Sha512Trunc256::digest(content);
    format!("\"{}\"", base64::encode_config(&hash[..], URL_SAFE_NO_PAD))
}

fn static_content(
    content: &'static [u8],
    content_type: &'static str,
) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    let etag: ETag = compute_etag(content).parse().expect("valid etag value");
    warp::header::headers_cloned()
        .map(move |headers: HeaderMap| {
            let if_none_match: Option<IfNoneMatch> = headers.typed_get();
            if let Some(if_none_match) = if_none_match {
                if !if_none_match.precondition_passes(&etag) {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_MODIFIED;
                    response.headers_mut().typed_insert(etag.clone());
                    return response;
                }
            }
            let mut response = Response::new(content.into());
            response.headers_mut().typed_insert(etag.clone());
            response
                .headers_mut()
                .insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
            response
        })
        .boxed()
}

async fn handle_scan_form(
    scanner: Arc<Scanner>,
    params: HashMap<String, String>,
) -> Result<Response<Body>, Infallible> {
    let format = get_format_param(&params);
    let color = get_colorspace_param(&params);
    let source = get_source_param(&params);
    let (resolution, quality) = get_quality_param(&params);
    let filename = scanner::output_file_name(format, &OffsetDateTime::now_utc());
    info!(
        "Scan parameters: format={:?}, color={:?}, source={:?}, resolution={}, quality={}",
        format, color, source, resolution, quality
    );
    let stream = match scan_to_stream(&scanner, format, color, source, resolution, quality).await {
        Ok(s) => s,
        Err(e) => return Ok(render_error(&e)),
    };
    let mut response = Response::new(Body::wrap_stream(stream));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, content_type(format));
    response
        .headers_mut()
        .insert(CONTENT_DISPOSITION, content_disposition(filename));
    Ok(response)
}

fn get_format_param(params: &HashMap<String, String>) -> Format {
    match params.get("format") {
        Some(pdf) if pdf == "pdf" => Format::Pdf,
        Some(jpeg) if jpeg == "jpeg" => Format::Jpeg,
        _ => Format::Pdf,
    }
}

fn get_colorspace_param(params: &HashMap<String, String>) -> ColorSpace {
    match params.get("colorspace") {
        Some(color) if color == "color" => ColorSpace::Color,
        Some(gray) if gray == "gray" => ColorSpace::Color,
        _ => ColorSpace::Color,
    }
}

fn get_source_param(params: &HashMap<String, String>) -> Source {
    match params.get("source") {
        Some(auto) if auto == "auto" => Source::auto,
        Some(adf) if adf == "adf" => Source::adf,
        Some(glass) if glass == "glass" => Source::glass,
        _ => Source::auto,
    }
}

fn get_quality_param(params: &HashMap<String, String>) -> (u32, u32) {
    match params.get("quality") {
        Some(auto) if auto == "base" => (300, 25),
        Some(adf) if adf == "high" => (600, 25),
        Some(glass) if glass == "best" => (600, 1),
        _ => (300, 25),
    }
}

fn content_type(format: Format) -> HeaderValue {
    match format {
        Format::Pdf => HeaderValue::from_static("application/pdf"),
        Format::Jpeg => HeaderValue::from_static("image/jpeg"),
    }
}

fn content_disposition(filename: String) -> HeaderValue {
    let mut value = "attachment; ".to_owned();
    if filename.is_ascii() {
        value += "filename=";
    } else {
        value += "filename*=utf-8''";
    }
    value += &filename;
    HeaderValue::from_str(&value).expect("valid header value")
}

fn render_error(error: &ScannerError) -> Response<Body> {
    match *error {
        ScannerError::AdfEmpty => error_page("ADF is empty"),
        ScannerError::Busy => error_page("Scanner is busy"),
        ScannerError::NotAvailable { ref source } => {
            error_page(format!("{}<p>Cause: {}</p>", error, source).as_str())
        }
        _ => {
            error!("InternalServerError: Failed to scan. {:?}", error);
            let mut response = Response::new(Body::empty());
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    }
}

fn error_page(error_message: &str) -> Response<Body> {
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

    const TEST_CONTENT: &[u8] = b"Hello world!";

    #[test]
    fn generate_etag() {
        assert_eq!(
            "\"-BYq1JGWwcEr3bz_HTYt2s8DriRranhkt1wkS5Zf5HU\"",
            compute_etag(TEST_CONTENT)
        );
    }

    #[test]
    fn generate_content_disposition() {
        assert_eq!(
            "attachment; filename=test.txt",
            content_disposition("test.txt".to_owned()).to_str().unwrap()
        );
        assert_eq!(
            "attachment; filename*=utf-8''äöü.txt",
            String::from_utf8_lossy(content_disposition("äöü.txt".to_owned()).as_bytes())
        );
    }
}
