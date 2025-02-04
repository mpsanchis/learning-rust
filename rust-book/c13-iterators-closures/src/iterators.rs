
pub fn iter_next() {
  let v1 = vec![1,2,3];

  println!("Creating a vector: {v1:?}");
  println!("Creating an iterator from it with '.iter()'");

  let sum: i32 = v1.iter().sum();

  println!("After calling .sum() on the iterator, we get: {sum}");
}