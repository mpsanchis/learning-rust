use std::time::Duration;

pub fn simple_channel() {
  trpl::run(async {
    let (tx, mut rx) = trpl::channel();
    let val = String::from("hi");
    println!("Sending: {val}");
    tx.send(val).unwrap();
    let received = rx.recv().await.unwrap();
    println!("Got: {received}");
  });
}

pub fn sending_msgs_with_delay() {
  println!("\n### Sending 'hi from the future' through a channel from a future, and awaiting the messages from main");
  println!("\n### Note: first messages are sent to the channel, and then they are polled (not what we'd like...)");
  trpl::run(async {
    let (tx, mut rx) = trpl::channel();
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("future"),
    ];
    for val in vals {
      tx.send(val).unwrap();
      trpl::sleep(Duration::from_millis(500)).await;
    }
    while let Some(value) = rx.recv().await {
      println!("received '{value}'");
    }
  });
}

pub fn sending_msgs_with_delay_fixed() {
  trpl::run(async {
    let (tx, mut rx) = trpl::channel();
    let tx_fut = async move {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("future"),
    ];
      for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
    } };
    let rx_fut = async {
      while let Some(value) = rx.recv().await {
        println!("received '{value}'");
      }
    };
    trpl::join(tx_fut, rx_fut).await;
  });
}