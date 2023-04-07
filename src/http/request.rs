use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: super::method::Method,
}

impl Request {
    fn bytes_from_array(buf: &[u8]) -> Result<Self, String> {  // convert buffer to Request type

    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;   // custom error type

    // GET /search?name=abc&sort=1 HTTP/1.1
    
    fn try_from(buf: &[u8]) -> <Self, Self::Error> {
        let request = str::from_utf8(buf)?  // err is linked to ParseError with From<Utf8Error> trait defined
        unimplemented!()
    }

}

// enum to capture error types
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// implement functions for an enum :-)
impl ParseError {
    fn message(&self) -> &str {
        match self {  // its an enum
            self::InvalidRequest => "InvalidRequest",
            self::InvalidCoding => "InvalidCoding",
            self::InvalidProtocol => "InvalidProtocol",
            self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())  // ewtite! macro to formatter with parseerror message
    };  // alias for the fmt::Result return type
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())  // ewtite! macro to formatter with parseerror message
    };  // alias for the fmt::Result return type
}

impl Error for ParseError {

}