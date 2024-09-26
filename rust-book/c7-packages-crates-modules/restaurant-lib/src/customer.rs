// Make "Client" available without having to reference its path
use crate::back_of_house::Client;

pub fn book_at_restaurant(name: &str, phone: &str) {
  let client = Client {
    name: String::from(name),
    phone: String::from(phone)
  };
  println!("Client {:?} wants to book a table", client);
  // Appetizer is out of scope because it's declared at root, not in-module
  let free_appetizer = crate::Appetizer::FreeAppetizerOnTheHouse;
}