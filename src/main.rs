#[macro_use] extern crate log;
extern crate simplelog;
extern crate iron;

use simplelog::*;
use iron::prelude::*;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    info!("hello_world");
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn main() {
    TermLogger::init(LogLevelFilter::Info, Config::default()).unwrap();

    let chain = Chain::new(hello_world);
    Iron::new(chain).http("localhost:3000").unwrap();
}
