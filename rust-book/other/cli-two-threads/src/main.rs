mod word_printer;

use std::process;
use std::str::FromStr;
use std::thread;
use std::io::{self};
use std::sync::mpsc;
use std::time::Duration;
use word_printer::{WordPrinter, FirstLetter};

enum MyMessage {
  Letter(FirstLetter),
  Cancel(),
}

fn main() {
  println!("");

  let (tx, rx) = mpsc::channel();

  thread_loop(rx);
  main_loop(tx);

}

fn thread_loop(rx: mpsc::Receiver<MyMessage>) {
  thread::spawn(move || {
    let mut word_printer = WordPrinter::new(FirstLetter::A);
    let mut keep_running = true;
    while keep_running {
      let msg_or_err = rx.try_recv();
      match msg_or_err {
        Ok(msg) => {
          match msg {
            MyMessage::Cancel() => {
              println!("closing thread");
              keep_running = false;
            },
            MyMessage::Letter(first_letter) => {
              word_printer.change_first_letter(first_letter);
            }
          }
        },
        Err(e) => {
          match e {
            mpsc::TryRecvError::Empty => {
              word_printer.print_word();
              thread::sleep(Duration::from_millis(500));
            },
            mpsc::TryRecvError::Disconnected => {
              println!("closing thread");
              keep_running = false;
            }
          }
        }
      }
    }
  });
}

fn main_loop(tx: mpsc::Sender<MyMessage>) {
  loop {
    let mut buf = String::new();
    let line_read = io::stdin().read_line(&mut buf);
    if line_read.is_err() {
      tx.send(MyMessage::Cancel()).unwrap();
      eprintln!("Could not read line! Error: {}", line_read.unwrap_err());
      process::exit(1);
    }

    let trimmed_buf = buf.trim();
    let letter = FirstLetter::from_str(trimmed_buf);
    match letter {
      Ok(first_letter) => {
        tx.send(MyMessage::Letter(first_letter)).unwrap();
      },
      Err(_) => {
        println!("Received '{}'. Stopping program...", trimmed_buf);
        tx.send(MyMessage::Cancel()).unwrap();
        thread::sleep(Duration::from_secs(1));
        process::exit(0);
      }
    }
  }
}
