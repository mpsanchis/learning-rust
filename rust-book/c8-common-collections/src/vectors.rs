
fn access_and_modify_vector() {
  println!("\n### Accessing and modifying a vector");
  let mut v = vec![0,1,2];
  let mut p0 = v[0];
  println!("Working with vector: {:?}", v);
  println!("Its first position, accessed with p0 = v[0], is: {p0}"); // 0
  p0 = 42;
  println!("After p0={}, the vector remains v: {:?}", p0, v); // [0,1,2]

  let p0 = &mut v[0]; // recycle var name
  *p0 = 42;
  println!("If p0 = &mut v[0] and then (*p0 = 42), we get a new v: {:?}", v); // [42,1,2]
}

pub fn do_not_respect_borrow_rules() {
  println!("Uncomment 'do_not_respect_borrow_rules' to show how compiler complains if vector modified while there is a borrow pointing to it");
  /* 
  let mut v = vec![1, 2, 3, 4, 5];
  
  let first = &v[0]; // immutable borrow
  
  v.push(6); // mutable borrow
  
  println!("The first element is: {first}"); // use of immutable borrow again
  */
}

pub fn update_vectors() {
  println!("### Updating vectors");
  fn add_33_to_vector(v: &mut Vec<i32>) {
    // Note the &mut notation (chapter 4) to declare that this function modifies the argument
    v.push(33);
  }

  let mut v1: Vec<i32> = Vec::new();
  let v2 = vec![1,2,3];

  // OBS: v2.push(42) doesn't work => vector is not declared as *mut*able

  println!("v1 (created with Vec::new()) originally is: {:?}", v1);
  add_33_to_vector(&mut v1);
  println!("v1 after v1.push(33) is: {:?}", v1);

  // Note the type difference between direct access with &v[index] (&i32) and v.get(index) (Option<&i32>)
  println!("v2 (created with vec![1,2,3]) originally is: {v2:?}");
  let first = &v2[0];
  let fifty_fifth = v2.get(55);
  let fifty_fifth_as_string = match fifty_fifth {
    None => String::from("not defined"),
    Some(val) => val.to_string()
  };

  println!("In v2 ({:?}), the first element is: {first} and the 55th is {fifty_fifth_as_string}", v2);

  // OBS: this would panick ==> out of bounds
  // let twentieth = v2[20];
  // println!("v2[20]: {twentieth}");
  access_and_modify_vector();
}

pub fn iterate_vectors() {
  println!("\n### Iterating vectors");
  
  let mut v = vec![1,2,3];
  println!("When starting for loop, v={v:?}");
  for i in &mut v {
   *i += 1;
   // v.push(5); => this would not compile, since it implies two mutable borrows interfering with each other
  }
  println!("When the loop is done, v={v:?}");
}