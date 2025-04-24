use std::time::Duration;
use std::pin::Pin;

pub fn sending_msgs_three_futures() {
  trpl::run(async {
    let (tx, mut rx) = trpl::channel();
    let tx2 = tx.clone();
    let tx1_fut = async move {
      let vals = vec![
        (1, String::from("hi")),
        (1, String::from("from")),
        (1, String::from("the")),
        (1, String::from("future")),
      ];
      for val in vals {
        tx.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    let tx2_fut = async move {
      let vals = vec![
        (2, String::from("hi")),
        (2, String::from("from")),
        (2, String::from("the")),
        (2, String::from("future")),
      ];
      for val in vals {
        tx2.send(val).unwrap();
        trpl::sleep(Duration::from_millis(500)).await;
      }
    };
    let rx_fut = async {
      while let Some((tx_id, msg)) = rx.recv().await {
        println!("received '{msg}' from tx{tx_id}");
      }
    };
    let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx2_fut)];
    trpl::join_all(futures).await;
  });
}