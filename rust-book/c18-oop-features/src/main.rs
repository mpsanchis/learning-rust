mod trait_objects;
mod state_pattern;

fn main() {
  println!("# Trait objects");
  trait_objects::gui_example();

  println!("# State pattern");
  println!("## State pattern OOP style");
  state_pattern::blog_post_oop();

  println!("## State pattern Rust style");
  state_pattern::blog_post_rust();
}
