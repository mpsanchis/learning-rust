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

### A closer look at an HTTP request

### Writing a response

### Returning a real HTML

### Validating the request and selectively responding

### Refactoring