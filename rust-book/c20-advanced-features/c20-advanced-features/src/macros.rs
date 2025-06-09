use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[macro_export]
macro_rules! my_vec {
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

pub fn declarative_macros() {
  let v = my_vec![1,2,3];
  println!("\n## Using declarative macro my_vec!, that emulates vec!");
  println!("my_vec![1,2,3] = {v:?}");
}

pub fn custom_derive_macro() {
  #[derive(HelloMacro)]
  struct Pancakes;

  println!("\n## Custom derive macro for an empty Pancakes struct");
  Pancakes::hello_macro();
}