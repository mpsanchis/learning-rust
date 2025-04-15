# Fearless Concurrency

Rust leverages ownership and a strong type system to help with concurrency. High-level programming languages can promise certain benefits by taking control of certain aspects (e.g. the event loop in Node). However, low-level programming languages are expected to provide fewer abstractions over the hardware, and be more performant. Therefore, Rust offers a variety of tools to deal with concurrency. This chapter will cover:
* How to create threads
* Message-passing concurrency, where channels send messages between threads
* Shared-state concurrency, where multiple threads have access to the same picece of data
* The `Sync` and `Send` traits, which extend Rust's concurrency guarantees to user-defined types, as well as types provided by the std library

## Using threads to run code simultaneously

```
note: idea of concurrency usage without parallelism

write a CLI tool whose:
- main thread listens to the stdin, and expects a letter to be passed. If passed, it transmits it to the secondary thread.
- secondary thread writes words which start with the last letter it received. Checks channel every now and then, and if it sees a new letter, it switches its internal state, and starts printing with that letter.
```

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

In Rust, pointers returned by mutexes implement two fundamental traits:
* The `Drop` trait, which releases the lock automatically when the smart pointer returned when acquiring the lock goes out of scope. This means that (2.) is not a problem.
* The `Deref` trait, which allows us to use the `&` and `*` notation when dealing with the result returned.

#### The API of Mutex<T>

The simplest usage of a mutex can be exemplified by this snippet:
```rust
use std::sync::Mutex;

let m = Mutex::new(5);
{
  let lock_ptr = m.lock(); // Result<MutexGuard, PoisonError>
  let mut num = lock_ptr.unwrap(); // panics, or MutexGuard<'_, u8> --> smart pointer
  *num = 6;
  // lock_ptr dropped --> lock freed
}
println!("m = {m:?}");
```

#### Sharing a Mutex<T> between multiple threads

The book mentions two "naive" approaches to making a Mutex available to multiple threads:
- Directly `move`ing the Mutex into the closures of the threads
- Wrapping the Mutex in an `Rc<Mutex>` to allow for multiple owners

The first one doesn't work, because of the ownership rules: once a thread owns the Mutex, another thread can't use it.

Neither does the latter because, as the compiler clearly states:
```
`Rc<Mutex<i32>>` cannot be sent between threads safely
```

The solution to this is `Arc<T>`: atomic reference counting.

#### Atomic reference counting with Arc<T>

The stdlib provides atomics for many primitives under `std::sync::atomic`, such as `AtomicBool`, `AtomicU16`, etc. These provide thread-safe primitives that can be used in multithreaded programs. Programs can leverage these atomic versions of the primitives, by paying a small runtime penalty.

At the top level of the `std::sync` module we can also find `Arc<T>`, which is the atomic version of `Rc<T>`.

The `Arc` struct also implements the `Clone` trait, by implementing a `clone` function that we can use to create a mutable reference to the shared resource, before spawing a thread that will `move` it into its closure and then access the shared resource:
```rust
let counter = Arc::new(Mutex::new(0));

for _ in 0..10 {
  // clone the mutable reference to the shared resource
  let counter = Arc::clone(&counter);
  let handle = thread::spawn(move || {
    // use the shared resource wrapped in a mutex in each thread
    let mut num = counter.lock().unwrap();
    *num += 1;
  });
  // ...
}
```

#### Similarities between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

1. Note that despite `counter` is immutable, we could get a mutable reference to the value inside it. This is because `Mutex<T>` provides interior mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in Ch15 to mutate contents inside a `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.

2. Also, similarly to how `Rc<T>` can create reference cycles, `Mutex<T>` can create deadlocks.


## Extensible concurrency with the Sync and Send traits

The rust language contains very few concurrency features. Almost all concepts discussed in this chapter are part of the stdlib, not the language. However, two concurrency concepts are embedded in the language: the traits `std::marker::{Sync, Send}`.

### Allowing ownership transference between threads with Send

The `Send` *marker* trait indicates that ownership of values of the type implementing `Send` can be transferred between threads.

Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`: this cannot be `Send` because if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to another thread, both threads might update the ref count at the same time. Rust's type system and trait bounds ensure that you can never accidentally send an `Rc<T>` value accross threads unsafely. When trying, the compiler will throw:
```
the trait Send is not implemented for Rc<Mutex<T>>
```

Note: any type composed entirely of `Send` types is marked as `Send` as well. Almost all primitive types are `Send`, aside from raw pointers, which are discussed in Chapter 20.

### Allowing access from multiple threads with Sync

The `Sync` *marker* trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words:
* `&T` is `Send` ==> `T` is `Sync`

Similar to `Send`, primitive types are `Sync`, and types composed entirely of types that are `Sync` are also `Sync`.

Note that:
* `Rc<T>` is not `Sync` for the same reason it's not `Send`. The `RefCell<T>` type and the family of related `Cell<T>` types are not `Sync`. The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe.
* the smart pointer `Mutex<T>` is `Sync` and can be used to share access with multiple threads, as shown in this chapter.

### Implementing 'Send' and 'Sync' manually is unsafe

As *marker* traits, these traits don't have any methods to implement. In most cases we should not need to implement them, since they are inherited by complex types that use simpler types (that do implement `Sync` and `Send`).

More on unsafe rust on chapter 20.
