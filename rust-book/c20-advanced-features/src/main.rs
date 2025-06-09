mod unsafe_superpowers;

fn main() {
  println!("Advanced features");

  println!("# Unsafe");
  unsafe_superpowers::dereference_raw_pointers();
  unsafe_superpowers::split_at_mut();
  unsafe_superpowers::custom_split_at_mut(&mut [1,2,3,4,5,6,7], 3);
}
