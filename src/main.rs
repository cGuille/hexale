#[macro_use]
extern crate log;
extern crate simplelog;

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use simplelog::*;
use iron::prelude::*;

use mount::Mount;
use router::Router;
use staticfile::Static;

use std::path::Path;

fn hello_world(request: &mut Request) -> IronResult<Response> {
    info!("Running hello_world handler, URL path: {}", request.url.path().join("/"));
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn main() {
    TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    let mut router = Router::new();
    router.get("/hello", hello_world, "hello");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("static")));
    mount.mount("/api", router);

    Iron::new(mount).http("localhost:3000").unwrap();
}
