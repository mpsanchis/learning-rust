# 1. How does Rust know when the elements of the vector are out of scope
..and thus they can be dropped (and thus, decrement the counter of the `Rc`)?

In the following example, an element is removed from the vector, and its ownership could be passed to another variable (depending on user input).
- If user passes "2 <bool>", the strong count always remains at 2, because we deleted a String.
- If user passes "1 true" (transfer ownership), then the strong count remains at 2, because we deleted an Rc from the vector, but the variable is still in scope.
- If user passes "1 false" (drop), then the strong count decreases to 1, because we deleted an Rc from the vector, and no other variable took ownership.

However, the decisions are only known at runtime. The doubt is: who takes care of deallocating the variable being dropped, and therefore allowing the Rc to decrement?

```rust
use std::rc::Rc;
use std::env;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum ListOrString {
  String(String),
  List(Rc<List>)
}

use crate::List::{Cons, Nil};

fn main() {
    let mut vec: Vec<ListOrString> = Vec::new();

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Rc::clone(&a); // `b` is another Rc pointing to the same data

    let mut args = env::args();
    args.next();
    let to_delete = match args.next() {
      None => panic!("Could not read program args"),
      Some(first_arg) => first_arg.parse::<usize>().unwrap()
    };
    if to_delete != 1 && to_delete != 2 {
      panic!("You can only delete elements 1 or 2 from the vector");
    }
    let to_move = match args.next() {
      None => panic!("Could not read program args"),
      Some(second_arg) => second_arg
    };
    if !to_move.eq("true") && !to_move.eq("false") {
      panic!("You can only pass true or false as second argument");
    }

    vec.push(ListOrString::List(a));
    vec.push(ListOrString::List(b));
    vec.push(ListOrString::String(String::from("hello")));

    if let ListOrString::List(rc) = vec.get(0).unwrap() {
      println!("Strong count after adding a and b to vec: {}", Rc::strong_count(rc));
    }

    println!("Deleting element: {to_delete}");

    if to_move.starts_with("true") {
      let rmvd = vec.remove(to_delete);
      println!("Removed {rmvd:?} and passing ownership");
      if let ListOrString::List(rc) = vec.get(0).unwrap() {
        println!("Strong count after removing and passing ownership: {}", Rc::strong_count(rc));
      }
    } else {
      println!("Removed {:?} and not passing ownership", vec.remove(to_delete));
      if let ListOrString::List(rc) = vec.get(0).unwrap() {
        println!("Strong count after removing and not passing ownership: {}", Rc::strong_count(rc));
      }
    }
}
```