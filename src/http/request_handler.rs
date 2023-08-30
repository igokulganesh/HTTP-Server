use super::{Request, Response, ParseError, StatusCode, Method};

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

        match request.method() {
            Method::GET => match request.path() {
                    "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                    "/about" => Response::new(StatusCode::Ok, Some("<h1>Hello World!</h1>".to_string())),
                    _ => Response::new(StatusCode::NotFound, Some("<h1>Page Not Found 404</h1>".to_string())), 
                },
            _ => Response::new(StatusCode::NotFound, Some("<h1>Page Not Found 404</h1>".to_string())),
        }
    }
}