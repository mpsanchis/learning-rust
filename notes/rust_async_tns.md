# Rust async: as explained by TNS

This is a summary of the content from a series by "The New Stack" newsletter. Series starts [here](https://thenewstack.io/how-rust-does-async-differently-and-why-it-matters/).

## Part 1: The poll-based model

### 1. The pull model: Laziness as a virtue

In many languages (Js Promises, Go Goroutines), async operations are "push-based":
- runtime schedules operation
- result is pushed when done

In rust, Futures are lazy, "pull-based": if you call an `async` function but don't `.await` it, nothing happens. Code from that function is not executed. 

The following example shows this behavior:
```rust
use std::time::Duration;
 
async fn complex_calculation() {
  println!("(2) Starting calculation...");
  tokio::time::sleep(Duration::from_secs(1)).await;
  println!("(3) Calculation finished!");
}
 
#[tokio::main]
async fn main() {
  println!("(1) Calling the function...");
   
  // ⚠️ NOTHING HAPPENS HERE
  // The function is called, but the code inside isn't executed yet.
  // It returns a 'Future' state machine.
  let my_future = complex_calculation();
   
  println!("(4) I haven't awaited it yet, so nothing printed above.");
   
  // 🚀 NOW the runtime starts pulling the future
  my_future.await;
}
```

The `Future` is a paused state machine. The runtime (e.g. Tokio) will start polling the future only when `.await` is called.

### 2. The Future trait: the engine under the hood

The `Future` trait looks like something like this:
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

When you write an `async` function, the Rust compiler automatically:
- Generates an anonymous struct that implements the `Future` trait
- Transforms your linear code into a state machine, breaking the function at every `.await` point

We could construct the struct ourselves, like in this example:
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct CountdownFuture {
    count: u32,
}
impl CountdownFuture {
    fn new(count: u32) -> Self { CountdownFuture { count } }
}
impl Future for CountdownFuture {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            return Poll::Ready("done!".to_string());
        }
        println!("Counting down: {}", self.count);
        self.count -= 1;
        // Tells the runtime to put task back in the queue, ready to be polled
        // Without this, runtime would put this taks to sleep and never check it again
        cx.waker().wake_by_ref();
        Poll::Pending;
    }
}

#[tokio::main]
async fn main() {
    let cd = CountdownFuture::new(3);
    let rs = cd.await;
    println!("{}", rs);
}
```

### 3. Breaking down the magic

In the poll signature, the following details are important:
- `Pin<&mut Self>`: allows us to mutate our state (`self.count`). The Pin wrapper ensures that we are safe to use even if we were self-referential (not leveraged in this example)
- `Context`: carries the waker, which is the "callback" mechanism. In a real-world scenario (e.g.: reading from a socket), you would hand this waker to the OS, and the OS would trigger it later when data arrives.
