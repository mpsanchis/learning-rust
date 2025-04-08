use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn send_many<T>(tx: mpsc::Sender<T>, vec: Vec<T>, sleep_time: Option<Duration>) {
  for item in vec {
    tx.send(item).unwrap();
    if sleep_time.is_some() {
      thread::sleep(sleep_time.unwrap());
    }
  }
}

pub fn simple_msg_passing() {
  let (tx, rx) = mpsc::channel();

  println!("Creating a thread that sends a message using the transmitter from std::sync::mpsc::channel()");
  thread::spawn(move || {
    let val = String::from("hi from child thread");
    tx.send(val).unwrap(); // own the tx
  });

  let rx_msg = rx.recv().unwrap();
  println!("In 'main' I received the message: '{rx_msg}'");
}

pub fn multiple_sent_messages() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    send_many(
      tx,
      vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ],
      Option::Some(Duration::from_secs(1))
    );
  });

  for received in rx {
    println!("Main thread got: {received}");
  }
}

pub fn multiple_transmitters_multiple_messages() {
  struct ThreadMessage<'a> {
    tx_name: &'a str,
    msg: &'a str,
  }

  let (tx, rx) = mpsc::channel();

  let tx1 = tx.clone();
  thread::spawn(move || {
    send_many(tx1,
      vec![
        ThreadMessage { tx_name: "tx1", msg: "hello"},
        ThreadMessage { tx_name: "tx1", msg: "from"},
        ThreadMessage { tx_name: "tx1", msg: "tx1"},
      ],
      Some(Duration::from_secs(1))
    );
  });

  thread::spawn(move || {
    send_many(tx,
      vec![
        ThreadMessage { tx_name: "tx2", msg: "more"},
        ThreadMessage { tx_name: "tx2", msg: "messages"},
        ThreadMessage { tx_name: "tx2", msg: "for"},
        ThreadMessage { tx_name: "tx2", msg: "you"},
      ],
      Some(Duration::from_secs(1))
    );
  });

  for ThreadMessage {tx_name, msg} in rx {
    println!("Got: '{msg}' from {tx_name}");
  }
}