use std::sync::Mutex;

pub mod lock_poisoning;

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