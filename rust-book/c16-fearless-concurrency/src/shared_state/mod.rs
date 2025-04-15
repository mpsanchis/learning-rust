use std::sync::{Arc,Mutex};
use std::thread;

mod lock_poisoning;

pub use lock_poisoning::lock_poisoning_example;

pub fn mutex_usage_single_thread() {
  println!("creating a Mutex<5>");
  let m: Mutex<u8> = Mutex::new(5);

  {
    println!("modifying Mutex<5> to Mutex<6> by acquiring lock in a MutexGuard smart pointer");
    let mut num = m.lock().unwrap();
    *num = 6;
    println!("MutexGuard releases the lock when it goes out of scope");
    // the Drop trait takes care of releasing the lock
  }

  println!("the mutex now looks like: {m:?}");
}

pub fn mutex_usage_multi_thread() {
  println!("creating an Arc<Mutex<0>>:");
  println!("let counter = Arc::new(Mutex::new(0))");
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  println!("spawning 10 threads, all of which do:");
  println!("\tmove an Arc::clone(&counter) into the thread");
  println!("\tlet mut num = counter.lock().unwrap()");
  println!("\t*num += 1");
  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();

      *num += 1;
    });
    handles.push(handle);
  }

  println!("calling handle.join() for all handles from the 10 threads");
  for handle in handles {
      handle.join().unwrap();
  }

  println!("Result at the end in main: {}", *counter.lock().unwrap());
}