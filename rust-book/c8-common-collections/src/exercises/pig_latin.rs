const VOWELS: [char; 5] = ['a','e','i','o','u'];

pub fn to_pig_latin(word: &str) {
  let first_letter_opt = word.chars().nth(0);

  if first_letter_opt.is_none() {
    println!("Word '{word}' has no letters");
    return;
  }

  let first_letter = first_letter_opt.unwrap();

  if VOWELS.contains(&first_letter) {
    println!("pig_latin({word}) = {word}-hay");
  } else {
    let rest_of_word = word.get(1..).unwrap_or("");
    if rest_of_word.eq_ignore_ascii_case("") {
      println!("pig_latin({word}) = {first_letter}ay");
    } else {
      println!("pig_latin({word}) = {rest_of_word}-{first_letter}ay");
    }
  }
}