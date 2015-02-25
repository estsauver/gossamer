use super::request::Request;
use super::response::Response;

pub trait Cabinet {
  fn initialize<'a>(app: &'a Self) -> &'a Self;
  fn call(request_env: Request) -> Response;
}
