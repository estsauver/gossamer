use super::request::Request;
use super::response::Response;

pub trait Cabinet {
  fn initialize<'a>(app: &'a Cabinet) -> &'a Cabinet;
  fn call(request_env: Request) -> Response;
}
