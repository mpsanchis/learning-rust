mod back_of_house {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  pub enum Appetizer {
    Soup,
    Salad,
    FreeAppetizerOnTheHouse
  }

  #[derive(Debug)]
  pub struct Client {
    pub name: String,
    pub phone: String
  }
}

mod customer;
mod front_of_house;

use crate::back_of_house::Appetizer;

pub fn eat_at_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");
  // Change public attributes
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");

  // Enums' values are public if the enum is "pub"
  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}

// Re-export: allows external users of the restaurant to call Restaurant::book_at_restaurant
// Without having to know about our internal code organisation (Restaurant::customer::book_at_restaurant)
pub use customer::book_at_restaurant;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    book_at_restaurant("foo", "bar");
}