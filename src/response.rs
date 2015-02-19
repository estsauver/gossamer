use std::old_io::{Writer, IoResult};

use super::request::Request;
pub struct Response<'a> {
  pub request: &'a Request<'a>
}

impl <'a> Response<'a> {
  pub fn dump(&self, writer: &mut Writer) -> IoResult<()> {
    writer.write_all(self.as_bytes())
  }

  fn as_bytes(&self) -> &[u8] {
"HTTP/1.1 200 OK\r\n
date: Mon, 23 May 2005 22:38:34 GMT\r\n
Server: Gossamer\r\n
Last-Modified: Wed, 08 Jan 2003 23:11:55 GMT\r\n
Content-Type: text/html; charset=UTF-8\r\n
Connection: close".as_bytes()
  }
}
