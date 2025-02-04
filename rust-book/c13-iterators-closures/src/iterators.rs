
pub fn iter_next() {
  let v1 = vec![1,2,3];

  println!("Creating a vector: {v1:?}");
  println!("Creating an iterator from it with '.iter()'");

  let sum: i32 = v1.iter().sum();

  println!("After calling .sum() on the iterator, we get: {sum} (and we cannot use the iterator anymore)");
}

pub fn iter_consumption() {
  let shoe_list = vec![
    iter_adapter::Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    iter_adapter::Shoe {
      size: 12,
      style: String::from("sandal"),
    }
  ];

  println!("original shoe list: {shoe_list:?}");

  let shoes_in_size_10 = iter_adapter::shoes_in_size(&shoe_list, 10);
  println!("shoes with size 10: {shoes_in_size_10:?}");
  println!("original shoe list: {shoe_list:?}");

  let shoes_in_size_12 = iter_adapter::shoes_in_size_owning(shoe_list, 12);
  println!("shoes with size 10: {shoes_in_size_12:?}");
  // This doesn't compile: function takes ownership
  // println!("original shoe list: {shoe_list:?}");
}

mod iter_adapter {
  #[derive(PartialEq, Debug, Clone)]
  pub struct Shoe {
      pub size: u32,
      pub style: String,
  }

  // Returns new vector of shoes, filtering only those that match the size
  pub fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.iter().filter(|&s| s.size == shoe_size).map(|s| s.clone()).collect()
  }

  // Returns new vector of shoes, filtering only those that match the size
  // AND: takes ownership of the original vector => will not be able to be used before
  pub fn shoes_in_size_owning(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
  }
}