# Reading a file async in Rust

## Linux API

The two main approaches are epoll and io_uring. The former is older and more widespread.

### epoll

Consists of several functions that achieve the async functionality if leveraged in different threads/processes.
The first type of functions are those that configure an object and tell the OS to block current thread until a file descriptor is ready:
- `open`, `epoll_create`: set up a config to ask the kernel to perform I/O
- `epoll_ctl`: adds items to a list of `fd`s to monitor
- `epoll_wait`: blocks until any of the monitored `fd`s is ready

The second is simply the `read` function, which won't block. Instead, it will return `EAGAIN` if content is not ready:
- `read`: attempts to load content of the file into a buffer

```
Your thread                          Kernel
─────────                            ──────
open(file) → fd
epoll_create() → epoll_fd
epoll_ctl(epoll_fd, ADD, fd, ...)   // "watch this fd"
epoll_wait(epoll_fd, ...)  ─────────► blocks until fd is readable
                           ◄───────── returns: "fd #7 is ready"
read(fd, buf, len)                   // now this won't block
```

### io_uring

This is the only "true async" approach in Linux:
```
Your thread                          Kernel
─────────                            ──────
// Set up shared ring buffers
io_uring_setup() 
// Submit a read request — don't wait at all
io_uring_sqe { op: READ, fd, buf }  ──► kernel queues the read
// ... do other things ...
io_uring_cqe = completion_queue.pop() ◄── kernel signals: "read done, N bytes"
```
## Tokio architecture

Tokio uses a dedicated thread that blocks on `epoll_wait`:
```
┌─────────────────────────────────────────────────────────┐
│  Tokio Runtime                                          │
│                                                         │
│  Worker threads (N)          I/O Driver thread (1)      │
│  ┌──────────────┐            ┌─────────────────────┐    │
│  │ poll tasks   │            │                     │    │
│  │              │  register  │  epoll_wait(...)    │    │
│  │  .await ──────────────────►  fd → Waker map     │    │
│  │              │            │         │           │    │
│  │              │   wake()   │         │ fd ready  │    │
│  │  ◄───────────────────────── waker.wake()        │    │
│  └──────────────┘            └─────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

Step by step:
1. `tokio::fs::File::read()` opens the file with `O_NONBLOCK` and registers the fd with epoll via `epoll_ctl`
2. It clones the `Waker` from the `Context` and stores it in a `fd → Waker` hashmap (we will call it *reactor* later)
3. It returns `Poll::Pending`
4. The I/O driver thread is sitting in `epoll_wait()` (blocking) watching all registered fds
5. If other reads come, `epoll_ctl` is used to add those fds to the list
6. When the kernel signals the fd is ready, `epoll_wait` returns on the driver thread (thread wakes)
7. The driver thread looks up the stored `Waker` and calls `waker.wake()`
8. `wake()` pushes the task back onto the worker thread queue
9. A worker thread polls the task again, this time `read()` succeeds

The `Waker` itself is just a fat pointer to a vtable with a `wake` function — it's pure Rust, no C ABI callbacks registered with the kernel.


## Opening a file with raw epoll

The following snippets show how we could manually replicate what Tokio is doing, by leveraging:
- a list of files to read in a HashMap
- a Driver I/O thread that waits for file descriptors to be ready

Given:
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::task::Waker;
```

we could build a reactor, shared between the I/O driver thread and the futures:
```rust
struct Reactor {
    wakers: Mutex<HashMap<i32, Waker>>,  // fd → Waker
    epoll_fd: i32,
}
impl Reactor {
    fn register(&self, fd: i32, waker: Waker) {
        // Store the waker so the epoll thread can find it later
        self.wakers.lock().unwrap().insert(fd, waker);

        // Tell epoll to watch this fd
        let mut event = libc::epoll_event {
            events: libc::EPOLLIN as u32,
            u64: fd as u64,  // epoll echoes this back to us when fd is ready
        };
        unsafe { libc::epoll_ctl(self.epoll_fd, libc::EPOLL_CTL_ADD, fd, &mut event); }
    }
}
```

The I/O driver thread would continuously loop, waking when OS signals that data is ready, and adding the available tasks to the worker queue:
```rust
fn epoll_loop(reactor: Arc<Reactor>) {
    loop {
        let mut events = [libc::epoll_event { events: 0, u64: 0 }; 64];
        // BLOCKS here until at least one fd is ready
        let n = unsafe { libc::epoll_wait(reactor.epoll_fd, events.as_mut_ptr(), 64, -1) };

        for event in &events[..n as usize] {
            let fd = event.u64 as i32;
            // Look up and remove the waker for this fd, then call it
            if let Some(waker) = reactor.wakers.lock().unwrap().remove(&fd) {
                waker.wake();  // This pushes the task back onto the worker queue
            }
        }
    }
}
```

and our Futures would attempt to `read`, knowing that the call is non-blocking (thanks to `epoll`):
```rust
struct AsyncFileRead {
    fd: i32,
    reactor: Arc<Reactor>,
    registered: bool,
}

impl Future for AsyncFileRead {
    type Output = Vec<u8>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try the read first — maybe data is already ready
        let mut buf = vec![0u8; 1024];
        let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut _, buf.len()) };

        if n >= 0 {
            buf.truncate(n as usize);
            return Poll::Ready(buf);
        }

        let err = unsafe { *libc::__errno_location() };
        if err == libc::EAGAIN {
            // Not ready — register our waker so the epoll thread wakes us
            self.reactor.register(self.fd, cx.waker().clone());
            return Poll::Pending;
        }

        panic!("read error: {}", err);
    }
}
```

A missing piece here would be the runtime: the loop that runs forever, checking for tasks that are ready, and dispatching them. It would create the reactor at startup as well. 
This is what Tokio is.

The flow would be:
```
poll() called
  → read() → EAGAIN
  → reactor.register(fd, waker)   // store waker, add fd to epoll
  → Poll::Pending

[epoll thread, maybe milliseconds later]
  → epoll_wait() returns: "fd 7 is ready"
  → wakers.remove(fd7).wake()     // pushes task onto worker queue

[worker thread]
  → poll() called again
  → read() → returns data this time
  → Poll::Ready(buf)
```

