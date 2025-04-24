use std::time::Duration;
use std::thread;
use trpl::Either;

async fn timeout<F: Future>(
  future: F,
  timeout: Duration
) -> Result<F::Output, Duration> {
  match trpl::race(future, trpl::sleep(timeout)).await {
    Either::Left(output) => Ok(output),
    Either::Right(_) => Err(timeout)
  }
}

fn slow_task_no_yield(name: &str, ms: u64) {
  thread::sleep(Duration::from_millis(ms));
  println!("'{name}' ran for {ms}ms");
}

pub fn race_two_futures() {
  trpl::run(async {
    let fut_slow = async {
      println!("Slow future started");
      trpl::sleep(Duration::from_millis(100)).await;
      println!("Slow future finished");
    };

    let fut_fast = async {
      println!("Fast future started");
      trpl::sleep(Duration::from_millis(50)).await;
      println!("Fast future finshed");
    };

    trpl::race(fut_slow, fut_fast).await;
  });
}

fn dont_yield_control_to_runtime() {
  println!("\n### Running 50ms+100ms+50ms in two futures, without a voluntary yield");
  trpl::run(async {
    let a = async {
      println!("Future 'a' started");
      slow_task_no_yield("a1", 50);
      slow_task_no_yield("a2", 100);
      slow_task_no_yield("a3", 50);
      println!("Slow future finished");
    };

    let b = async {
      println!("Future 'b' started");
      slow_task_no_yield("b1", 50);
      slow_task_no_yield("b2", 100);
      slow_task_no_yield("b3", 50);
      println!("Fast future finshed");
    };

    trpl::join(a, b).await;
  });
}

fn yield_control_to_runtime() {
  println!("\n### Running 50ms+100ms+50ms in two futures, yielding to the runtime after every sub-task");
  trpl::run(async {
    let a = async {
      println!("Future 'a' started");
      slow_task_no_yield("a1", 50);
      trpl::yield_now().await;
      slow_task_no_yield("a2", 100);
      trpl::yield_now().await;
      slow_task_no_yield("a3", 50);
      println!("Slow future finished");
    };

    let b = async {
      println!("Future 'b' started");
      slow_task_no_yield("b1", 50);
      trpl::yield_now().await;
      slow_task_no_yield("b2", 100);
      trpl::yield_now().await;
      slow_task_no_yield("b3", 50);
      println!("Fast future finshed");
    };

    trpl::join(a, b).await;
  });
}

pub fn yielding() {
  dont_yield_control_to_runtime();
  yield_control_to_runtime();
}

pub fn own_abstractions() {
  println!("\n### Building an own 'timeout' abstraction that executes a future and kills it if it lasts longer than the timeout");
  trpl::run(async {
    let slow_task = async {
      println!("Starting slow task...");
      trpl::sleep(Duration::from_millis(5000)).await;
      "Finished slow task!"
    };
    match timeout(slow_task, Duration::from_millis(10)).await {
      Err(max_time) => println!("Slow task was slower than {:?} and did not finish", max_time),
      Ok(msg) => println!("Slow task finished and returned: {msg}")
    }
  })
}

