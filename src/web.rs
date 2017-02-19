extern crate iron;
extern crate router;

use self::iron::{headers, status};
use self::iron::modifiers::Header;
use self::iron::prelude::*;
use self::router::Router;

const INDEX_HTML: &'static [u8] = include_bytes!("resources/index.html");

pub fn run_server() {
    println!("Running on http://localhost:3000/");

    let mut router = Router::new();
    router.get("/", index, "index");

    fn index(req: &mut Request) -> IronResult<Response> {
        println!("{:?}", req);
        Ok(Response::with((status::Ok, Header(headers::ContentType::html()), INDEX_HTML)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
