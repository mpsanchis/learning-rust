mod summary;
mod lifetimes;

use summary::{Article, Displayable, Summary, Tweet};
use lifetimes::{longest, return_first_and_log_second, MyStruct};

fn main() {
  implement_trait();
  trait_bound_syntax();
  blanket_implementation();
  variable_lifetimes();
}

fn implement_trait() {
  let my_first_tweet = Tweet{
    username: "@me".to_string(),
    content: String::from("Hello world"),
    retweeted: 0
  };

  println!("My tweet, summarized: {}", my_first_tweet.summarize());

  let other_tweet = summary::Tweet{
    username: String::from("@other"),
    content: String::from("bla bla"),
    retweeted: 10
  };

  summary::log_same_type_items(&my_first_tweet, &other_tweet);
  //summary::log_same_items(&my_tweet, item2);
}

fn trait_bound_syntax() {
  let my_article = Article {
    author: "Smith, John".to_string(),
    content: String::from("Once upon a time..."),
    publication_year: 2024
  };
  let my_second_tweet = Tweet {
    username: String::from("@me"),
    content: String::from("Now it is called X"),
    retweeted: 1
  };

  // Note: this doesn't work: we force both to be of same type <T>
  //summary::log_same_type_items(&my_article, &my_second_tweet);
  // However, this works: arguments only need to implement the Summary trait
  summary::log_any_two_items(&my_article, &my_second_tweet);
}

fn blanket_implementation() {
  println!("## Blanket implementation (display_me function)");
  let my_third_tweet = Tweet {
    username: String::from("@me"),
    content: String::from("Another tweet example"),
    retweeted: 1
  };
  println!("Display of a tweet:\n{}", my_third_tweet.display_me());
}

fn variable_lifetimes() {
  println!("# Lifetimes");

  let s1 = String::from("veryverylongstring");
  let longest_string: &str;
  let first_string: &str;
  {
    let s2 = String::from("shortstring");
    longest_string = longest(&s1, &s2);
    first_string = return_first_and_log_second(&s1, &s2);
  }
  // Does not compile: longest_string has same
  // println!("Longest string: {longest_string}");
  // This does work: 'first_string' has the lifetime of 's1'
  println!("First string: {first_string}");

  let some_str = "some_str";
  let mut some_int: i32 = 32;

  let my_struct = MyStruct {
    borrowed_str: some_str,
    borrowed_int: &some_int
  };

  println!("my_struct: {}", my_struct.stringify());
  some_int = 33;
  // Using my_struct after modifying one of its attributes does not compile: lifetime of my_struct depends on that of its attributes
  // println!("my_struct: {}", my_struct.stringify());
}