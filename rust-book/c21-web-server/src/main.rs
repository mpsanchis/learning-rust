use std::{
  fs,
  io::{BufReader, prelude::*}, // to get access to traits and types that let us read from and write to the stream
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);
  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let response: String;
  if http_request.first().unwrap() == "GET / HTTP/1.1" {
    response = [
      "HTTP/1.1 200 OK",
      &fs::read_to_string("src/static/hello.html").unwrap(),
    ]
    .join("\r\n\r\n");

    println!("Ok request");
  } else {
    response = [
      "HTTP/1.1 404 Not Found",
      &fs::read_to_string("src/static/404.html").unwrap(),
    ]
    .join("\r\n\r\n");

    println!(
      "Bad request to {}",
      http_request.first().unwrap().split(" ").nth(1).unwrap()
    );
  }

  stream.write_all(response.as_bytes()).unwrap();
}
