mod rectangle;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

// ######## TESTS FROM HERE ON #########

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  #[ignore]
  fn failing_test() {
    panic!("Make this test fail");
  }
}
