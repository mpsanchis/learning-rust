struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
// Tuple Struct
struct Point(i8,i8,i8);
struct Color(i8,i8,i8);

fn main() {
  //defining_and_instantiating();
  // print_structs();
  method_syntax();
}

fn method_syntax() {
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
      return (self.height > other.height) && (self.width > other.width);
    }
    fn square(size: u32) -> Self {
      return Self {
        width: size,
        height: size
      };
    }
  }

  let rect1 = Rectangle { width: 30, height: 50};
  
  println!("The area of the rectangle is {} square pixels.", rect1.area());
  
  let rect2 = Rectangle { width: 20, height: 100};
  println!("Rectangle 1 {} hold Rectangle 2", if rect1.can_hold(&rect2) { "can" } else  { "cannot" });

  let sq = Rectangle::square(4);
  println!("The area of sq is: {}", sq.area())
}

fn print_structs() {
  #[derive(Debug)]
  struct Student {
    name: String,
    age: u8
  }

  let s = Student { name: String::from("Miguel"), age: 30 };
  println!("The student found is {:?}", s);

  // Using the dbg! macro:
  dbg!(s);
  // This cannot be done, as dbg! took ownership
  // --> println!("The name of s is: {}", s.name);
  let s2 = Student {
    name: String::from("Mario"),
    age: dbg!( 2 * 25 )
  };
  dbg!(&s2);
}

fn defining_and_instantiating() {
  // Struct creation and initialization
  let u1 = build_user("foo@bar.com", "foo");
  println!("Found user1 with name '{}' and email '{}'", u1.username, u1.email);
  
  let mut u2 = User {
    username: String::from("bar"),
    active: false,
    ..u1 // always at the end
  };
    
  u2.email = String::from("foo@foo.com");
  println!("Found user2 with name '{}' and email '{}'", u2.username, u2.email);
  // This line would not work, because u2 took ownership of u1.email (String does not implement the Copy trait)
  // println!("Now user1 has name '{}' and email '{}'", u1.username, u1.email);

  // Struct tuples
  let _black = Color(0,0,0);
  let origin = Point(0,0,0);
  // Note that origin and black have the same value but are of different types
  // --> This cannot be done: origin = Color(1,2,3);
  // Access values with their position
  println!("The first coordinate of the origin is: {}", origin.0);
}

fn build_user(email: &str, username: &str) -> User {
  User {
      active: true,
      username: String::from(username),
      email: String::from(email),
      sign_in_count: 1,
  }
}