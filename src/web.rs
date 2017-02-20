use iron::status;
use iron::headers::{ContentDisposition, ContentType, DispositionType, DispositionParam, Charset};
use iron::mime::Mime;
use iron::modifiers::Header;
use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedBody;

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
        let content_disposition = content_disposition_attachment("test.pdf".to_owned());
        let mime: Mime = "application/pdf".parse().unwrap();
        let content_type = Header(ContentType(mime));
        Ok(Response::with((status::Ok, content_disposition, content_type,
            format!("Scanned documents {:?}", &parameters))))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}

fn content_disposition_attachment(filename: String) -> Header<ContentDisposition> {
    Header(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![DispositionParam::Filename(
            Charset::Ext("UTF-8".to_owned()),
            None,
            filename.into_bytes()
        )]
    })
}
