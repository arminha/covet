extern crate iron;
extern crate router;

use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;

pub fn run_server() {
    println!("Running on http://localhost:3000/");

    let mut router = Router::new();
    router.get("/", index, "index");

    fn index(req: &mut Request) -> IronResult<Response> {
        println!("{:?}", req);
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
