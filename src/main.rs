#![allow(dead_code)]  // linter activation

use server::Server;

use http::Request;
use http::Method;

mod server;
mod http;

fn main() {

    let server = Server::new("127.0.0.1:8080".to_string());   // convert literal to String type
    server.run();
}

