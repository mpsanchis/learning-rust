use mybox::mybox_code;
use refcount::ref_count;

mod mybox;
mod refcount;


fn main() {
  println!("# Box<T>");
  mybox_code();

  println!("# Rc<T>");
  ref_count();
}

