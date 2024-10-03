mod vectors;
mod strings;

fn main() {
  println!("# Common collections code!");

  println!("\n## Vectors");
  vectors::update_vectors();

  vectors::do_not_respect_borrow_rules();

  vectors::iterate_vectors();

  println!("\n## Strings");
  strings::create_strings();

  strings::update_strings();

  strings::access_string_indices();
}



