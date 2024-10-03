
pub fn create_strings() {
  println!("\n### Create strings");

  let s0 = String::new();
  let s1 = "initial contents".to_string();
  let s2 = String::from("initial contents");

  // Note: &str is a pointer to part of a sequence of characters owned by someone else
  let _lit = "string literal"; // a &str because the string literal is compiled with the code => it lives in the code section of memory (not heap)
  let _foo = s1.strip_prefix("string ").unwrap_or("literal"); // a &str because it points to a sequence of chars owned by someone else (the String)

  println!("The string '{s0}' has been created with: String::new()");
  println!("The string '{s1}' has been created with: \"initial contents\".to_string()");
  println!("The string '{s2}' has been created with: String::from(\"initial contents\")");
}

pub fn update_strings() {
  println!("\n### Update strings");
  let mut s0 = String::new();
  println!("The string s0: '{s0}' has been created with: String::new()");
  s0.push_str("hello");
  println!("After .push_str(\"hello\"), s0 is: '{s0}'");
  s0.push(',');
  println!("After .push(','), s0 is: '{s0}'");
  let s1 = String::from(" world!");
  println!("s1 is: '{s1}'");
  let mut s2 = s0 + &s1; // note s0 has been moved here and can no longer be used
  println!("s2 = s0 + s1 = {s2}");
  s2 += &s1;
  println!("s2 += &s1 expectedly leads to: {s2}");
  // println!("s0 is: {s0}") // this line does not compile: s0 has been moved with the add
  // Implementation is efficient: s2 is now the owner of the memory that used to belong to s1 (no copying extra)
  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");
  let tictactoe = format!("{tic}-{tac}-{toe}");
  println!("Using the format! macro, we can obtain a String such as: '{tictactoe}' by combining other Strings");
}

pub fn access_string_indices() {
  println!("\n### Access strings using indices (generally a bad idea)");
  let hello = "Здравствуйте";
  println!("Hello can be written as: {hello}");
  let not_a_three_opt = hello.get(0..2);
  if not_a_three_opt.is_some() {
    let not_a_three = not_a_three_opt.unwrap();
    println!("Accessing the first two bytes of 'Здравствуйте' with '.get(0..2) returns: {not_a_three}");
    // let panic_runtime &str = &hello[0..3]; // this breaks: we're accessing bytes halfway through a char
  }
}

pub fn iterate_over_string() {
  let string_with_diacritics = "grüezi";
  println!("\n#### Iterating over a string, such as '{string_with_diacritics}'");
  let string_length = string_with_diacritics.len();
  println!("Length of the 6-letter string: {string_length}");
  println!("Iterating string by chars:");
  print!("{}", string_with_diacritics.chars().nth(0).unwrap());
  for c in string_with_diacritics.chars().skip(1) {
    print!(",{c}");
  }
  println!("\t(length: {})", string_with_diacritics.chars().count());
  println!("Iterating string by bytes:");
  print!("{}", string_with_diacritics.bytes().nth(0).unwrap());
  for c in string_with_diacritics.bytes().skip(1) {
    print!(",{c}");
  }
  println!("\t(length: {})", string_with_diacritics.bytes().count());

}