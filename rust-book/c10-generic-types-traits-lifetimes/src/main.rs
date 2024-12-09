mod summary;

use summary::{Article, Tweet, Summary};

fn main() {
  implement_trait();
  trait_bound_syntax();
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