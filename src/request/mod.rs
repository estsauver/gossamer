use super::headers::Headers;
use self::method::Method;
use std::io::{Cursor, Result};
use std::io::prelude::*;
use regex::Regex;

mod method;

pub struct Request<'a> {
  headers: Headers<'a>,
  pub method: Method
}

impl <'a> Request<'a> {

  pub fn make<'a>(reader: &mut Read, buffer: &'a mut [u8]) -> Option<Request<'a>> {
    let result = reader.read(buffer);
    let mut str_buff = String::from_utf8_lossy(buffer).into_owned();
    match Method::make(&str_buff) {
      Some((http_method, rest)) => Some(Request { headers: Headers::empty(), method: http_method } ),
      None => None
   }

  }
}


#[cfg(test)]
mod tests {
  extern crate test;
  use super::*;
  use std::io::{Cursor, Read};
  use test::Bencher;

  fn test_req<F: Fn(Request) -> ()>(input:&str, assertions: F) {
    let mut buf = [0u8; 4096];
    let mut reader = Cursor::new(input.as_bytes());
    let req = Request::make(&mut reader, &mut buf).unwrap();
    assertions(req)
  }
}
