use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, Result};
use std::sync::TaskPool;
use std::thread::Thread;

extern crate gossamer;
extern crate log;
use gossamer::headers::Headers;
use gossamer::cabinet::Cabinet;
use gossamer::request::{Request};
use gossamer::response::Response;

fn main() {

  let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

  fn handle_request<'a>(request: &'a Request, stream: &mut TcpStream) -> Result<()> {
    Response {request: request}.dump(stream)
  }

  // TODO Not sure what this should return.
  fn handle_stream(stream: TcpStream) -> () {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = [0u8; 4096];
  
    let request = Request::make(&mut reader, &mut buffer).unwrap();
    handle_request(&request, &mut stream.try_clone().unwrap());
  }


  // TODO: Make this configurable
  let pool = TaskPool::new(6);
  
  for stream in listener.incoming() {
    match stream {
      Err(e) => { println!("ERROR!") },
        Ok(stream) => {
          pool.execute(move|| {
              handle_stream(stream);
              });
        }
    }
  }
  drop(listener);
}
