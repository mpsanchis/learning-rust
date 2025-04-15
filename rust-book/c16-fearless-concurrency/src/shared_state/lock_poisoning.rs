use std::sync::{Arc, Mutex};
use std::thread;

pub fn lock_poisoning_example() {
  println!("creating a new Mutex<1>");
  let mutex = Arc::new(Mutex::new(1));

  // poison the mutex
  let c_mutex = Arc::clone(&mutex);
  let _ = thread::Builder::new().name("panicking_thread".to_string()).spawn(move || {
    println!("modifying Mutex<1> to Mutex<2> in thread '{}', and then panicking...", thread::current().name().unwrap_or("unknown"));
    let mut data = c_mutex.lock().unwrap();
    *data = 2;
    panic!();
  }).unwrap().join();

  match mutex.lock() {
      Ok(_) => unreachable!(),
      Err(p_err) => {
          let data = p_err.get_ref();
          println!("main thread found poisoned lock, and recovered data: {data}");
      }
  };
}