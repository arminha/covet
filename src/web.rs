use iron::status;
use iron::headers::{ContentDisposition, ContentType, DispositionType, DispositionParam, Charset};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::response::BodyReader;
use iron::{Handler, Timeouts};
use router::Router;
use time;
use urlencoded::UrlEncodedBody;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;

use cli::Source;
use message::scan_job::{ScanJob, ColorSpace, Format, InputSource};
use scanner;
use scanner::{Scanner, ScannerError};
use message::scan_status::AdfState;

const INDEX_HTML: &'static [u8] = include_bytes!("resources/index.html");
const STYLE_CSS: &'static [u8] = include_bytes!("resources/style.css");

pub fn run_server(scanner_host: &str, listen_port: u16) {
    println!("Running on http://localhost:{}/", listen_port);

    let scanner = Scanner::new(scanner_host);

    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/style.css", style, "style.css");
    router.post("/scan", scanner, "scan_post");

    fn index(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Header(ContentType::html()), INDEX_HTML)))
    }
    fn style(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Header(ContentType("text/css".parse().unwrap())), STYLE_CSS)))
    }

    let iron = Iron {
        handler: router,
        threads: 2,
        timeouts: Timeouts::default(),
    };
    iron.http(("localhost", listen_port)).unwrap();
}

impl Handler for Scanner {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = match req.get_ref::<UrlEncodedBody>() {
            Ok(hashmap) => hashmap,
            Err(e) => {
                println!("BadRequest: Failed to parse request parameters. {}", e);
                return Ok(Response::with(status::BadRequest));
            }
        };
        let format = get_format_param(params);
        let color_space = get_colorspace_param(params);
        let source = get_source_param(params);
        let filename = scanner::output_file_name(&format, &time::now());
        println!("Scan parameters: format={:?}, color={:?}, source={:?}",
                 format, color_space, source);
        let body = match do_scan(self, format, color_space, source) {
            Ok(body) => body,
            Err(e)   => return Ok(render_error(e))
        };
        Ok(Response::with((status::Ok,
            Header(content_disposition(filename)),
            Header(content_type(&format)),
            body)))
    }
}

fn do_scan(scanner: &Scanner, format: Format, color: ColorSpace, source: Source)
        -> Result<BodyReader<Box<Read + Send>>, ScannerError> {
    let status = scanner.get_scan_status()?;
    if !status.is_idle() {
        return Err(ScannerError::Busy);
    }
    let input_source = choose_source(source, status.adf_state())?;
    let mut job = scanner.start_job(ScanJob::new(input_source, 300, format, color))?;
    println!("Job: {:?}", job);
    loop {
        let ready = job.retrieve_status()?;
        if ready {
            println!("Job: {:?}", job);
            let reader = job.download_reader()?;
            return Ok(BodyReader(reader))
        }
        thread::sleep(Duration::from_millis(500));
    }
}

fn choose_source(source: Source, adf_state: AdfState) -> Result<InputSource, ScannerError> {
    let input_source = match source {
        Source::auto => {
            if adf_state == AdfState::Loaded {
                InputSource::Adf
            } else {
                InputSource::Platen
            }
        },
        Source::adf => {
            if adf_state == AdfState::Loaded {
                InputSource::Adf
            } else {
                return Err(ScannerError::AdfEmpty);
            }
        },
        Source::glass => InputSource::Platen
    };
    Ok(input_source)
}

fn get_format_param(params: &HashMap<String, Vec<String>>) -> Format {
    match params.get("format") {
        Some(values) => match values.first() {
            Some(pdf) if pdf == "pdf" => Format::Pdf,
            Some(jpeg) if jpeg == "jpeg" => Format::Jpeg,
            _ => Format::Pdf,
        },
        _ => Format::Pdf
    }
}

fn get_colorspace_param(params: &HashMap<String, Vec<String>>) -> ColorSpace {
    match params.get("colorspace") {
        Some(values) => match values.first() {
            Some(color) if color == "color" => ColorSpace::Color,
            Some(gray) if gray == "gray" => ColorSpace::Color,
            _ => ColorSpace::Color,
        },
        _ => ColorSpace::Color
    }
}

fn get_source_param(params: &HashMap<String, Vec<String>>) -> Source {
    match params.get("source") {
        Some(values) => match values.first() {
            Some(auto) if auto == "auto" => Source::auto,
            Some(adf) if adf == "adf" => Source::adf,
            Some(glass) if glass == "glass" => Source::glass,
            _ => Source::auto,
        },
        _ => Source::auto
    }
}

fn content_type(format: &Format) -> ContentType {
    match format {
        &Format::Pdf => ContentType("application/pdf".parse().unwrap()),
        &Format::Jpeg => ContentType::jpeg()
    }
}

fn content_disposition(filename: String) -> ContentDisposition {
    ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(
            Charset::Ext("UTF-8".to_owned()),
            None,
            filename.into_bytes()
        )]
    }
}

fn render_error(error: ScannerError) -> Response {
    match error {
        ScannerError::AdfEmpty => Response::with((status::Ok, "ADF is empty")),
        ScannerError::Busy => Response::with((status::Ok, "Scanner is busy")),
        ScannerError::NotAvailable(_) => Response::with((status::Ok, format!("{}", error))),
        _ => {
            println!("InternalServerError: Failed to scan. {:?}", error);
            Response::with(status::InternalServerError)
        }
    }
}
