
pub fn dereference_raw_pointers() {
  println!("## Creating r1, r2 raw pointers to a newly created i32 variable with value 5...");
  let mut num = 5;

  let r1 = &raw const num;
  let r2 = &raw mut num;

  unsafe {
    println!("r1 points @ {r1:?} which contains value: {}", *r1);
    println!("r2 points @ {r2:?} which contains value: {}", *r2);
    println!("num variable has address &num: {:p}", &num);
  }

    println!("\n## Creating r3 raw pointer to a newly created String variable with value 'hello world!'...");
    let hello = String::from("hello world!");

    let r3 = &raw const hello;
    unsafe {
      println!("r3 points @ {r3:?} which contains value: {}", *r3);
      println!("&hello returns address: {:p}", &hello);
      println!("hello.as_ptr() returns address: {:p}", hello.as_ptr());
    }

    println!("\n## Creating r4 by casting r3 to be *const usize instead of *const String...");
    let r4 = r3 as *const usize;
    unsafe {
      println!("r4 points @ {r4:?} which contains value: {}", *r4);
      println!("previous memory location ({:p}) contains value: {}", r4.byte_sub(8), *r4.byte_sub(8));
      println!("next memory location ({:p}) contains value: {:#x}", r4.byte_add(8), *r4.byte_add(8));
      println!("next next memory location ({:p}) contains value: {}", r4.byte_add(16), *r4.byte_add(16));
    }
}