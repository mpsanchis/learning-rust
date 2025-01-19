use adder_lib::add; // Note: dashes not allowed (transform to snake_case)

mod common;
use common::common_assertions;

#[test]
fn adds_small_numbers() {
  let result = add(2, 2);
  common_assertions::assert_geq(result, 4);
}
