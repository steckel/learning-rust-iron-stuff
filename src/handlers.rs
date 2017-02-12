extern crate iron;
extern crate router;

use iron::prelude::{IronResult, Request, Response};
use iron::status;
use router::Router;

pub fn index_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World")))
}

pub fn query_handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}
