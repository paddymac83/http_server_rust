#![allow(dead_code)]  // linter deactivation

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));  // reads env var at compile time
    let server = Server::new("127.0.0.1:8080".to_string());   // convert literal to String type
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Using the public path {}", public_path);
    server.run(WebsiteHandler::new(public_path));
}

