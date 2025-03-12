mod mybox;
mod refcount;
mod refcell;

use mybox::mybox_code;
use refcount::ref_count;
use refcell::refcell_usage;


fn main() {
  println!("# Box<T>");
  mybox_code();

  println!("\n# Rc<T>");
  ref_count();

  println!("\n# RefCell<T>");
  refcell_usage();
}

