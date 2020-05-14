/*
Copyright (C) 2019  Armin Häberling

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>
*/
use base64::{self, URL_SAFE_NO_PAD};
use iron::headers::{
    Charset, ContentDisposition, ContentType, DispositionParam, DispositionType, ETag, EntityTag,
    IfNoneMatch,
};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::response::BodyReader;
use iron::status;
use iron::{Handler, Timeouts};
use router::Router;
use sha2::{Digest, Sha512Trunc256};
use time::OffsetDateTime;
use urlencoded::UrlEncodedBody;

use std::collections::HashMap;
use std::io::Read;
use std::thread;
use std::time::Duration;

use crate::cli::Source;
use crate::message::scan_job::{ColorSpace, Format, InputSource, ScanJob};
use crate::message::scan_status::AdfState;
use crate::scanner;
use crate::scanner::{Scanner, ScannerError};

const INDEX_HTML: &[u8] = include_bytes!("resources/index.html");
const STYLE_CSS: &[u8] = include_bytes!("resources/style.css");
const ERROR_TEMPLATE: &str = include_str!("resources/error.html");

struct StaticContent {
    content: &'static [u8],
    content_type: ContentType,
    etag: EntityTag,
}

pub fn run_server(
    scanner_host: &str,
    listen_addr: &str,
    listen_port: u16,
    use_tls: bool,
) -> Result<(), iron::error::HttpError> {
    println!("Running on http://{}:{}/", listen_addr, listen_port);

    let scanner = Scanner::new(scanner_host, use_tls);

    let mut router = Router::new();
    router.get(
        "/",
        StaticContent::new(INDEX_HTML, ContentType::html()),
        "index",
    );
    router.get(
        "/style.css",
        StaticContent::new(STYLE_CSS, ContentType("text/css".parse().unwrap())),
        "style.css",
    );
    router.post("/scan", scanner, "scan_post");

    let iron = Iron {
        handler: router,
        threads: 2,
        timeouts: Timeouts::default(),
    };
    iron.http((listen_addr, listen_port))?;
    Ok(())
}

impl StaticContent {
    fn new(content: &'static [u8], content_type: ContentType) -> Self {
        let mut hasher = Sha512Trunc256::default();
        hasher.input(content);
        let hash = hasher.result();
        let etag = EntityTag::strong(base64::encode_config(&hash[..], URL_SAFE_NO_PAD));
        StaticContent {
            content,
            content_type,
            etag,
        }
    }

    fn etag_header(&self) -> Header<ETag> {
        Header(ETag(self.etag.clone()))
    }

    fn content_type_header(&self) -> Header<ContentType> {
        Header(self.content_type.clone())
    }
}

impl Handler for StaticContent {
    fn handle(&self, req: &mut Request<'_, '_>) -> IronResult<Response> {
        if let Some(if_none_match) = req.headers.get::<IfNoneMatch>() {
            let tag_matches = match *if_none_match {
                IfNoneMatch::Any => true,
                IfNoneMatch::Items(ref tags) => tags.iter().any(|t| self.etag.weak_eq(t)),
            };
            if tag_matches {
                return Ok(Response::with((status::NotModified, self.etag_header())));
            }
        }
        Ok(Response::with((
            status::Ok,
            self.content_type_header(),
            self.etag_header(),
            self.content,
        )))
    }
}

impl Handler for Scanner {
    fn handle(&self, req: &mut Request<'_, '_>) -> IronResult<Response> {
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
        let (resolution, compression) = get_quality_param(params);
        let filename = scanner::output_file_name(format, &OffsetDateTime::now_utc());
        println!(
            "Scan parameters: format={:?}, color={:?}, source={:?}, resolution={}, compression={}",
            format, color_space, source, resolution, compression
        );
        let body = match do_scan(self, format, color_space, source, resolution, compression) {
            Ok(body) => body,
            Err(e) => return Ok(render_error(&e)),
        };
        Ok(Response::with((
            status::Ok,
            Header(content_disposition(filename)),
            Header(content_type(format)),
            body,
        )))
    }
}

fn do_scan(
    scanner: &Scanner,
    format: Format,
    color: ColorSpace,
    source: Source,
    resolution: u32,
    compression: u32,
) -> Result<BodyReader<Box<dyn Read + Send>>, ScannerError> {
    let status = scanner.get_scan_status()?;
    if !status.is_idle() {
        return Err(ScannerError::Busy);
    }
    let input_source = choose_source(source, status.adf_state())?;
    let mut job = scanner.start_job(ScanJob::new(
        input_source,
        resolution,
        compression,
        format,
        color,
    ))?;
    println!("Job: {:?}", job);
    loop {
        let ready = job.retrieve_status()?;
        if ready {
            println!("Job: {:?}", job);
            let reader = job.download_reader()?;
            return Ok(BodyReader(reader));
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
        }
        Source::adf => {
            if adf_state == AdfState::Loaded {
                InputSource::Adf
            } else {
                return Err(ScannerError::AdfEmpty);
            }
        }
        Source::glass => InputSource::Platen,
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
        _ => Format::Pdf,
    }
}

fn get_colorspace_param(params: &HashMap<String, Vec<String>>) -> ColorSpace {
    match params.get("colorspace") {
        Some(values) => match values.first() {
            Some(color) if color == "color" => ColorSpace::Color,
            Some(gray) if gray == "gray" => ColorSpace::Color,
            _ => ColorSpace::Color,
        },
        _ => ColorSpace::Color,
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
        _ => Source::auto,
    }
}

fn get_quality_param(params: &HashMap<String, Vec<String>>) -> (u32, u32) {
    match params.get("quality") {
        Some(values) => match values.first() {
            Some(auto) if auto == "base" => (300, 25),
            Some(adf) if adf == "high" => (600, 25),
            Some(glass) if glass == "best" => (600, 1),
            _ => (300, 25),
        },
        _ => (300, 25),
    }
}

fn content_type(format: Format) -> ContentType {
    match format {
        Format::Pdf => ContentType("application/pdf".parse().unwrap()),
        Format::Jpeg => ContentType::jpeg(),
    }
}

fn content_disposition(filename: String) -> ContentDisposition {
    ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(
            Charset::Ext("UTF-8".to_owned()),
            None,
            filename.into_bytes(),
        )],
    }
}

fn render_error(error: &ScannerError) -> Response {
    match *error {
        ScannerError::AdfEmpty => error_page("ADF is empty"),
        ScannerError::Busy => error_page("Scanner is busy"),
        ScannerError::NotAvailable { ref source } => {
            error_page(format!("{}<p>Cause: {}</p>", error, source).as_str())
        }
        _ => {
            println!("InternalServerError: Failed to scan. {:?}", error);
            Response::with(status::InternalServerError)
        }
    }
}

fn error_page(error_message: &str) -> Response {
    let page = ERROR_TEMPLATE.replace("{error_message}", error_message);
    Response::with((status::Ok, Header(ContentType::html()), page))
}

#[cfg(test)]
mod test {

    use super::*;

    const TEST_CONTENT: &[u8] = b"Hello world!";

    #[test]
    fn static_content_generate_etag() {
        let sc = StaticContent::new(TEST_CONTENT, ContentType::plaintext());
        assert_eq!("-BYq1JGWwcEr3bz_HTYt2s8DriRranhkt1wkS5Zf5HU", sc.etag.tag());
    }
}
