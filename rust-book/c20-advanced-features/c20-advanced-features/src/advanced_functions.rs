pub fn function_pointers() {
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
  }
  println!("\n## Function composition (different from rust book (!)):");
  println!("do_twice(add_one, 3) = {}", do_twice(add_one, 3));

  #[derive(Debug)]
  enum Status {
    Value(u32),
    Stop,
  }

  let list_of_statuses = (0..4).map(Status::Value).collect::<Vec<Status>>();

  println!("\n Using initializer functions as closures");
  println!("(0..3).map(Status::Value).collect::<Vec<Status>>():\n{:?}", list_of_statuses);
}

pub fn returning_closures() {
  fn returns_closure_alone() -> impl Fn(i32) -> i32 {
    |x| x + 1
  }

  let closure = returns_closure_alone();

  println!("\n## We can return closures from functions (e.g.: '-> impl Fn(i32) -> i32'):");
  println!("closure(41) = {}", closure(41));

  println!("\n## We need to use trait objects when using many closures with the same signature, to avoid opaque types:");
  type Number2NumberFn = Box<dyn Fn(i32) -> i32>;

  fn one_adder() -> Number2NumberFn {
    Box::new(|x| x + 1)
  }
  fn two_adder() -> Number2NumberFn {
    Box::new(|x| x + 2)
  }

  let my_closures = vec![
    one_adder(),
    two_adder()
  ];

  println!("Return 'Box<dyn Fn(i32) -> i32>' and we can store all closures in a vector");
  for (i, c) in my_closures.iter().enumerate() {
    println!("c_{}(41) = {}", i, c(41));
  }
}