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

A thread pool has `N` threads waiting for jobs, and a queue of tasks to serve. Each thread from the pool pulls tasks from the queue as they are free.

Other options (not explored in this exercise) would be:
- Fork/join model
- Single-threaded async I/O model
- Multi-threaded async I/O model

Before starting, we will explore the technique we're not going to use as a starting point.

### Spawning a thread for each request

Not ideal if many requests can arrive, but a good starting point. See code.
Note: a good alternative to this would be async/await. Not implemented here.

### Creating a finite number of threads

We define an interface of what we would like to have (but don't- yet):
```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
let pool = ThreadPool::new(4);

for stream in listener.incoming() {
  let stream = stream.unwrap();

  pool.execute(|| {
    handle_connection(stream);
  });
}
```

#### Building a threadpool using compiler-driven development

Idea behind: since the compiler is quite strict, we can define interfaces (structs, impls, etc.) and run `cargo check` every now and then to verify that the code is respecting the contracts.
Then, we can iterate by implementing functionalities one by one.

#### Validating the number of threads in `new`

We can use `assert!` to validate inputs. However, it panics at runtime if assertion fails. Adding doc comments helps the user of a function/struct to understand what happens when executing.

#### Creating space to store the threads

By looking at the `thread::spawn` signature, we see that the return type is:
```rust
JoinHandle<T>
// ...
where
    F: FnOnce() -> T
```
Basically, `JoinHandle<T>`, where `T` is the return type of the closure passed to the thread.

If our threads don't return (<=> they return `()`), we can store them in a `Vec<JoinHandle<()>>`.

#### A `Worker` Struct responsible for sending code from the `ThreadPool` to a thread

Rust's stdlib doesn't include any way to create a thread that *waits* for code. Only `thread::spawn` is there, which will create a thread and start it immediately.

Therefore, we need additional auxiliary code to store such "waiting threads", which we'll name `Worker`s.

#### Sending requests to threads via channels

We want the `Worker` structs to fetch the code to run from a queue held in the `ThreadPool` and send that code to its thread.
A simple way to communicate between threads is using the channels from Chapter 16. The idea is to:
1. Create a channel in the `ThreadPool` and hold on to the sender
2. Each `Worker` will hold on to the receiver
3. A `Job` struct will hold the closures we want to send down the channel
4. The `execute` method will send the job through the sender
5. In its thread, the `Worker` will loop over its receiver, and execute the closures of any jobs it receives

Notes:
* for this we need to share the receiver among many threads, so we need a `Mutex` to deal with concurrency
* because we share ownership of the receiver among several threads, we also need an `Arc` (to count references and drop the receiver if references disappear (which doesn't happen in this case, but the compiler needs this information))

#### Implementing the `execute` method

We use a `Box` to put the closure to the heap, and send it as a job down the channel.

The `Worker` then loops infinitely, checking the receiver and attempting to receive a Job from it.

Note (subtle): the following implementation doesn't work as expected
```rust
while let Ok(job) = receiver.lock().unwrap().recv() {
  println!("Worker {id} got a job; executing.");

  job();
}
```

This is because `while let` does not drop temporary values until the end of the associated block. Our solution, instead:
```rust
loop {
  let job = receiver.lock().unwrap().recv().unwrap();
  job();
}
```
drops the lock (and frees it) before running the job.

// TODO: have an async version of the same

## Graceful shutdown and cleanup

So far, when we `Ctrl+C` to halt the main thread, all other threads are stopped as well (because they belong to the same process).

As we implement the following features, note that none of them affect the parts of the code that handle executing closures. Therefore, everything here would be the same if we were using a thread pool for an async runtime.

### Implementing the `Drop` trait on `ThreadPool`

Step 1: Call `join()` on every thread.

At this step we don't have a compiling project, because of:
```
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
    --> src/lib.rs:73:7
     |
73   |       worker.thread.join().unwrap();
     |       ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
     |       |
     |       move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
     |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
```
Basically, calling `join` takes ownership of the thread: it can only be used once (it consumes `self`). This is intentional: `join` waits for the thread to be finished, so it can only be called once. However, in our code, each `thread` belongs to each `Worker` struct in the `ThreadPool.workers` vector, which is owned by the `ThreadPool` itself.

Step 2: Fix the ownership issue by emptying the workers

If `join` takes ownership of the thread, then the thread is unusable, and so is the `Worker` that owns it. Therefore, a way to solve the ownership issue is to remove the `Worker` from the pool when finishing (joining) the thread. The book's alternative to the code found here was to `drain` the vector, which does effectively the same thing:
```rust
for worker in self.workers.drain(..)
```

### Signaling to the threads to stop listening for jobs

The code at this point compiles, but an infinite `loop` in the threads prevents them from finishing (joining). Therefore, when the `ThreadPool` is dropped, it waits infinitely for the first thread to finish.

Some actions are needed to solve this issue:
1. Wrapping the `ThreadPool.sender` in an `Option`, to be able to `take` it when dropping the `ThreadPool`, and dropping it.
2. When calling `ThreadPool::execute`, we need to use `as_ref().unwrap()` on the `sender`, because now it will be wrapped in an `Option`
3. Dropping the channel with `drop(self.sender.take())`
4. Once the channel is dropped, the thread will receive an `Err(RecvError)` when calling `recv()`, instead of an `Ok(msg)`. We can leverage this in the thread to `break` the `loop`.

We can test the correct behaviour if we iterate through `listener.incoming.take(N)` in `main`: after *N* requests, the server will shut down.