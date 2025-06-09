static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

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

    println!("\n## Creating r3 raw pointer to a newly created String variable with value 'hello world!' and reserving 100 additional Bytes of capacity...");
    let mut hello = String::from("hello world!");
    hello.reserve(100);

    let r3 = &raw const hello;
    unsafe {
      println!("r3 points @ {r3:?} which contains value: {}", *r3);
      println!("&hello returns address: {:p} (in the stack, because it is a local variable)", &hello);
      println!("hello.as_ptr() returns address: {:p} (in the heap, becaue as_ptr returns pointer to content and String lives in heap)", hello.as_ptr());
    }

    println!("\n## Creating r4 by casting r3 to be *const usize instead of *const String...");
    let r4 = r3 as *const usize;
    unsafe {
      println!("r4 points @ {r4:?} which contains value: {}", *r4);
      println!("next memory location ({:p}) contains value: {:#x}", r4.byte_add(8), *r4.byte_add(8));
      println!("next next memory location ({:p}) contains value: {}", r4.byte_add(16), *r4.byte_add(16));
    }
    println!("At the time of coding this, rustc 1.86.0 for macos stored Strings in memory with layout: (capacity, pointer to heap, length)");
}

pub fn split_at_mut() {
  let mut v = vec![1,2,3,4,5,6];
  let slice = &mut v[..];

  let (a,b) = slice.split_at_mut(3);
  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);
}

pub fn custom_split_at_mut(values: &mut [i32], mid: usize) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    let (a, b): (&mut [i32], &mut [i32]);

    unsafe {
      (a, b) =  (
        std::slice::from_raw_parts_mut(ptr, mid),
        std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
      );
    }

    assert_eq!(a.len(), mid);

    for i in 0..mid {
      assert_eq!(a.get(i), Option::Some(&values[i]));
    }
    assert_eq!(b.len(), len - mid);
    for j in 0..(len - mid) {
      assert_eq!(b.get(j), Option::Some(&values[mid + j]));
    }
}

unsafe extern "C" {
  safe fn abs(input: i32) -> i32;
}

pub fn foreign_function() {
  println!("\n## Calling a foreign function");
  println!("The rust compiler knows where to find C's stdlib in your OS...");
  println!("According to C, abs(-3) = {}", abs(-3));
}

pub fn static_variables() {
  println!("\n## Accessing and modifying static variables");
  println!("HELLO_WORLD = {HELLO_WORLD}");
  unsafe {
    println!("at start, COUNTER = {}", *(&raw const COUNTER));
    COUNTER += 42;
    println!("after inc, COUNTER = {}", *(&raw const COUNTER));
  }
}
