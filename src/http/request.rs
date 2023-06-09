use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter, Debug};
use std::str;
use std::str::Utf8Error;
use super::{QueryString};

#[derive(Debug)]   // derive the debug trait for the request type, ! means it applies to the entire module

pub struct Request<'buf> {   // generic over lifetime 'a, lifencestime of the buffer it refere
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: super::method::Method,
}

// getters
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()   // takes a ref to the value wrapped inside an Option
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;   // custom error type

    // GET /search?name=abc&sort=1 HTTP/1.1
    
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {   // the buf ref should live as long as the return type
        let request = str::from_utf8(buf)?;  // err is linked to ParseError with From<Utf8Error> trait defined

        // note the spaces leave the path/query string still connected by a ?
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;  // convert Option to Result
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // check its 1.1
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;   // impl FromStr trait on method enum, we get the .parse func for free

        let mut query_string = None;

        if let Some(i) = path.find("?"){
            query_string = Some(QueryString::from(&path[i+1..]));   // everything after the first ? mark 
            path = &path[..i];
        }
        // return request value (Ok(Self))
        Ok(Self {
            path,   // expects a string, not a str slice
            query_string,             // expects Option(String)
            method
        })
    }
}

// helper to extract words from request
fn get_next_word(request: &str) -> Option<(&str, &str)> {   // to track pos in request

    for (i, c) in request.chars().enumerate() {   // note "" and '' are diffenet, as former is string 
        if c == ' ' || c == '\r'  {
            return Some((&request[..i], &request[i+1..]));   // return up to the first space. in tuple
        }
    }
    None
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
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidCoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        ParseError::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())  // ewtite! macro to formatter with parseerror message
    }  // alias for the fmt::Result return type
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())  // ewtite! macro to formatter with parseerror message
    }  // alias for the fmt::Result return type
}

impl Error for ParseError {

}