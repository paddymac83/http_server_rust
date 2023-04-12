use super::StatusCode;
use std::{fmt::{Display, Formatter, Result as FmtResult, write}, net::TcpStream};
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String> ,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {   // impl static dispatch to make response.send more generic
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, 
        self.status_code.reason_phrase(), 
        body)
        
    }
}
