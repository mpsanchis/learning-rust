fn main() {
  let mut counter = 0;

  let result = loop {
      counter += 1;

      if counter == 10 {
          break counter * 2;
      }
  };

  println!("The result of the main function is {result}");

  foo();

  for_loop();

  fibo();
}

fn foo() {
  let mut count = 0;
  'counting_up: loop { // Adding a label to a loop
      println!("count = {count}");
      let mut remaining = 10;

      loop {
          println!("remaining = {remaining}");
          if remaining == 9 {
              break;
          }
          if count == 2 {
              break 'counting_up;
          }
          remaining -= 1;
      }

      count += 1;
  }
  println!("End count in foo = {count}");
}

fn for_loop() {
    let arr = [5,4,3,2,1];

    for v in arr {
        println!("{v}...")
    }
    println!("liftoff!")
}

fn fibo() {
    for n in 0..7 {
        let fibon = fibo_n(n);
        println!("fibo({n}): {fibon}")
    }
}

fn fibo_n(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }
    return fibo_n(n-1) + fibo_n(n-2);
}