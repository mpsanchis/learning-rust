use std::thread;
use std::time::Duration;

fn print_five_from(thead_name: &str) {
  for i in 1..5 {
      println!("> number {i} from thread: {thead_name}");
      thread::sleep(Duration::from_millis(1));
  }
}

pub fn wait_at_the_end() {
  let handle = thread::spawn(|| print_five_from("spawned"));
  print_five_from("main");

  handle.join().unwrap();
}

pub fn wait_for_spawned_thread() {
  let handle = thread::spawn(|| print_five_from("spawned"));

  handle.join().unwrap();

  print_five_from("main");
}

pub fn move_var_to_thread() {
  let vec = vec![1, 2, 3];
  println!("Creating a vector in the 'main' thread: {vec:?}");
  let handle = thread::spawn(move || println!("Printing moved vector from thread: {vec:?}"));

  // println!("Printing again from 'main' thread: {vec:?}"); // Does NOT compile

  handle.join().unwrap();
}