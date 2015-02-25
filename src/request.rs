use super::headers::Headers;

pub struct Request<'a> {
  headers: Headers<'a>
}


impl <'a> Request<'a> {

  pub fn make<'a>(reader: &mut Reader, buffer: &'a mut [u8]) -> Option<Request<'a>> {
    let result = reader.read(buffer);
    match result {
      Ok(n) => {
        unsafe {
          let string = String::from_utf8_lossy(buffer);
          debug!("{}", string)
        }
  
      },
      Err(e) => panic!("Couldn't read from the TCP buffer{}", e)
    }

    Some(Request { headers: Headers::empty() } )
  }


}

