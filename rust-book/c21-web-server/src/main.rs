use c21_web_server::ThreadPool;
use std::{
  fs,
  io::{BufReader, prelude::*}, // to get access to traits and types that let us read from and write to the stream
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);
  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

  let (status_line, filename) = match &http_request.first().unwrap()[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("HTTP/1.1 200 OK", "hello.html")
    }
    _ => ("HTTP/1.1 404 Not Found", "404.html"),
  };

  let content = fs::read_to_string(format!("static/{filename}")).unwrap();
  let content_length = content.len();
  let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

  stream.write_all(response.as_bytes()).unwrap();
}
