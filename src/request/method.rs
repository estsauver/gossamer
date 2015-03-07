use super::Request;
use regex::Regex;
/// The Http method of the request
#[derive(PartialEq, Debug, Clone)]
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
  /// This method should maybe be refactored to take a list of methods and map across it?
  pub fn make<'a>(input: &'a str) -> Option<(Method, &'a str)> {
    let methods = vec![
      (regex!(r"^[Gg][Ee][Tt] "), Method::GET),
      (regex!(r"^[Pp][Oo][Ss][Tt] "), Method::POST),
      (regex!(r"^[Pp][Uu][Tt] "), Method::PUT),
      (regex!(r"^[Hh][Ee][Aa][Dd] "), Method::HEAD),
      (regex!(r"^[Dd][Ee][Ll][Ee][Tt][Ee] "), Method::DELETE),
      (regex!(r"^[Cc][Oo][Nn][Nn][Ee][Cc][Tt] "), Method::CONNECT),
      (regex!(r"^[Oo][Pp][Tt][Ii][Oo][Nn][Ss] "), Method::OPTIONS),
      (regex!(r"^[Tt][Rr][Aa][Cc][Ee] "), Method::TRACE)
    ];

    methods.into_iter()
      .map(|(regex, method)| {
        Method::match_method(input, regex, method)
      })
      .filter(|x: &Option<(Method, &str)>| { x.is_some() })
      .map(|x| { x.unwrap() })
      .next()
  }

  fn match_method<'a>(input: &'a str, regex: Regex, method: Method) -> Option<(Method, &'a str)> {
    match regex.find(input) {
      Some((_,y)) => Some((method, &input[y ..])),
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

  fn test_method<F: Fn(Method) -> ()>(input:&str, assertions: F) {
    let method = Method::make(input).unwrap().0;
    assertions(method)
  }

  #[test]
  fn it_parses_an_http_get() {
    test_method("GET / HTTP/1.1", |method| {
      assert_eq!(method, Method::GET);
    });
  }

  #[test]
  fn it_parses_an_http_head() {
    test_method("HEAD / HTTP/1.1", |method| {
      assert_eq!(method, Method::HEAD);
    });
  }

  #[test]
  fn it_parses_an_http_post() {
    test_method("POST / HTTP/1.1", |method| {
      assert_eq!(method, Method::POST);
    });
  }

  #[test]
  fn it_parses_an_http_put() {
    test_method("PUT / HTTP/1.1", |method| {
      assert_eq!(method, Method::PUT);
    });
  }

  #[test]
  fn it_parses_an_http_delete() {
    test_method("DELETE / HTTP/1.1", |method| {
      assert_eq!(method, Method::DELETE);
    });
  }

  #[test]
  fn it_parses_an_http_connect() {
    test_method("CONNECT / HTTP/1.1", |method| {
      assert_eq!(method, Method::CONNECT);
    });
  }

  #[test]
  fn it_parses_an_http_options() {
    test_method("OPTIONS / HTTP/1.1", |method| {
      assert_eq!(method, Method::OPTIONS);
    });
  }

  #[test]
  fn it_parses_an_http_trace() {
    test_method("TRACE / HTTP/1.1", |method| {
      assert_eq!(method, Method::TRACE);
    });
  }

  #[test]
  fn it_handles_case_insensitive_versions() {
    test_method("gEt / HTTP/1.1", |method| {
      assert_eq!(method, Method::GET);
    });
  }

  #[bench]
  fn bench_parse_method_speed(b: &mut Bencher) {
    b.iter(|| {
      Method::make("GET / HTTP/1.1").unwrap().0
    })
  }
}
