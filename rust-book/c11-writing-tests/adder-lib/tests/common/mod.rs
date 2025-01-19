
pub mod common_assertions {
  pub fn assert_geq(a: usize, b: usize) {
    if a < b {
      panic!("{} is not >= {}", a, b);
    }
  }
}
