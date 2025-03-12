mod boxtype;
mod dereferencing;

use boxtype::MyBox;
use dereferencing::{dereferencing_code, dereferencing_and_modifying};

pub fn mybox_code() {
  println!("## Box<T> example with MyBox<String>: mybox::new('world')");
  let mybox = MyBox::new(String::from("world"));
  dereferencing_code(&mybox);
  let mut mymutbox = mybox;
  dereferencing_and_modifying(&mut mymutbox);
  dereferencing_code(&mymutbox);
  foo(&mymutbox);
}

fn foo(input: &str) {
  println!("Printing: {input}");
}