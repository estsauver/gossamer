#[feature(old_io)]
use std::io::{Write, Result};
use super::request::Request;

pub struct Response<'a> {
  pub request: &'a Request<'a>
}

impl <'a> Response<'a> {
  pub fn dump(&self, writer: &mut Write) -> Result<()> {
    writer.write_all(&self.as_string().as_slice().as_bytes())
  }

  fn as_string(&self) -> String {
    format!("{:?}", self.request.method.clone())
  }
}
