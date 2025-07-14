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

### Reading the request

### A closer look at an HTTP request

### Writing a response

### Returning a real HTML

### Validating the request and selectively responding

### Refactoring