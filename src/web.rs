use iron::{headers, status};
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

    fn index(req: &mut Request) -> IronResult<Response> {
        println!("{:?}", req);
        Ok(Response::with((status::Ok, Header(headers::ContentType::html()), INDEX_HTML)))
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
        Ok(Response::with((status::Ok, format!("Scanned documents {:?}", &parameters))))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
