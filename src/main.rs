extern crate iron;
extern crate router;
extern crate logger;
extern crate env_logger;

mod handlers;

use iron::prelude::{Iron, Chain};
use router::Router;
use logger::Logger;


fn main() {
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/", handlers::index, "index");
    router.get("/query/:query", handlers::query, "query");
    router.get("/error", handlers::error, "error");
    router.get("/*", handlers::catch_all, "catch_all");

    let (logger_before, logger_after) = Logger::new(None);
    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    println!("Run `RUST_LOG=logger=info cargo run` to see logs.");
    match Iron::new(chain).http("localhost:3000") {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
