use super::headers::Headers;
use std::io::{Cursor, Result};
use std::io::prelude::*;
use regex::Regex;


pub struct Request<'a> {
  headers: Headers<'a>,
  pub method: Method
}

/// The Http method of the request
#[derive(PartialEq, Debug)]
pub enum Method {
  GET,
  HEAD,
  POST,
  PUT,
  DELETE,
  CONNECT,
  OPTIONS,
  TRACE
}

impl Method {

  /// This gets a request method from a given request line string. 
  /// It could proably be converted to use case insensitive regexes/a faster match then the regexes.
  /// These regexe's are all compiled, which does increase the size of the binary.
  pub fn make<'a>(input: &'a str) -> Option<(Method, &'a str)> {
    Method::match_method(input, regex!(r"^[Gg][Ee][Tt] "), Method::GET).or_else(|| {
      Method::match_method(input, regex!(r"^[Pp][Oo][Ss][Tt] "), Method::POST).or_else(|| {
        Method::match_method(input, regex!(r"^[Pp][Uu][Tt] "), Method::PUT).or_else(|| {
          Method::match_method(input, regex!(r"^[Hh][Ee][Aa][Dd] "), Method::HEAD).or_else(|| {
            Method::match_method(input, regex!(r"^[Dd][Ee][Ll][Ee][Tt][Ee] "), Method::DELETE).or_else(|| {
              Method::match_method(input, regex!(r"^[Cc][Oo][Nn][Nn][Ee][Cc][Tt] "), Method::CONNECT).or_else(|| {
                Method::match_method(input, regex!(r"^[Oo][Pp][Tt][Ii][Oo][Nn][Ss] "), Method::OPTIONS).or_else(|| {
                  Method::match_method(input, regex!(r"^[Tt][Rr][Aa][Cc][Ee] "), Method::TRACE)
                })
              })
            })
          })
        })
      })
    })
  }

  fn match_method<'a>(input: &'a str, regex: Regex, method: Method) -> Option<(Method, &'a str)> {
    match regex.find(input) {
      Some((_,y)) => Some((method, &input[y ..])),
      None => None
    }
  }

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
  
  #[test]
  fn it_parses_an_http_get() {
    test_req("GET / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::GET);
    });
  }
    
  #[test]
  fn it_parses_an_http_head() {
    test_req("HEAD / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::HEAD);
    });
  }

  #[test]
  fn it_parses_an_http_post() {
    test_req("POST / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::POST)
    });
  }

  #[test]
  fn it_parses_an_http_put() {
    test_req("PUT / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::PUT);
    });
  }

  #[test]
  fn it_parses_an_http_delete() {
    test_req("DELETE / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::DELETE);
    });
  }

  #[test]
  fn it_parses_an_http_connect() {
    test_req("CONNECT / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::CONNECT);
    });
  }

  #[test]
  fn it_parses_an_http_options() {
    test_req("OPTIONS / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::OPTIONS);
    });
  }

  #[test]
  fn it_parses_an_http_trace() {
    test_req("TRACE / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::TRACE);
    });
  }

  #[test]
  fn it_handles_case_insensitive_versions() {
    test_req("gEt / HTTP/1.1", |req| {
      assert_eq!(req.method, Method::GET);
    });
  }

  #[bench]
  fn bench_parse_method_speed(b: &mut Bencher) {
    b.iter(|| { 
      let f = |req: Request| { };
      test_req("GET / HTTP/1.1", f);
    })
  }
}
