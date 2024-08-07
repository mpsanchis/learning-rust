
fn main() {
  println!("---- Usage and types ----");
  usage_and_types();
  println!("---- Enum with data ----");
  enum_with_data();
  println!("---- Option<T> enum ----");
  option_enum();
  println!("---- Match control flow ----");
  match_control_flow();
}

// Enum without data
enum IpAddrKind {
  V4,
  V6,
}

fn usage_and_types() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  fn route(ip_kind: IpAddrKind) {
    match ip_kind {
      IpAddrKind::V4 => println!("Routing a v4 IP address"),
      IpAddrKind::V6 => println!("Routing a v6 IP address")
    }
  }
  route(four);
  route(six);
}

// Enum with data: each value can have different data (or none at all)
enum Message {
  Quit, // no data
  Move { x: i32, y: i32 }, // like a struct, but inline
  Jump(JumpMessage), // a struct
  Write(String), // string
  ChangeColor(i32, i32, i32), // tuple
}

// Adding functions to enums: the same way as with structs (similar to inheritance)
impl Message {
  fn to_string(&self) -> String {
    match self {
      Self::Quit => String::from("Quit"),
      Self::Move { x, y } => format!("({x}, {y})"),
      Self::Jump(JumpMessage { how_high, how_far }) => format!("(h: {how_high}, f: {how_far})"),
      Self::Write(s) => String::from(s),
      Self::ChangeColor(x, y, z) => format!("({x}, {y}, {z})")
    }
  }
}

struct JumpMessage {
  how_high: u8,
  how_far: u8
}

fn enum_with_data() {
  let move_back = Message::Move { x: -1, y: 0 };
  let jump_up = Message::Jump(JumpMessage { how_high: 100, how_far: 0 });

  fn print_message(msg: Message) {
    let msg_str = msg.to_string();
    match msg {
      Message::Move { x, y } => {
        println!("A message has been received with instruction to move to: {msg_str}");
      }
      Message::Jump(JumpMessage { how_high, how_far }) => {
        println!("A message has been received with instruction to jump to: {msg_str}");
      }
      _ => println!("Some other message has been received")
    }
  }

  print_message(move_back);
  print_message(jump_up);
}

fn option_enum() {
  let maybe_text = maybe_a_string("test");
  println!("test ->");
  print_if_string(maybe_text);
  
  let maybe_text_2 = maybe_a_string("secret");
  println!("secret ->");
  print_if_string(maybe_text_2);
}

fn print_if_string(s: Option<String>) {
  match s {
    Some(text) => println!("Text found: {text}"),
    None => println!("No text found")
  }
}

fn maybe_a_string(s: &str) -> Option<String> {
  if s == "secret" {
    return Some(String::from("You have discovered the secret"));
  }
  return None;
}

// Examples of "match" control flow

#[derive(Debug)] // so we can print using {state:?}
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("Quarter from state: {state:?}");
      return 25;
    }
  }
}

fn match_control_flow() {
  let penny_value = value_in_cents(Coin::Penny);
  println!("The value of a penny is: {penny_value}");
  let quarter_value = value_in_cents(Coin::Quarter(UsState::Alabama));
  println!("The value of a quarter is: {quarter_value}");
  let some_one: Option<u8> = Some(penny_value);
  let some_two = plus_one(some_one);

  if some_two.is_some() {
    let two: u8 = some_two.unwrap();
    println!("The value of some_two is: {two}");
  }
  let some_none = plus_one(None);
  println!("some_none contains just: {some_none:?}");
}

fn plus_one(x: Option<u8>) -> Option<u8> {
  match x {
      None => None,
      Some(i) => Some(i + 1),
  }
}