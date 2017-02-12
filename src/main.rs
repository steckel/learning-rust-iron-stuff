extern crate iron;
extern crate router;

mod handlers;

use iron::prelude::{Iron};
use router::Router;


fn main() {
    let mut router = Router::new();
    router.get("/", handlers::index_handler, "index");
    router.get("/:query", handlers::query_handler, "query");

    Iron::new(router).http("localhost:3000").unwrap();
}
