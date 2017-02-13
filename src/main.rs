extern crate iron;
extern crate router;

mod handlers;

use iron::prelude::{Iron};
use router::Router;


fn main() {
    let mut router = Router::new();
    router.get("/", handlers::index, "index");
    router.get("/query/:query", handlers::query, "query");
    router.get("/error", handlers::error, "error");
    router.get("/*", handlers::catch_all, "catch_all");

    Iron::new(router).http("localhost:3000").unwrap();
}
