use std::str::FromStr;
use rand::Rng;
use std::io::{self, Write};

pub enum FirstLetter {
  A,
  E,
  I,
  O,
  U
}

const WORDS_A: &[&str] = &["apple", "ant", "arrow", "astronaut", "avocado", "art"];
const WORDS_E: &[&str] = &["elephant", "eagle", "engine", "egg", "elevator", "earth"];
const WORDS_I: &[&str] = &["igloo", "ice", "icon", "island", "ink", "idea"];
const WORDS_O: &[&str] = &["octopus", "orange", "orbit", "owl", "ocean", "onion"];
const WORDS_U: &[&str] = &["umbrella", "unicorn", "unit", "utopia", "ukulele", "universe"];

impl FromStr for FirstLetter {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "a" => Ok(FirstLetter::A),
      "e" => Ok(FirstLetter::E),
      "i" => Ok(FirstLetter::I),
      "o" => Ok(FirstLetter::O),
      "u" => Ok(FirstLetter::U),
      _ => Err(())
    }
  }
}

impl FirstLetter {

  fn get_random_word(&self) -> &'static str {
    // return a random word from the array for the letter
    let word_list = match self {
      FirstLetter::A => WORDS_A,
      FirstLetter::E => WORDS_E,
      FirstLetter::I => WORDS_I,
      FirstLetter::O => WORDS_O,
      FirstLetter::U => WORDS_U,
    };

    let mut rng = rand::rng();
    let random_position = rng.random_range(0..word_list.len());
    return word_list.get(random_position).unwrap();
  }
}

pub struct WordPrinter {
  first_letter: FirstLetter
}

impl WordPrinter {

  pub fn new(first_letter: FirstLetter) -> Self {
    WordPrinter { first_letter }
  }

  pub fn change_first_letter(&mut self, new_letter: FirstLetter) {
    self.first_letter = new_letter;
  }

  pub fn print_word(&self) {
    // Save cursor position
    print!("\x1B[s");
    // Move cursor up two lines
    print!("\x1B[2A");
    // Clear the line
    print!("\x1B[2K");
    // Move cursor to the beginning of the line
    print!("\x1B[1G");
    // Print the word
    print!("{}", self.first_letter.get_random_word());
    // Restore cursor position to where input happens
    print!("\x1B[u");
    io::stdout().flush().unwrap();
  }
}