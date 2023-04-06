use super::method::Method;
use std::convert::TryFrom;

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
    type Error = String;
    
    fn try_from(buf: &[u8]) -> <Self, Self::Error> {
        unimplemented!()
    }

}
