use super::server::Handler;
use super::http::{Response, Request, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        println!("Using the public path {}", path);
        fs::read_to_string(path).ok() // reads entire file to string and returns a result,conver to option
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &crate::http::Request) -> crate::http::Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },

            }
            _ => Response::new(StatusCode::NotFound, None),
        }
        
    }
}