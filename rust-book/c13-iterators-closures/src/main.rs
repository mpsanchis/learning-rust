mod closures;
mod iterators;

fn main() {
  println!("# C13 Iterators and Closures");
  println!("## Closures");
  closures();
  println!("## Iterators");
  iterators();
}

fn closures() {
  closures::closures_general();
  println!("### Ownership in closures");
  println!("#### Immutable borrows (Fn)");
  closures::ownership::immutable();
  println!("#### Mutable borrows (FnMut)");
  closures::ownership::mutable();
  println!("#### Changing ownership (FnOnce)");
  closures::ownership::once();
}

fn iterators() {
  println!("### Creating iterators");
  iterators::iter_next();
  println!("### Consuming iterators");
  iterators::iter_consumption();
}


