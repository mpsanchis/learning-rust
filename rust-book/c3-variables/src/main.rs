fn main() {

  // booleans
  let t = true;
  let _f: bool = false; // with explicit type annotation

  // shadowing
  let x = 25;
  if t == true {
    let x = 26;
    println!("In the if condition, x = {x}");
  }
  println!("Outside of the if condition, x = {x}");

  // characters: all Unicode
  let _heart_eyed_cat = '😻';

  // tuples (fixed length, different types)
  let tup: (i32, f32, u8) = (500, 6.6, 1);
  let (_a, b, _c) = tup;
  println!("The first element of the tuple is {0} and the second is {1}", tup.0, b);

  // arrays (fixed length, same types)
  let arr = [1, 2, 3, 4, 5];
  
}
