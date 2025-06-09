use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Counter {
  num: i32
}

pub fn associated_types() {
  println!("\n## Associated types");

  struct Counter2 {
    num: i32
  }

  trait MyIteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
  }

  trait MyIteratorAssociated {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
  }

  // generics
  impl MyIteratorGeneric<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
      Option::Some(42)
    }
  }
  impl MyIteratorGeneric<String> for Counter {
    fn next(&mut self) -> Option<String> {
      Option::Some(String::from("42"))
    }
  }

  let mut c = Counter { num: 0 };
  let n: Option<i32> = c.next();
  println!("Getting a number using generic types: {}", n.unwrap());

  // associated type
  impl MyIteratorAssociated for Counter2 {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
      Option::Some(42)
    }
  }

  let mut c = Counter2 { num: 0 };
  let n = c.next(); // no type annotation, no multiple implementations
  println!("Getting a number using associated types: {}", n.unwrap());
}

pub fn default_value_generics() {
  println!("\n## Implementing a trait MyAdd<Rhs=Self> that allows to .add(other)");

  trait MyAdd<Rhs=Self> {
    fn add(&self, other: &Rhs) -> Self;
  }

  impl MyAdd for Counter {
    fn add(&self, other: &Self) -> Self {
        Counter {
          num: self.num + other.num
        }
    }
  }

  let c1 = Counter { num: 0 };
  let c2 = Counter { num: 42 };

  let c3 = c1.add(&c2);
  println!("We can 'impl MyAdd for Counter' without needing extra verbosity 'impl MyAdd<Self> for Counter', and get: {c3:?} ");
}

pub fn method_disambiguation() {
  trait Pilot {
    fn fly(&self);
  }

  trait Wizard {
    fn fly(&self);
  }

  struct Human;

  impl Pilot for Human {
    fn fly(&self) {
      println!("This is your captain speaking.");
    }
  }

  impl Wizard for Human {
    fn fly(&self) {
      println!("Flying on a broom...");
    }
  }

  impl Human {
    fn fly(&self) {
      println!("*waving arms furiously*");
    }
  }

  println!("\n## Multiple methods with the same name:");
  let h = Human;
  h.fly();
  Pilot::fly(&h);
  Wizard::fly(&h);

  trait Animal {
    fn baby_name() -> String;
  }

  struct Dog;

  impl Dog {
    fn baby_name() -> String {
      String::from("Spot")
    }
  }

  impl Animal for Dog {
    fn baby_name() -> String {
      String::from("puppy")
    }
  }

  println!("\n## Trait functions without &self (not methods) need further disambiguation:");
  println!("Dog::baby_name() = {}", Dog::baby_name());
  println!("<Dog as Animal>::baby_name() = {}", <Dog as Animal>::baby_name());
}

pub fn supertraits() {
  trait StarDisplay: std::fmt::Display {
    fn star_representation(&self) -> String {
      format!("***\n{}\n***", self.to_string())
    }
  }

  struct Person {
    name: String,
    age: u8
  }

  impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} ({})", self.name, self.age)
    }
  }

  impl StarDisplay for Person {}

  println!("\n## Using a supertrait to define a trait (trait StarDisplay: std::fmt::Display)");
  let p = Person { name: String::from("James"), age: 42 };
  println!("p.star_representation():\n{}", p.star_representation());
}

pub fn newtype_pattern() {
  println!("\n## Using the newtype pattern to wrap a Vec<String>");
  struct WrappedVec(Vec<String>);

  impl std::fmt::Display for WrappedVec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "[{}]", self.0.join(", "))
    }
  }

  impl Deref for WrappedVec {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }
  impl DerefMut for WrappedVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.0
    }
  }

  let mut v = WrappedVec(vec![String::from("hello"), String::from("world")]);

  println!("New wrapped vector v = {v}");
  v.push(String::from("!!"));
  println!("WrappedVec(Vec<String>) implements Deref and DerefMut, so v can be modified");
  println!("Modified wrapped vector v = {v}");
}