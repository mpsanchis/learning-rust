use trpl::StreamExt;
use std::time::Duration;
use std::pin::pin;
use super::get_messages::get_messages;

pub fn read_msgs_from_stream_with_timeout() {
  trpl::run(async {
    let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

    while let Some(result) = messages.next().await {
      match result {
        Ok(message) => println!("{message}"),
        Err(reason) => eprintln!("Problem: {reason:?}"),
      }
    }
  })
}
