use super::boxtype::MyBox;
use std::ops::Deref;

pub fn dereferencing_code(mybox: &MyBox<String>) {
  println!("### We can call hello(name: &str) with &MyBox: hello(&mybox)");
  hello_print_ref_to_str(mybox);

  println!("### Rust will equivalently call: hello((*(mybox.deref())).deref())");
  hello_print_ref_to_str((*mybox).deref());

  println!("### Without deref coercion, we would have to transform our Box and call: hello(&(*mybox)[..])");
  hello_print_ref_to_str(&(*mybox)[..]);
}

pub fn dereferencing_and_modifying(mybox: &mut MyBox<String>) {
  println!("### Printing the contents of a mutable box:");
  hello_print_ref_to_str(mybox);

  println!("### Modifying the contents of a mutable box with '*'. New value: w0rld");
  // typeof mybox:    &MyBox
  // typeof *mybox:   *(&MyBox) = MyBox
  // typeof **mybox:  *MyBox    = *(MyBox.deref()) = *(&String) = String
  **mybox = String::from("w0rld");
  hello_print_ref_to_str(mybox);
}

fn hello_print_ref_to_str(name: &str) {
  println!("Hello {name}");
}