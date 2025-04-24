use std::time::Duration;
use trpl::{Stream, ReceiverStream};

pub fn get_messages() -> impl Stream<Item = String> {
  let (tx, rx) = trpl::channel();

  trpl::spawn_task(async move {
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for (index, message) in messages.into_iter().enumerate() {
      let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
      trpl::sleep(Duration::from_millis(time_to_sleep)).await;

      if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
        eprintln!("Could not send message '{message}': {send_error}");
        break;
      }
    }
  });

  ReceiverStream::new(rx)
}