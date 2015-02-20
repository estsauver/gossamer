use super::headers::Headers;
pub struct Request<'a> {
  headers: Headers<'a>
}


impl <'a> Request<'a> {

  pub fn make<'a>(reader: &mut Reader, buffer: &'a mut [u8]) -> Option<Request<'a>> {
    Some(Request { headers: Headers::empty() } )
  }


}

