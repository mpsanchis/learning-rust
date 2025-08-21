use std::{
  sync::{Arc, Mutex, mpsc},
  thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let msg = receiver
          .lock()
          .expect("Lock for receiving messages was poisoned")
          .recv();

        if let Ok(job) = msg {
          println!("Worker #{id} got a job. Executing...");
          job();
        } else {
          println!("Worker #{id} disconnected. Shutting down...");
          break;
        }
      }
    });
    Worker { id, thread }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender: Option::Some(sender) }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());
    for _ in 0..self.workers.len() {
      if let Some(Worker { id, thread }) = self.workers.pop() {
        thread.join().unwrap();
        println!("Worker {id} shut down");
      }
    }
  }
}
