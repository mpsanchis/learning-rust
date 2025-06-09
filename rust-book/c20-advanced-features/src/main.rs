mod unsafe_superpowers;

fn main() {
  println!("Advanced features");

  println!("# Unsafe");
  unsafe_superpowers::dereference_raw_pointers();
}
