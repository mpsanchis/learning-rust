use std::env;
use std::process;

use minigrep::Config;

fn main() {
  println!("Starting minigrep");
  let args: Vec<String> = env::args().collect(); // note: will panic if invalid Unicode

  let config = Config::build(&args).unwrap_or_else(|err| {
    eprintln!("Error when parsing arguments: {err}");
    process::exit(1);
  });

  if let Err(err) = minigrep::run(config) {
    eprintln!("Application error: {err}");
    process::exit(1);
  }
}


