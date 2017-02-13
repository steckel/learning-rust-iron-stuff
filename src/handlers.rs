extern crate iron;
extern crate router;

use iron::prelude::{IronError, IronResult, Request, Response};
use iron::status;
use router::Router;

use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

pub fn index(_: &mut Request) -> IronResult<Response> {
    Result::Ok(Response::with((status::Ok, "Hello World")))
}

pub fn query(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Result::Ok(Response::with((status::Ok, *query)))
}

pub fn error(_: &mut Request) -> IronResult<Response> {
    Result::Err(IronError::new(StringError("Error".to_string()), status::BadRequest))
}

pub fn catch_all(_: &mut Request) -> IronResult<Response> {
    Result::Ok(Response::with((status::NotFound)))
}
