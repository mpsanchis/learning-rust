use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use route_macro::route;

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

type Handler = fn();

// Simulates a routing table, generated at compile time
static ROUTES: Lazy<Mutex<HashMap<(String, String), Handler>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

// Our "route_macro" crate requires "crate::register_route" to exist
pub fn register_route(method: &str, path: &str, handler: Handler) {
    let mut map = ROUTES.lock().unwrap();
    map.insert((method.to_string(), path.to_string()), handler);
}

pub fn attribute_macro() {
  println!("\n## Custom attribute macro: #[route(method, path)]");

  // This could be provided as a test utility by a web framework
  fn handle_request(method: &str, path: &str) {
    let map = ROUTES.lock().unwrap();
    match map.get(&(method.to_string(), path.to_string())) {
      Some(handler) => handler(),
      None => println!("404 Not Found: {} {}", method, path),
    }
  }

  #[route(GET, path = "/")]
  fn index() {
    println!("This is the index page!");
  }

  #[route(POST, path = "/submit")]
  fn submit() {
    println!("Submitted something!");
  }

  handle_request("GET", "/");
  handle_request("POST", "/submit");
  handle_request("GET", "/not-found");
}