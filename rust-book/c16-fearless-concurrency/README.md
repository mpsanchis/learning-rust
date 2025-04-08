# Fearless Concurrency

Rust leverages ownership and a strong type system to help with concurrency. High-level programming languages can promise certain benefits by taking control of certain aspects (e.g. the event loop in Node). However, low-level programming languages are expected to provide fewer abstractions over the hardware, and be more performant. Therefore, Rust offers a variety of tools to deal with concurrency. This chapter will cover:
* How to create threads
* Message-passing concurrency, where channels send messages between threads
* Shared-state concurrency, where multiple threads have access to the same picece of data
* The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types, as well as types provided by the std library

## Using threads to run code simultaneously

Concurrency can hugely improve performance, but it can also lead to problems such as:
* Race conditions, where threads are accessing data or resources in an inconsistent order
* Deadlocks, where two threads are waiting for each other
* Bugs that happen only in certain situations and are hard to reproduce

Many OSs provide an API with which to create threads. The Rust std lib uses a 1:1 model of thread implementation: a program uses one OS thread per language thread. There are crates that use other models.

### Creating a new thread with 'spawn'

The simplest way to create a new thread is to call `thread::spawn` and pass it a closure, such as in:
```rust
thread::spawn(|| {
  for i in 1..10 {
    println!("hi number {i} from the spawned thread!");
    thread::sleep(Duration::from_millis(1));
  }
});
```

### Waiting for all threads to finish with 'join'

The `spawn` method returns an instance of `JoinHandle` that we can decide to store in a variable:
```rust
let handle == thread::spawn( /** closure **/ );
```

the `JoinHandle` type has a `join` method, that blocks the thread until the handle is done:
```rust
handle.join().unwrap();
```

### Using 'move' closures with threads

The `move` keyword (briefly introduced in chapter 13) is often used when dealing with threads, to take ownership of the values used from the environment from the main thread into the spawned thread.

The following code does **NOT** compile:
```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(|| {
    println!("Here's a vector: {v:?}");
});

handle.join().unwrap();
```

and throws the error:
```
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
...
note: function requires argument type to outlive `'static`
```

Rust is inferring how to capture `v`: because `println!` needs only a reference, the closure (`|| {println!("{v:?}")}`) tries to borrow `v`. However, rust can't tell how long the spawned thread will live, so it doesn't known if the reference to `v` will always be valid.
One way to solve this is to `move` the vector into the closure:
```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {v:?}");
});
```

Note that this will prevent future usage of `v` outside the closure, since now its value is owned.

## Using message passing to transfer data between threads

To accomplish message-sending concurrency, rust's stdlib provides an implementation of channels. A *channel* is a general programming concept by which data is sent from one thread to another.

A first channel we can create is by using `std::sync::mpsc`, which stands for *multiple producer, single consumer*. An example of its usage is:
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi from child thread");
    tx.send(val).unwrap(); // own the tx
  });

  let rx_msg = rx.recv().unwrap();
  println!("In 'main' I received the message: {rx_msg}");
}
```

The receiver (`rx`) has two useful methods:
* `recv`: blocks the thread and waits until a value is sent down the channel. Once sent, returns a `Result<T, E>`. When channel is closed, returns an error.
* `try_recv`: doesn't block the thread, and returns a `Result<T, E>` immediately (an `Ok` holding a message if available, and an `Err` if no messages).

### Channels and ownership transference

The `send` function takes ownership of its parameter: this means that once a thread sends a message, it cannot re-use the value it has sent (unless it has cloned it). This ensures that data is properly owned by a single thread, and if we want to duplicate data, it must be explicitly stated.

In other words, the following code does **NOT** compile:
```rust
thread::spawn(move || {
  let val = String::from("hi");
  tx.send(val).unwrap(); // takes ownership
  println!("val is {val}"); // value borrowed after move
});
```

### Sending multiple values and seeing the receiver waiting

We can treat the `rx` as an iterator, and get its values while waiting:
```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
  // ...
  tx.send(/** stuff **/).unwrap();
  // ...
});

for received_msg in rx {
  println!("In main received: {received_msg}");
}
```

### Creating multiple producers by cloning the transmitter

We can clone the trasnmitter (`tx.clone()`), and use it in different threads:
```rust
let (tx1, rx) = mpsc::channel();
let tx2 = tx1.clone();

thread::spawn(move || {
  // ...
  tx1.send(/** stuff **/).unwrap();
  // ...
});
thread::spawn(move || {
  // ...
  tx2.send(/** stuff **/).unwrap();
  // ...
});

for received_msg in rx {
  println!("In main received: {received_msg}");
}
```

## Shared-state concurrency

There's a certain parallelism between:
* message passing <=> single ownership
* memory sharing <=> multiple ownership

which makes memory sharing difficult. There are some techniques to deal with these difficulties.

### Using mutexes to access data from one thread at a time

Mutexes (mutual exclusion data structures) contain data and a lock, and one has to remember to:
1. Attempt to acquire the lock before using the data
2. Unlock the data after using it, to make it available to other consumers

#### The API of Mutex<T>

