use std::old_io::{TcpStream, TcpListener, IoResult};
use std::old_io::{Acceptor, Listener, BufferedReader};
use std::sync::TaskPool;
use std::thread::Thread;

extern crate gossamer;
extern crate log;
use gossamer::headers::Headers;
use gossamer::cabinet::Cabinet;
use gossamer::request::{Request};
use gossamer::response::Response;



fn main() {

  let listener = TcpListener::bind("127.0.0.1:3000");
  let mut acceptor = listener.listen();

  fn handle_request<'a>(request: &'a Request) -> Response<'a> {
    Response {request: request}
  }

  // TODO Not sure what this should return.
  fn handle_stream(stream: TcpStream) -> () {
    let mut reader = BufferedReader::new(stream.clone());
    let mut buffer = [0u8; 4096];
  
    let request = Request::make(&mut reader, &mut buffer).unwrap();
    let response = handle_request(&request);
    response.dump(&mut stream.clone());
  }


  // TODO: Make this configurable
  let pool = TaskPool::new(6);
  
  for stream in acceptor.incoming() {
    match stream {
      Err(e) => { println!("ERROR!") },
        Ok(stream) => {
          pool.execute(move|| {
              handle_stream(stream);
              });
        }
    }
  }
  drop(acceptor);
}
