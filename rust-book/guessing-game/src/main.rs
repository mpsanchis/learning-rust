use std::io;

fn main() {
    println!("Hello, world!");
    println!("Input your guess.");

    let mut guess = String::new(); // mutable variable (vars are immutable by default)

    // https://doc.rust-lang.org/std/io/struct.Stdin.html
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
