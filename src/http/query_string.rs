use std::{collections::HashMap, intrinsics::unlikely};

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>  // key-value pair, with value linked to an enum
}

pub enum Value<'buf> {    // capture variaible types within query string
    Single(&str),
    Multiple(Vec<&'buf str>),    // Vec is a heap-allocated dynamic array..
}

impl<'buf> QueryString<'buf> {
    fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)   // returns ref to value specified by the key :-)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        QueryString { data }  // empty
    }

}