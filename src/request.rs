use super::headers::Headers;
pub struct Request<'a> {
  headers: Headers<'a>
}

trait RequestIterator : Iterator {
  type Item = Request;

}
