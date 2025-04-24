use std::marker::PhantomPinned;

#[derive(Debug)]
struct Move {
  pub data: String,
}

impl Move {
  fn new(data: &str) -> Self {
    Move {
      data: data.to_string(),
    }
  }
}

#[derive(Debug)]
struct NoMove {
  pub data: String,
  _pin: PhantomPinned, // If a type contains a `PhantomPinned`, it will not implement `Unpin` by default
}

impl NoMove {
  fn new(data: &str) -> Self {
    NoMove {
      data: data.to_string(),
      _pin: PhantomPinned,
    }
  }
}

pub fn unpin_example_no_compile() {
  let mut pinned1 = Box::pin(Move::new("first"));
  println!("Pinned data: {}", pinned1.data);
  // This reassignment is allowed
  *pinned1 = Move::new("second");
  println!("Pinned data: {}", pinned1.data);
  // Also this one (less "raw")
  let mut foo = pinned1.as_mut();
  *foo = Move::new("third");
  println!("Pinned data: {}", pinned1.data);


  let mut pinned2 = Box::pin(NoMove::new("hello"));
  println!("Pinned data: {}", pinned2.data);
  // This reassignment is **not allowed** (does not compile):
  /*
  *pinned2 = NoMove::new("goodbye");
  */

  // Only this unsafe block allows us to modify the content of the pinned object:
  /*
  unsafe {
    let raw = Pin::as_mut(&mut pinned2).get_unchecked_mut();
    ptr::write(raw, NoMove::new(String::from("goodbye")));
  }
  */
  println!("Pinned data: {}", pinned2.data);
}