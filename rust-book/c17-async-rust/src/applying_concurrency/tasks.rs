use std::time::Duration;

pub fn join_two_futures() {
  println!("### Creating fut1 (3 prints) and fut2 (2 prints) and joining them");
  trpl::run(async {
    let fut1 = async {
      for i in 0..3 {
        println!("hi number {i} from the 1st task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    let fut2 = async {
      for i in 0..2 {
        println!("hi number {i} from the 2nd task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    trpl::join(fut1, fut2).await;
  });
}

pub fn await_two_futures() {
  println!("### Creating fut1 (3 prints) and fut2 (2 prints) and joining them");
  trpl::run(async {
    let _fut1 = async {
      for i in 0..3 {
        println!("hi number {i} from the 1st task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    }.await;
    let _fut2 = async {
      for i in 0..2 {
        println!("hi number {i} from the 2nd task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    }.await;
    // trpl::join(fut1, fut2).await; // => doesn't compile, because once awaited, futures aren't futures anymore
  });
}

pub fn await_fut1_after_loop2() {
  println!("### Creating fut1 (3 prints) and fut2 (2 prints) and joining them");
  trpl::run(async {
    let fut1 = async {
      for i in 0..3 {
        println!("hi number {i} from the 1st task!");
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };

    for i in 0..2 {
      println!("hi number {i} from the 2nd task!");
      trpl::sleep(Duration::from_millis(500)).await;
    }

    fut1.await;
  });
}