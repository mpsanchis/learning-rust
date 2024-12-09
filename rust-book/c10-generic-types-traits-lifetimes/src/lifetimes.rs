pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn return_first_and_log_second<'a>(x: &'a str, y: & str) -> &'a str {
  println!("Second string passed to return_first_and_log_second: '{y}'");
  x
}

pub struct MyStruct<'a,'b> {
  pub borrowed_str: &'a str,
  pub borrowed_int: &'b i32
}

impl<'a,'b> MyStruct<'a,'b> {
  pub fn stringify(&self) -> String {
    return format!("String: {} - Int: {}", self.borrowed_str, self.borrowed_int);
  }
}