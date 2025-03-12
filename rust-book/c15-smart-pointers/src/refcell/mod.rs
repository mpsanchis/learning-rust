mod messenger;

use messenger::{Messenger, LimitTracker};
use std::{cell::RefCell, ops::Deref, rc::Rc};

#[derive(Debug)]
struct MessengerLogger ();

impl MessengerLogger {
  fn new() -> MessengerLogger {
    MessengerLogger()
  }
}
impl Messenger for MessengerLogger {
  fn send(&self, msg: &str) {
    println!("Sending: {msg}...");
  }
}

fn refcell_example() {
  println!("\n## Hello RefCell: using RefCell<T> in a mock object which runs in the tests (run 'cargo test -- --nocapture')");
  let messenger_logger = MessengerLogger::new();
  let mut limit_tracker = LimitTracker::new(&messenger_logger, 5);
  println!("Creating LimitTracker: {limit_tracker:?}");
  println!("Setting LimitTracker value to 3:");
  limit_tracker.set_value(3);
  println!("Setting LimitTracker value to 4:");
  limit_tracker.set_value(4);
  println!("Setting LimitTracker value to 5:");
  limit_tracker.set_value(5);
}

fn refcell_and_rc() {
  println!("\n## Rc<RefCell<T>>: multiple mutable references");
  // WIP: attempt to have a simpler example than the one in the book (why would we need an enum?)
  #[derive(Debug)]
  struct MyMutableStruct {
    data: Rc<RefCell<i32>>
  }
  println!("Creating a data owner with:\n let val = Rc::new(RefCell::new(42))");
  let val = Rc::new(RefCell::new(42)); // owner

  println!("Creating 2 Structs that clone the Rc in them:\n let refx = MyMutableStruct {{ data: Rc::clone(&val) }}");
  let ref1 = MyMutableStruct { // co-owner
    data: Rc::clone(&val)
  };
  let ref2 = MyMutableStruct {
    data: Rc::clone(&val)
  };

  println!("Mutating 'val' with its original owner:\n *(val.borrow_mut()) = 10");
  *(val.borrow_mut()) = 10;
  println!("ref1: {ref1:?}");
  println!("ref2: {ref2:?}");

  println!("Mutating 'val' by mutably borrowing it from an object that contains the data:\n *(ref1.data.borrow_mut()) = 20");
  *(ref1.data.borrow_mut()) = 20;
  println!("ref1: {ref1:?}");
  println!("ref2: {ref2:?}");

  let foo = ref2.data.borrow();

  // Playing with deref coercion...
  print_inner_value(&foo);
  let value = Rc::new(RefCell::new(5));
  let b = value.borrow(); // equivalent to: (*value).borrow();
  println!("Inner value is: {}", &value.borrow()); // equiv to: &*(*value).borrow()
  print_inner_value(&b);
}

fn print_inner_value(val: &i32) {
  println!("the value is: {val}");
}

pub fn refcell_usage() {
  refcell_example();
  refcell_and_rc();
}