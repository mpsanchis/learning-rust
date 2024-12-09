
pub trait Summary {
  fn summarize_type(&self) -> &str;

  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more {}s from {}...)", self.summarize_type(), self.summarize_author())
  }
}

pub trait Displayable {
  fn display_me(&self) -> String;
}

impl<T: Summary> Displayable for T {
  fn display_me(&self) -> String {
    format!(">>> DISPLAY\n{}\n<<<", self.summarize())
  }
}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub retweeted: i64
}

impl Summary for Tweet {
  fn summarize_type(&self) -> &str {
    "tweet"
  }

  fn summarize_author(&self) -> String {
    self.username.clone()
  }

  fn summarize(&self) -> String {
    format!("{}\n- By: {}", self.content, self.username)
  }
}

pub struct Article {
  pub author: String,
  pub content: String,
  pub publication_year: u64
}

impl Summary for Article {
  fn summarize_author(&self) -> String {
    self.author.clone()
  }

  fn summarize_type(&self) -> &str {
    "Article"
  }
}

pub fn log_same_type_items<T: Summary>(item1: &T, item2: &T) {
  println!("** Logging items:\nItem 1:\n{}\nItem 2:\n{}\n**", item1.summarize(), item2.summarize());
}

pub fn log_any_two_items(item1: &impl Summary, item2: &impl Summary) {
  println!("** Logging items:\nItem 1: {}\nItem 2: {}\n**", item1.summarize(), item2.summarize());
}