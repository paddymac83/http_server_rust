use super::server::Handler;
use super::http::{Response, Request, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &crate::http::Request) -> crate::http::Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
        
    }
}