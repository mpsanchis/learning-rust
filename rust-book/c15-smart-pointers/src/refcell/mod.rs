mod messenger;

use messenger::{Messenger, LimitTracker};
use std::{cell::RefCell, cell::Ref, rc::Rc};

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

// Attempt to have a simpler example than the one in the book (why would we need an enum?)
#[derive(Debug)]
struct MyMutableStruct {
  data: Rc<RefCell<i32>>
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
}

fn play_with_deref_coercion() {
  println!("\n### Playing with complex deref coercion...");

  println!("Declaring fn print_me(v: &i32)");
  let print_me = |v: &i32| {
    println!("printing value: {v}");
  };
  let my_str = MyMutableStruct {
    data: Rc::new(RefCell::new(30))
  };
  println!("Creating a struct containing data: Rc<RefCell<30>>: {my_str:?}");

  println!("If foo: Ref<i32>, we can call print_me(&foo), and it is equivalent to print_me(&(*foo))");
  let foo: Ref<i32> = my_str.data.borrow();
  print_me(&foo);

  println!("If bar: &RefCell<i32> (reference), or even Rc<RefCell<i32>> (smart pointer), we can call its methods (bar.borrow() for instance). Rust does the dereferencing");
  let bar = &RefCell::new(30);
  let bar_ref = (*bar).borrow();
  print_me(&bar_ref);

  println!("If baz: &Rc<RefCell<i32>>, we can call print_me(&baz.borrow()), and it is equiv to print_me(&(*((*(*baz)).borrow())))");
  let baz = &my_str.data;
  print_me(&baz.borrow());
}

pub fn refcell_usage() {
  refcell_example();
  refcell_and_rc();
  play_with_deref_coercion();
}