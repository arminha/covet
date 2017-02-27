use iron::status;
use iron::headers::{ContentDisposition, ContentType, DispositionType, DispositionParam, Charset};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::Timeouts;
use router::Router;
use time;
use urlencoded::UrlEncodedBody;

use std::collections::HashMap;

use cli::Source;
use message::scan_job::{ColorSpace, Format};
use scanner;

const INDEX_HTML: &'static [u8] = include_bytes!("resources/index.html");

pub fn run_server() {
    println!("Running on http://localhost:3000/");

    let mut router = Router::new();
    router.get("/", index, "index");
    router.post("/scan", scan, "scan_post");

    fn index(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, Header(ContentType::html()), INDEX_HTML)))
    }

    fn scan(req: &mut Request) -> IronResult<Response> {
        let params = match req.get_ref::<UrlEncodedBody>() {
            Ok(hashmap) => hashmap,
            Err(ref e) => {
                println!("{:?}", e);
                return Ok(Response::with(status::BadRequest));
            }
        };
        let format = get_format_param(params);
        let color_space = get_colorspace_param(params);
        let source = get_source_param(params);
        let filename = scanner::output_file_name(&format, &time::now());
        println!("format: {:?}, color: {:?}, source: {:?}", format, color_space, source);
        Ok(Response::with((status::Ok,
            Header(content_disposition(filename)),
            Header(content_type(&format)),
            format!("Scanned documents {:?}", &params))))
    }

    let iron = Iron {
        handler: router,
        threads: 4,
        timeouts: Timeouts::default(),
    };
    iron.http("localhost:3000").unwrap();
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
