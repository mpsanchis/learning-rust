use std::io;

fn main() {

  // booleans
  let t = true;
  let _f: bool = false; // with explicit type annotation

  // shadowing
  let x = 25;
  if t == true {
    let x = 26;
    println!("In the if condition, x = {x}");
  }
  println!("Outside of the if condition, x = {x}");

  // characters: all Unicode
  let _heart_eyed_cat = '😻';

  // tuples (fixed length, different types)
  let tup: (i32, f32, u8) = (500, 6.6, 1);
  let (_a, b, _c) = tup;
  println!("The first element of the tuple is {0} and the second is {1}", tup.0, b);

  // arrays (fixed length, same types). Useful when:
  // - you want data allocated in stack instead of heap
  // - you want to ensure a fixed number of elements
  let arr = [1, 2, 3, 4, 5];
  
  // Declaring array with type and size:
  let arr2: [i32; 5] = [1, 2, 3, 4, 5];

  // Call a function
  print_labeled_measurement(42, 'h');
  statement_vs_expression();

  // Branching
  let mut print_hello = String::new();

  println!("Please write true or false: ");
  io::stdin()
    .read_line(&mut print_hello)
    .expect("Failed to read line");

  if print_hello == "true" {
    println!("hello is being printed");
  } else {
    println!("goodbye is being printed");
  }
}

// Example of function with two arguments (note the types are required)
fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn statement_vs_expression() {
  let y = {
    let x = 3;
    x + 1 // note there is no semicolon here
  };

  println!("The value of y is: {y}");
}

fn five() -> i32 {
  5 // no need to return: the last expression in the function is returned
}
