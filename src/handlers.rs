extern crate iron;
extern crate router;

use std::error::Error;
use std::fmt::{self, Debug};

use iron::prelude::{IronError, IronResult, Request, Response};
use iron::headers::ContentType;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct TestStruct  {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
}

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
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2,3,4,5],
    };

    // Serialize using `json::encode`
    let encoded = json::encode(&object).unwrap();
    let content_type = ContentType::json();
    Result::Ok(Response::with((content_type.0, status::Ok, encoded)))
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
