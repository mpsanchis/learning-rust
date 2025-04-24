use trpl::{Stream, StreamExt, ReceiverStream};
use std::time::Duration;
use std::pin::pin;
use super::get_messages::get_messages;

fn get_intervals() -> impl Stream<Item = u32> {
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
    let mut count = 0;
    loop {
      trpl::sleep(Duration::from_millis(1)).await;
      count += 1;

      if let Err(send_error) = tx.send(count) {
        eprintln!("Could not send interval {count}: {send_error}");
        break;
      }
    }
  });

  ReceiverStream::new(rx)
    .take(30)
}

pub fn read_msgs_from_composed_stream() {
  println!("### Reading messages from a merged stream (number stream and letters stream)");
  trpl::run(async {
    let messages = get_messages().timeout(Duration::from_millis(100));
    let intervals = get_intervals()
      .map(|cnt| format!("Interval: {cnt}"))
      .throttle(Duration::from_millis(100)) // stream gets polled less often
      .timeout(Duration::from_secs(10)); // too long so that it doesn't timeout

    let mut merged = pin!(messages.merge(intervals));

    while let Some(result) = merged.next().await {
      match result {
        Ok(message) => println!("{message}"),
        Err(reason) => eprintln!("Problem: {reason:?}"),
      }
    }
  })
}