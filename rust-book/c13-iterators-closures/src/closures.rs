#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
  Red,
  Blue,
}

pub struct Inventory {
  pub shirts: Vec<ShirtColor>,
}

impl Inventory {
  pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  pub fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

pub fn closures_general() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
  };

  let user_pref1 = Some(ShirtColor::Red);
  let giveaway1 = store.giveaway(user_pref1);
  println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

  let user_pref2 = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
}

pub mod ownership {
  pub fn immutable() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Acts as an immutable pointer to the list: we can use other immutable borrows as many times as we want
    let borrows_immutable = || println!("During closure: {list:?}");

    println!("Before calling closure: {list:?}");
    borrows_immutable();
    println!("After calling closure: {list:?}");
  }

  pub fn mutable() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Acts as a mutable pointer to `list`: no other borrows between pointer declaration and usage can be used (because they'd get unexpected values)
    // Equivalent with variables: let list_ptr = &mut list;
    let mut borrows_and_changes = || {
      list.push(4);
      println!("No other borrows allowed during a mutable closure!");
    };

    // Between `let mut <closure>` and `closure()`, we cannot have any prints, or other functions that need a pointer to the object
    // println!("After calling closure: {list:?}"); // ====> this does not compile

    borrows_and_changes();

    println!("After calling closure: {list:?}");
  }

  pub fn once() {
    let mut list = vec![];
    let value = String::from("hello world");

    let fn_once = || {
      list.push(value);
      println!("List: {list:?}");
    };

    println!("If closure moves variables out of their environment, it becomes an FnOnce...");
    fn_once();
  }
}

