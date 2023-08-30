use super::{Request, Response, ParseError, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response; 

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to Parse the Request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    } 
}

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>It works</h1>".to_string())) 
    }
}