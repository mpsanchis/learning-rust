pub fn function_pointers() {
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
  }
  println!("\n## Function composition (different from rust book (!)):");
  println!("do_twice(add_one, 3) = {}", do_twice(add_one, 3));
}