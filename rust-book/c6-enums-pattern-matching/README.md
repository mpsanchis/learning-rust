# Enums and Pattern Matching

## 6.1 Defining an enum
Enums can be just a list of possible values:
```rust
enum IpAddrKind {
  V4,
  V6,
}
```

but also a list of object-like values:
```rust
enum Message {
  Quit, // no data
  Move { x: i32, y: i32 }, // like a struct, but inline
  Jump(JumpMessage), // a struct
  Write(String), // string
  ChangeColor(i32, i32, i32), // tuple
}

struct JumpMessage {
  how_high: u8,
  how_far: u8
}
```

Sub-types can be accessed with the `::` syntax. Functions can for instance use `match` to deal with the different values of the enum:
```rust
fn route(ip_kind: IpAddrKind) {
  match ip_kind {
    IpAddrKind::V4 => println!("Routing a v4 IP address"),
    IpAddrKind::V6 => println!("Routing a v6 IP address")
  }
}
```

Enum objects can be instantiated with the `::` syntax as well:
```rust
  let move_back = Message::Move { x: -1, y: 0 };
  let jump_up = Message::Jump(JumpMessage { how_high: 100, how_far: 0 });
```

Methods can also be defined for enums, as done with structs:
```rust
impl Message {
  fn to_string(&self) -> String {
    match self {
      Self::Quit => String::from("Quit"),
      Self::Move { x, y } => format!("({x}, {y})"),
      // ...
    }
  }
}
```

### Option enum

Docs: https://doc.rust-lang.org/std/option/enum.Option.html

There is no `null` type in Rust. Instead, the `Option<T>` enum is defined in the standard library, and used often:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

Since it is such a basic enum, there is syntactic sugar if wanted. If `let s = String::from("hello")`, creating an option can be done as usual:
```rust
let maybe_a_string = Option::Some(s);
```

but also:
```rust
let maybe_a_string = Some(s);
```

The same applies to `None`. However, we need to explicitly declare the type here:
```rust
let not_a_string: Option<String> = None;
```

## 6.2 Match control flow

Already used throughout this and past chapters of the book, but here defined with a bit more detail.

Its basic syntax is the following:
```rust
match variable {
  pattern1 => code1, // note the comma
  pattern2 => code2,
  ...
  pattern_n => {
    // curly braces if more code
    instructions_with_semicolon();
  }
}
```

We can de-struct the variable being matched when matching in the branch:
```rust
match student {
  Student(degree) => {
    // do something with the degree
  }
  // ...
}
```

a typical use case of the previous feature is when working with options:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
```

Note that matches must cover all possibilities of the variable being matched (all types it could have). Otherwise rust won't compile. If we want to omit cases, we can use the `other` keyword (if we want to use its value), or the `_` (if we don't want to use it). An example of this is working with numbers (we cannot cover all cases):

```rust
let dice: u8 = roll_dice();
match dice {
  6 => println!("You have won");
  3 => println!("Play again");
  other => end_game_with_code(other);
}
```