use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug)]   // implenent Copy trait for the StatusCode integers
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)  // conver the status code to integer, created a copy so that we don't have to affect the self ref
    }
}