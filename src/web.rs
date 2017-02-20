use iron::status;
use iron::headers::{ContentDisposition, ContentType, DispositionType, DispositionParam, Charset};
use iron::modifiers::Header;
use iron::prelude::*;
use router::Router;
use time;
use urlencoded::UrlEncodedBody;

use std::collections::HashMap;

use message::scan_job::Format;
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
        println!("{:?}", req);
        let parameters = match req.get_ref::<UrlEncodedBody>() {
            Ok(hashmap) => hashmap,
            Err(ref e) => {
                println!("{:?}", e);
                return Ok(Response::with(status::BadRequest));
            }
        };
        let format = get_format_param(parameters);
        let filename = scanner::output_file_name(&format, &time::now());
        Ok(Response::with((status::Ok,
            Header(content_disposition(filename)),
            Header(content_type(&format)),
            format!("Scanned documents {:?}", &parameters))))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_format_param(parameters: &HashMap<String, Vec<String>>) -> Format {
    match parameters.get("format") {
        Some(values) => match values.first() {
            Some(pdf) if pdf == "pdf" => Format::Pdf,
            Some(jpeg) if jpeg == "jpeg" => Format::Jpeg,
            _ => Format::Pdf,
        },
        _ => Format::Pdf
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
