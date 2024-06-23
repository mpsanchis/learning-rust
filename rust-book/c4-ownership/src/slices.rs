pub fn slices() {
  let s = String::from("hello world");
  
  let fw = first_word(&s);

  println!("First word from {s} is: {fw}");

  let s_stack = "hello world";

  let fw_stack = first_word_generic(s_stack);
  let fw_heap = first_word_generic(&s);

  println!("Because &str is more general that &String (deref coercions- not yet covered), we can calculate the first word with stack ({fw_stack}) or heap ({fw_heap})");

  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];

  assert_eq!(slice, &[2, 3]);
  println!("The slice of array 'a' we chose is: {:?}", slice);
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn first_word_generic(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}