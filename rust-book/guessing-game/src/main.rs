use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("** Welcome to the number guessing game! **\n");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("The secret number is: {secret_number}");

  loop {
    println!("Please input your guess.");
    let mut guess = String::new(); // mutable variable (vars are immutable by default)
  
    // https://doc.rust-lang.org/std/io/struct.Stdin.html
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
  
    // variable shadowing => same name, used to change type but keeping name
    let guess: u32 = match guess.trim().parse() {
      Ok(foo) => foo,
      Err(_) => {
        println!("Your input must be a number. Try again...");
        continue;
      }
    };
  
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
  
}
