# Building a web server

Goals:
1. Learn a bit about TCP and HTTP.
2. Listen for TCP connections on a socket.
3. Parse a small number of HTTP requests.
4. Create a proper HTTP response.
5. Improve the throughput of our server with a thread pool.

Note:
- async/await won't be used, as it would be too ambitious to build an async runtime, and too much abstracted to use Tokio

## Building a single-threaded web server

### Listening to the TCP connection

In networking, connecting to a port to listen to is known as "binding to a port", so the `std::net` lib has the following functionality:
```rust
let listener = TcpListener::bind("128.0.0.1:7878").unwrap();
```
which will return:
- an error, in some cases such as binding to port `80` without admin rights, or binding to a port already in use
- a `TcpListener` it manages to connect

The TCP listener returns an iterator that gives us a sequence of streams. Each one represents an open connection (more specifically, a connection *attempt*) to the server.

The program until now will log "Connection established!" a couple of times, since the browser will make many requests (such as for favicon and main HTML, and retries if our tcp server doesn't reply).

### Reading the request

```rust
fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);
  let http_request: Vec<_> = buf_reader
      .lines()
      .map(|result| result.unwrap())
      .take_while(|line| !line.is_empty())
      .collect();

  println!("Request: {http_request:#?}");
}
```

We leverage `BufReader` from `std::io`, whose constructor accepts anything that implements the `std::io::Read` trait. It has the `lines()` method, which provides an iterator over the lines of its content (the stream). The lines could be an error if the data wasn't valid UTF-8, or there was a problem when reading from the stream, but this code `unwrap`s for simplicity.

The `take_while` method of `Iterator`s is similar to the `filter` method, but stops when the closure returns `false`. We use it to stop reading lines when the browser sends `\r\n\r\n` to signal end of message (as defined in the HTTP protocol).

The output we get is an array of requests similar to:
```
[
  "GET / HTTP/1.1",
  "Host: 127.0.0.1:7878",
  "Sec-Fetch-Site: none",
  "Connection: keep-alive",
  "Upgrade-Insecure-Requests: 1",
  "Sec-Fetch-Mode: navigate",
  "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
  "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15",
  "Accept-Language: en-GB,en;q=0.9",
  "Sec-Fetch-Dest: document",
  "Accept-Encoding: gzip, deflate",
]
```

### A closer look at an HTTP request

An HTTP request has this format:
```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

where:
- `Method` is the HTTP method used, such as `GET` or `POST`
- `Request-URI` is a resource identifier
- `HTTP-Version` is something like `HTTP/1.1`
- `headers` are key-value pairs, like `"Accept: text/html"`
- `message-body` is the payload (not used in `GET` calls)

### Writing a response

HTTP responses have the following format:
```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The simplest response is: `HTTP/1.1 200 OK\r\n\r\n`.

### Returning a real HTML

We can return HTML in the `message-body`. A browser will correctly display it if it matches the HTTP response template.

### Validating the request and selectively responding

So far, our server was always replying with the same content, independently of the request.

We can match the `Request-URI` and return different html content based on the request.

### Refactoring

See code.

## Turning our single-threaded server into a multithreaded server

### Simulating a slow request in the current server implementation

Code now shows how a single-threaded server can "block" fast requests if they arrive after "slow" ones. Try to:
1. Load `localhost:7878/sleep`, and immediately after
2. Load `localhost:7878/`

Request 2 has to wait for 1 to finish, even if it's a very fast request.

### Improving throughput with a thread pool

### Spawning a thread for each request

### Creating a finite number of threads

### Building a threadpool using compiler-driven development

### Validating the number of threads in `new`

### Creating space to store the threads

### A `Worker` Struct responsible for sending code from the `ThreadPool` to a thread

### Sending requests to threads via channels

### Implementing the `execute` method

