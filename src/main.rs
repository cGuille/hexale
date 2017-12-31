#[macro_use]
extern crate log;
extern crate simplelog;

extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate unicode_normalization;

use iron::prelude::*;
use mount::Mount;
use router::Router;
use simplelog::*;
use staticfile::Static;
use std::ascii::AsciiExt;
use std::path::Path;
use unicode_normalization::UnicodeNormalization;

fn hello_world(request: &mut Request) -> IronResult<Response> {
    info!("Running hello_world handler, URL path: {}", request.url.path().join("/"));
    Ok(Response::with((iron::status::Ok, normalize(String::from("Hello World")))))
}

// This function can be used to check guesses: instead of comparing "accusé"
// with "ACCUSE", compare normalize("accusé") with normalize("ACCUSE").
fn normalize(input: String) -> String {
    input.to_lowercase().nfkd().filter(|char| char.is_ascii()).collect()
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
