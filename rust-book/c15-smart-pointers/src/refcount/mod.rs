#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

pub fn ref_count() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("Created a: {a:?}");
  println!("(strong) count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("Created b: {b:?}");
  println!("(strong) count after creating b = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("Created c: {c:?}");
    println!("(strong) count after creating c, for a = {}", Rc::strong_count(&a));
  }
  println!("(strong) count after c goes out of scope = {}", Rc::strong_count(&a));
}