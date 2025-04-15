mod threads_intro;
mod message_passing;
mod shared_state;

use threads_intro::{wait_for_spawned_thread, wait_at_the_end, move_var_to_thread};
use message_passing::{simple_msg_passing, multiple_sent_messages, multiple_transmitters_multiple_messages};
use shared_state::{lock_poisoning_example, mutex_usage_single_thread, mutex_usage_multi_thread};

fn main() {
  println!("# CH16: Fearless concurrency");

  println!("\n# Intro to threads");
  threads_intro();

  println!("\n# Message passing");
  message_passing();

  println!("\n# Shared state");
  shared_state();
}

fn threads_intro() {
  println!("## Creating a spawned thread, continuing working, and waiting for the thread at the end (random order)");
  wait_at_the_end();
  println!("\n## Creating a spawned thread and waiting for it to finish before continuing (predictable order)");
  wait_for_spawned_thread();
  println!("\n## Moving a variable into the closure passed to the thread");
  move_var_to_thread();
}

fn message_passing() {
  println!("## Creating a child thread which sends a message to main thread");
  simple_msg_passing();

  println!("## Sending several messages (single transmitter) and reading them in main");
  multiple_sent_messages();

  println!("## Sending several messages (multiple transmitters) and reading them in main");
  multiple_transmitters_multiple_messages();
}

fn shared_state() {
  println!("## Using a Mutex<T>");
  mutex_usage_single_thread();

  println!("##\nLock poisoning when using Mutex<T>");
  lock_poisoning_example();

  println!("## Using an Arc<Mutex<T>> in multi-threaded scenarios");
  mutex_usage_multi_thread();
}