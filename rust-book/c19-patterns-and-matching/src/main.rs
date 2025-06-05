enum Message {
    Hello { id: i32 },
}

fn main() {
  let msg = Message::Hello { id: 13 };

  match msg {
    Message::Hello { id: id_msg @ 3..=7 } => println!("Found an id ({id_msg}) in range [3,7]"),
    Message::Hello { id: id_msg } if (8..=9).contains(&id_msg) => println!("Found an id ({id_msg}) in range [8,9]"),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in range [10, 12]")
    }
    Message::Hello { id } => println!("Found some other id: {id}"),
  }
}
