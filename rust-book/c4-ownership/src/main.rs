fn main() {
  {
    let _s = "hello"; // this is a string literal. Lives in the stack (immutable)
    
    let mut s1 = String::from(_s); // this is a String (capital S): lives in the heap
    
    s1.push_str(", world!");
    println!("{s1}");

    let s2 = s1;

    // println!("{s}"); --> doesn't work, because "s2" took ownership
    
    println!("{s2}");

    let mut s3 = s2.clone();
    s3 = s3.replace("!", " from s3!");

    println!("Now I can print both s2 ({s2}) and s3 ({s3})");

  } // rust will drop variables from this block {} because they'll be out of scope

  let s = String::from("hello");  // s comes into scope
  let s_cp = s.clone();

  takes_ownership(s); // s's value moves into the function... and so is no longer valid after here

  //println!("after takes_ownership, s's value is: {s}"); //this doesn't work

  let p = takes_and_gives_back(s_cp);

  println!("P's value is: {p}");

  let x = 5; // x comes into scope

  // x would move into the function but i32 is Copy, so it's okay to still use x afterward
  makes_copy(x);

  // Pass as a reference, so that function does not take ownership:
  let l: usize = calculate_length(&p);
  println!("I should be able to print both p ({p}) and its length ({l}) if I have passed it as a reference");
  
  // Mutate a variable inside a function by using a reference
  let mut p_mut = p.clone();
  println!("A clone of 'p' is: {p_mut}");
  mutate_string_without_taking_ownership(&mut p_mut);
  println!("After changing, p_mut is: {p_mut}");
}

fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("some_string has value: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("some_integer has value: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
  return a_string;  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
  return s.len();
}

fn mutate_string_without_taking_ownership(s: &mut String) {
  s.push_str("_something_appended");
}
