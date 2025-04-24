# Fundamentals of asynchronous programming

An alternative approach to threads is async, using rust's:
* `Future`s
* `Stream`s
* `async` / `await`

and the tools for managing and coordinating between asynchronous operations.

The ideas behind this alternative are:
* Avoiding the overhead created by spawning many threads
* Giving up the CPU voluntarily in our code to let others run, when we understand that an operation is slow
* Having a simpler syntax than thread, that allows us to write "direct code", and know what tasks come before others

Note: when working with async in Rust, we're always dealing with concurrency (not parallelism). However, depending on the hardware, the OS, and the async runtime we are using, that concurrency might also use parallelism under the hood.

## Future and the async syntax

The `Future` trait is provided by rust to represent values that may not be ready now but will become ready at some point in the future. Therefore, in Rust futures are simply types that implement the `Future` trait. Each future holds its own information about the progress that has been made and what "ready" means.

You can apply the `async` keyword to blocks and functions, to specify that they can be interrupted and resumed. Within an async block or function, you can use the `await` keyword to *await* a future (that is, wait for it to become ready). Any point where you await a future within an async block or function is a potential spot for that async block or function to pause and resume. Thre process of checking with a future to see if its value is available yet is called *polling*.

When writing async rust, the compiler uses the `async` and `await` keywords to transform the code. The compiler is doing this kind of operations all the time, for instance when compiling `for` loops into equivalent code using the `Iterator` trait.

### Our first async program

Our first async program uses a runtime to execute an async function:
```rust
async fn page_title(url: &str) -> Option<String> {
  // code that uses `await` and returns Option<String>
}
```

When Rust sees a function marked with async, it compiles it into a non-async function whose body is an async block:
```rust
fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
  async move {
    // code that uses `await` and returns Option<String>
  }
}
```

Thus, writing `async fn` is equivalent to writing a function that returns a future of the return type. To the compiler, both functions above are equivalent.

Also, when Rust sees a block marked with the `async` keyword, it compiles it into a unique, anonymous data type that implements the `Future` trait.

Notes:
* The new function body is an `async move` block because of how it uses the `url` parameter. (We'll talk much more about `async` versus `async move` later in the chapter).
* The new version of the function has a kind of lifetime we haven't seen before in the output type: `'_`. Because the function returns a future that refers to a reference— in this case, the reference from the `url` parameter— we need to tell Rust that we want that reference to be included. We don't have to name the lifetime here, because Rust is smart enough to know there's only one reference that could be involved, but we do have to be explicit that the resulting future is bound by that lifetime.

In order to be able to run the function, we need a *runtime*. The `main` function cannot be declared as `async`, because that would mean:
* it returns a `Future`
* it might have `await` points, where execution is handed back to a runtime
* it keeps an internal state with checkpoints in the `await` steps, so that when given control back, it knows how to continue

In the `page_title` function, we could have the code:
```rust
let response = trpl::get(url).await;
let response_text = response.text().await;
return Html::parse(&response_text)
  .select_first("title")
  .map(|title_element| title_element.inner_html());
```

this would generate a state machine with states that could be represented in this enum:
```rust
enum PageTitleFuture<'a> {
  Initial { url: &'a str },
  GetAwaitPoint { url: &'a str },
  TextAwaitPoint { response: trpl::Response },
}
```

Managing such a state machine, which could have more states as `await` calls increase, is tedious and error-prone. That is one of the reasons why async rust exists.

In short, we need a runtime that can manage the async function and poll it until it is done. A simple code with one runtime and one async function looks like:
```rust
async fn page_title(url: &str) -> Option<String> { ... }

fn main() {
  trpl::run(async {
    let url = "https://doc.rust-lang.org";
    match page_title(url).await {
      Some(title) => println!("The title for {url} was {title}"),
      None => println!("{url} had no title"),
    }
  });
}
```

In this example, we are using `trpl::run`, which is using `Tokio` behind the scenes. The rust *stdlib* comes with no async runtime, so we have to choose a 3rd party crate, or develop one ourselves.

## Applying concurrency with async

### Creating a new task with *spawn_task*

Async crates usually provide functions to create separate tasks, that mimic the thread behaviour. In the crate used by the book, we have `trpl::spawn_task`, that allows us to work with async very similarly to multi-threading:
```rust
trpl::run(async {
  let handle = trpl::spawn_task(async {
    for i in 1..10 {
      println!("hi number {i} from the first task!");
      trpl::sleep(Duration::from_millis(500)).await;
    }
  });
  for i in 1..5 {
    println!("hi number {i} from the second task!");
    trpl::sleep(Duration::from_millis(500)).await;
  }
  // wait for the task to finish, before finishing the program
  handle.await.unwrap();
});
```

The difference with threads, in this example, is that we didn't have to create a thread for the "main" program (the loop for the "second task" and last await).

In this example, we don't even need to spawn a task. Because async blocks compile to anonymous futures, we can put each "task" in its own async block, and then await them both with the `trpl::join` function:
```rust
let fut1 = async {
  for i in 1..10 {
    println!("hi number {i} from the first task!");
    trpl::sleep(Duration::from_millis(500)).await;
  }
};
let fut2 = async {
  for i in 1..5 {
    println!("hi number {i} from the second task!");
    trpl::sleep(Duration::from_millis(500)).await;
  }
};
trpl::join(fut1, fut2).await;
```

Note that the crate implementing the `join` function decides when to poll each future. In the example, the `join` function is *fair* (i.e., it polls each future equally), so the outputs are alternated (unlike the example with threads, where the OS decided when to run each thread). It is the async runtime, therefore, deciding when tasks are running, instead of delegating that to the OS. The only caveat is that the runtime itself might leverage threads under the hood, so that can have an effect.

### Counting up on two tasks using message passing

Async crates can provide channels, similar to the *multiple-producer single-consumer* one we saw in chapter 16, to send information among tasks. In the example used in the book, we can do:
```rust
let (tx, mut rx) = trpl::channel();

tx.send(String::from("hi")).unwrap();
let received = rx.recv().await.unwrap();
println!("Got: {received}");
```

Note that `std::mpsc::channel` provided a sync method (`Receiver::recv`), which blocks until it receives a message. In this example, instead, the method `trpl::Receiver::recv` hands control back to the runtime when `await`ed.

An example closer to a real-life scenario, with two futures using a channel, is the following:
```rust
let (tx, mut rx) = trpl::channel();

let tx_fut = async move {
  let vals = vec![
    String::from("hi"),
    String::from("from"),
    String::from("the"),
    String::from("future"),
  ];
  for val in vals {
    tx.send(val).unwrap();
    trpl::sleep(Duration::from_millis(500)).await;
  }
};

let rx_fut = async {
  while let Some(value) = rx.recv().await {
    println!("received '{value}'");
  }
};

trpl::join(tx_fut, rx_fut).await;
```

Note how the `async move` syntax moves the `tx`, so that it's dropped at the end (channel closed, so `rx` receives `None` and program ends). This is equivalent to adding the `move` keyword before the parameters of a closure.

## Working with any number of futures

Following the example from the previous sub-chapter: if we wanted to send messages from two transmitters, we could have a `tx2_fut` that uses `tx1 = tx.clone()`. This would mean that we have 3 futures (`tx_fut`, `tx1_fut` and `rx_fut`), and we would need another join function, such as `join3`. This, however, doesn't scale.

The first alternative is the macro form of `join`:
```rust
trpl::join!(tx1_fut, tx_fut, rx_fut);
```

This is an improvement, but comes with a main drawback: we need to know the number of futures ahead of time. In many scenarios, we would have a "collection" of futures, and we would like to `await` them all. For that, we have a `join_all` function, but this DOES NOT compile yet:
```rust
let futures = vec![tx1_fut, rx_fut, tx_fut];
trpl::join_all(futures).await;
```

and throws the error:
```
let futures = vec![tx1_fut, rx_fut, tx_fut];
                            ^^^^^^ expected `async` block, found a different `async` block
[...]
note: no two async blocks, even if identical, have the same type
help: consider pinning your async block and casting it to a trait object
```

The issue is that the async blocks are compiled to produce a *unique* and *anonymous* data type (enum) that implements `Future<Output = ()>` (empty return because our blocks don't return anything in the expression). Therefore, the vector cannot be created because it would contain different elements.

To make this work, we need to use *trait objects*, which will be covered in chapter 18. Basically, trait objects allow us to treat each of the anonymous futures as the same type, because all of them implement the `Future` trait. A way to solve this compile-time issue is to wrap the futures (`F`) in a `Pin<Box<F>>`, and declaring them as dynamic size when constructing the vector:
```rust
let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
```

this does compile, and we can then call:
```rust
trpl::join_all(futures).await;
```

⚠️ Note: there are two distinct scenarios that are usually accepted when working with many futures
1. EITHER we have a fixed number of futures with different outputs, and we use something such as `join!`:
```rust
let a = async { 1u32 };
let b = async { "Hello!" };
let c = async { true };

let (a_result, b_result, c_result) = trpl::join!(a, b, c);
println!("{a_result}, {b_result}, {c_result}");
```

2. OR we have an arbitrary number of futures in a collection, but they have to be of the same type in order to `join_all` them, as we just did.

### Racing futures

Sometimes, waiting for the first future to finish is enough, and we don't have to wait for all of them like `join` does. In those cases, runtimes provide functions such as `race`, the one used in *Our first async program*:
```rust
let slow = async { /* ... */ };
let fast = async { /* ... */ };
trpl::race(slow, fast).await;
```

Some notes:
- Depending on the implementation of `race`, the Future that starts might be random (fair implementation), or based on argument position (unfair).
- Each task runs synchronously up until an `await` is hit, where control is given to the runtime, which can decide to run something else.
- A consequence of the previous point is that long-running tasks without `async` breakpoints can starve other tasks.

### Yielding control to the runtime

If a future has many long-running tasks that are not async, we can force it to yield control back to the runtime. In the example crate used, the `yield_now` function does that. An example code would be:
```rust
let a = async {
  println!("a started...");
  slow_computation();
  trpl::yield_now().await;
  another_slow_computation();
  trpl::yield_now().await;
  println!("a finished...");
}
```

This is a form of *cooperative multitasking*, where each future has the power to determine when it hands over control via await points. In some rust-based embedded operating systems, this is the only kind of multitasking.

### Building our own async abstractions

We can compose futures together to create new patterns. For example, we can build a `timeout` function with the async building blocks we already have. The result is another building block that could be use to create still more async abstractions. We would expect the `timeout` function to work as the following code:

```rust
let slow = async {
  trpl::sleep(Duration::from_millis(1000)).await;
  "Slow finished"
};

match timeout(slow, Duration::from_millis(10).await).await {
  Ok(msg) => println!("'Slow' lasted less than 10ms, and finished with message: {msg}"),
  Err(duration) => println!("'Slow' task failed after {}s", duration.as_secs())
}
```

In order to implement such a function, let's first discuss some API details:
- It needs to be an async function so that we can await it
- Its first parameter should be the Future to run. Making it generic allows to pass any Future.
- The second parameter must be the time to wait before exiting.
- It should return a `Result`, because there are two scenarios: either the Future finishes, or the timeout kills it before it's done

A simple implementation uses the building blocks from previous exercises:
```rust
async fn timeout<F: Future>(
  future: F,
  timeout: Duration
) -> Result<F::Output, Duration> {
  match trpl::race(future, trpl::sleep(timeout)).await {
    Either::Left(output) => Ok(output),
    Either::Right(_) => Err(timeout)
  }
}
```

We can then use it in an async block as:
```rust
let slow = async {
  trpl::sleep(Duration::from_secs(5)).await;
  "Finally finished"
};

match timeout(slow, Duration::from_secs(2)).await {
  Ok(message) => println!("Succeeded with '{message}'"),
  Err(duration) => {
    println!("Failed after {} seconds", duration.as_secs())
  }
}
```

## Streams: Futures in sequence

A stream is a pattern where a future generates a sequence of items over time.

In Chapter 13, the `Iterator` trait was presented, which deals with sequences of items. In this chapter, we used a channel to pass messages between futures. The main two differences between an `Iterator` and the combo `trpl::channel` + `trpl::Receiver` are:
1. Iterators are synchronous, while the channel receiver is asynchronous
2. When using an Iterator, we call its `next` method, whereas Receiver exposes an async `recv` method

Apart from that, the APIs of both have a similar feel. In fact, we can even build streams from iterators:
```rust
use trpl::StreamExt;

let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let iter = values.iter().map(|n| n * 2);
let mut stream = trpl::stream_from_iter(iter);

while let Some(value) = stream.next().await {
  println!("The value was: {value}");
}
```

Note how we must include the `StreamExt` trait into scope. This will include the following:
```rust
// 1. Declares a new trait StreamExt
// Any type that implements StreamExt must also implement Stream
pub trait StreamExt: Stream {
    // some methods of the trait
}

// 2. "Blanket" implementation of StreamExt
// Implements StreamExt for all types St that implement Stream, and allows St to be a possibly unsized type (like a trait object 'dyn Stream')
impl<St: ?Sized> StreamExt for St where St: Stream {}
```

This syntax defines
1. any type that implements `StreamExt` must also implement `Stream`
2. implement `StreamExt` for all types that already implement `Stream`


### Composing streams

Many concepts are naturally represented as streams. Some examples include:
- items becoming available in a queue
- chunks of data comping from the filesystem when a full dataset is too large
- data arriving over the network

Because streams are futures, we can use them with any other kind of future and combine them. For example, we can:
- batch events to avoid triggering too many network calls
- set timeouts on sequences of long-running operations
- throttle user interface events to avoid doing needless work (i.e.: only respond to UI events such as mouse movements every X milliseconds)

A way to create streams is through creating a channel (`let (tx, rx) = trpl::channel()`), and a task that writes to it (using `tx`), and then generating the stream from the `rx`. An example of it is:
```Rust
fn get_messages() -> impl Stream<Item = String> {
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for (index, message) in messages.into_iter().enumerate() {
      let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
      trpl::sleep(Duration::from_millis(time_to_sleep)).await;

      tx.send(format!("Message: '{message}'")).unwrap();
    }
  });

  ReceiverStream::new(rx)
}
```

And then this stream can be used by iterating it after pinning it (TODO: why pinning?):
```Rust
pub fn read_msgs_from_stream_with_timeout() {
  trpl::run(async {
    let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    while let Some(result) = messages.next().await {
      match result {
        Ok(message) => println!("{message}"),
        Err(reason) => eprintln!("Problem: {reason:?}"),
      }
    }
  })
}
```

Notes about the library we use (`trpl`, which uses `tokio`):
- Calling `spawn_task` works because we already set up our runtime. Had we not, it would cause a panic. Other implementations choose different tradeoffs (e.g., they might spawn a new runtime to avoid the panic but leading to more overhead, or they might not provide a way to spawn tasks without a reference to the runtime).
- This code outputs `Elapsed(())` when a message is not ready (times out), but the message is not lost. Instead, the channel (transformed by `.timeout()`) keeps all the messages, and makes the next task progress until the timeout is over, but does not discard the tasks. Other channels and other streams provide different functionalities.

### Merging streams

We can merge streams, as long as they are of the same type. For instance, we could merge the stream from before, of type:
```rust
Timeout<impl Stream<Item = String>>
```

with another stream returned by `get_intervals()` which is originally of type:
```rust
impl Stream<Item = u32>
```

as long as we transform them:
```Rust
let messages = get_messages().timeout(Duration::from_millis(200));
let intervals = get_intervals()
    .map(|count| format!("Interval: {count}"))
    .timeout(Duration::from_secs(10));
let merged = messages.merge(intervals);
let mut stream = pin!(merged);
```

In the book example, we also modify the stream by calling `.throttle(Duration::from_millis(100)` on it. This transforms the stream in another one that only gets polled every 100ms. This is done in a *lazy* way: Rust never calls the underlying stream every 1ms, even if it's capable of generating data at that rate.

## Async traits in detail

### The Future trait

The `Future` trait is defined as follows:
```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
  type Output;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

and relies on the `Poll` type, which is similar to `Option`:
```Rust
enum Poll<T> {
  Ready(T),
  Pending,
}
```

This trait makes trust compile our `await` blocks (running in a runtime) such as this:
```Rust
match async_fn(arg).await {
  Some(res) => println!("async fn returned: {res}"),
  None => println!("async fn returned None")
}
```

into something like this:
```Rust
let mut async_fn_fut = async_fn(arg);
loop {
  match async_fn_fut.poll() {
    Ready(value) => match value {
      Some(res) => println!("async fn returned: {res}"),
      None => println!("async fn returned None")
    }
    Pending => {
      // return control back to runtime
    }
  }
}
```

The key mechanism of how a runtime polls the futures is beyond the scope of this book.

### The Pin and Unpin traits

We had to use pinning when we encountered the error:
```
trpl::join_all(futures).await;
                        ^^^^^ the trait `Unpin` is not implemented for `{async block@src/main.rs:10:23: 10:33}`, which is required by `Box<{async block@src/main.rs:10:23: 10:33}>: Future`

pub struct JoinAll<F>
           ------- required by a bound in this struct

where
    F: Future,
       ^^^^^^ required by this bound in `JoinAll`
```

Which means:
* The `trpl::join_all` function returns a struct called `JoinAll<F>`, where `F: Future`
* Directly awating a Future with `await` pins the future implicitly

However, we're not directly awaiting a future here. We construct a new future by passing a collection of futures to a `join_all` function. The signature of `join_all` requires that the types of the items in the collection all implement the `Future` trait, and `Box<T>` implements `Future` only if `T` is a future that implements the `Unpin` trait.

Let's focus on the `poll` function that the `Future` trait defines, and the type of `self`:
```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
```

This type annotation for `self` means that `self` must be of type `Pin<&mut Self>` in order to implement the `Future` trait correctly. In particular, the type is restricted to either of:
* the type on which the method is implemented
* a reference or smart pointer to that type, or
* a `Pin` wrapping a reference to that type

Chapter 18 explains more about this syntax. For now, it's enough to know that in order to poll a future, we need a `Pin`-wrapped mutable reference to the type.

`Pin` is a wrapper for pointer-like types such as:
- `&`
- `&mut`
- `Box`, or
- `Rc`

Technically, `Pin` works with types that implement the `Deref` or `DerefMut` traits, but this is effectively equivalent to working only with pointers. `Pin` **is not a pointer** itself, and doesn't have any behaviour of its own (like `Rc` or `Arc` do with ref counting); it's purely a tool the compiler uses to enforce constraints on pointer usage.

Remember that the list of `await` points in a future get compiled into a **state machine**, and the Rust compiler makes sure that state machine follows all of Rust's normal rules around safety, including borrowing and ownership. To make that work, Rust looks at what data is needed between `await` points (or the end of the future), and creates a corresponding variant in the compiled state machine. Each variant gets the access it needs to that data, either by:
- taking ownership of that data, or
- getting a mutable or immutable reference to it

Given this, when we want to move around the future that corresponds to that block, things get complicated. Some ways of moving the future include:
- Pushing it into a data structure (such as `Vec`), to- for example- use as an iterator with `join_all`
- Returning it from a function

These movements imply moving the state machine Rust creates for us. Unlike most other types, the futures Rust creates for async blocks can end up with references to themselves. By default, any object that has a reference to itself is unsafe to move, because references always point to the actual memory address of whatever they refer to. If you move the data structure, those internal references will be left pointing at the old location, that is now invalid (freed). Instead of ensuring that the move updates all references correctly (which implies compilation overhead), the approach in Rust is to make sure the data structure *doesn't move in memory*.

When we *pin* a value by wrapping a pointer to that value in `Pin`, it cannot longer move. Thus, a `Pin<Box<SomeType>>` is pinning `SomeType`, not the `Box` pointer. This means we can use another `Box` *b2* to point at the `SomeType` (implementing `Future`, in our example).

We might want to leverage pinning for types with internal references. Most types, however, are safe to move around, even if they are behind a `Pin`. For those cases, the `Unpin` marker trait comes handy: it informs the compiler that even if pinned, the type can be moved around in memory without problem. Just as it does with `Send` and `Sync`, the compiler implements `Unpin` for all types where it can prove it's safe. The special (and more rare) cases are when some type has `impl !Unpin for SomeType` (as it happens with futures).

A rust code that shows how the absence of `Unpin` prevents us from modifying the contents of the memory pointed at by the pointer or smart pointer being pinned can be found in function: `unpin_example_no_compile`. Note how `Pin<Box<T>>` does not implement the `DerefMut` trait, and therefore we are not allowed to do `*pinned = new_value`.

Note: chapters 2 and 4 of [async programming in rust](https://rust-lang.github.io/async-book/) cover this topic more in depth.

### The Stream trait

As seen before, a stream is like an async iterator. Unlike the `Iterator` trait, however, `Stream` is not part of the standard library. However, there is a very common definition from the `futures` crate. Note how:
- `Iterator` provides a `next` method returning `Option<Self::Item>`
- `Future` provides a `poll` method returning `Poll<Self::Output>`

The `Stream` trait combines both with a `poll_next` method:
```Rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
  type Item;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```

The return type of this function is `Poll<Option<T>>`, where:
- The outer type is `Poll`, because it has to be checked for readiness, just like futures
- The inner type is `Option`, because it needs to signal whether there are more messages, just like iterators

In the previous examples, however, we didn't use `poll_next` or `Stream`. Instead, we used `next` and `StreamExt`. We could work directly with the `poll_next` API by hand-writing our own `Stream` state machines, just as we could work with the futures directly via their `poll` method. Using `await` is much easier, though, and the `StreamExt` trait from our examples supplies the `next` method so that we can simply `await` it:
```Rust
trait StreamExt: Stream {
  async fn next(&mut self) -> Option<Self::Item>
  where
    Self: Unpin;
  // other methods...
}
```

// TODO: idea: write a very small program using Futures and building the state machine myself, and call the `poll` methods of the futures

The `StreamExt` trait contains other utility methods. `StreamExt` is automatically implemented for every type that implements `Stream`, but these traits are defined separately to enable the contributors to iterate on these utility methods without affecting the foundational trait.

In the version of `StreamExt` used in the `trpl` crate, the trait not only defines the `next` method, but also supplies a default implementation of `next` that handles the details of calling `Stream::poll_next`. This means that a self-defined streaming data type only needs to implement `Stream`, and consumers of this new type will be able to use the utility methods from `StreamExt` automatically.

## Summary: Futures, Tasks and Threads


Some pros of async (vs. threads) include:
- Doesn't incur on an extra memory footprint per task, as threads do
  - Million of async tasks can be handled, but not millions of threads (RAM runs out)
- Doesn't have overhead for startup and shutdown
- Doesn't need an OS exposing a thread API and that manages execution
- Async tasks can be cancelled, but threads in Rust can't
- The async APIs are more sophisticated, so it is easier to build helpers such as `timeout` and `throttle`

Other things to consider:
- Async needs a runtime (which someone must code) to handle tasks (instead of threads, which use the OS's capabilities to manage threads)
- Threads act as a boundary for sets of synchronous operations: concurrency is possible *between* threads
- Tasks act as a boundary for sets of asynchronous operations: concurrency is possible *between* and *within* tasks, because a task can switch between futures in its body
- Futures are Rust's most granular unit of concurrency, and each future may represent a tree of other futures. The runtime (specifically, its executor) manages tasks, and tasks manage futures.
  - In that regard, tasks are similar to lightweight, runtime-managed (as opposed to OS-managed) threads

In many cases, a combination of async and threads is the best solution to deal with concurrency and parallelism. In fact, the tokio crate provides a thread pool, and so do other frameworks and languages. Many runtimes use an approach called *work stealing* to transparently move tasks around between threads, based on how the threads are currently being utilized.

When choosing what method to use, consider the rules of thumb:
- If the work is very *parallelizable*, such as processing a bunch of data where each part can be processed separately, threads are a better choice
- If the work is very *concurrent*, such as handling messages from different sources that may come in at different intervals or rates, async is a better choice

A snippet that combines both approaches (threads for sending and async for receiving) is the following:
```Rust
use std::{thread, time::Duration};

fn main() {
  let (tx, mut rx) = trpl::channel();

  thread::spawn(move || {
    for i in 1..11 {
      tx.send(i).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  trpl::run(async {
    while let Some(message) = rx.recv().await {
      println!("{message}");
    }
  });
}
```

This could represent a set of video encoding tasks using a dedicated thread, but notifying the UI that those operations are done with an async channel.