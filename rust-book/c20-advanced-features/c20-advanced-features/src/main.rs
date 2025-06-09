mod unsafe_superpowers;
mod advanced_traits;
mod advanced_functions;
mod macros;

fn main() {
  println!("Advanced features");

  println!("\n# Unsafe");
  unsafe_superpowers::dereference_raw_pointers();
  unsafe_superpowers::split_at_mut();
  unsafe_superpowers::custom_split_at_mut(&mut [1,2,3,4,5,6,7], 3);
  unsafe_superpowers::foreign_function();
  unsafe_superpowers::static_variables();

  println!("\n# Advanced traits");
  advanced_traits::associated_types();
  advanced_traits::default_value_generics();
  advanced_traits::method_disambiguation();
  advanced_traits::supertraits();
  advanced_traits::newtype_pattern();

  println!("\n# Advanced functions");
  advanced_functions::function_pointers();
  advanced_functions::returning_closures();

  println!("\n# Macros");
  macros::declarative_macros();
  macros::custom_derive_macro();
  macros::attribute_macro();
}
