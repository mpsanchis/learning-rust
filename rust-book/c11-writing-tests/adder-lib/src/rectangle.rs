
#[derive(Debug)]
pub struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn new(width: u32, height: u32) -> Rectangle {
    if width > 100 || height > 100 {
      panic!("Width and height must be smaller than 100, but were:  w: {width}, h: {height}");
    }
    return Rectangle {
      width,
      height
    }
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}


#[cfg(test)]
mod rectangle_tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    // given
    let larger = Rectangle {
      width: 8,
      height: 7
    };
    let smaller = Rectangle {
      width: 5,
      height:1
    };

    // when-then
    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  #[should_panic(expected = "must be smaller than 100")]
  fn new_panics_if_rectangle_bigger_than_100() {
    let _too_big_rectangle = Rectangle::new(150, 30);
  }
}