mod mybox;
mod refcount;
mod refcell;
mod refcyle_memleaks;

use mybox::mybox_code;
use refcount::ref_count;
use refcell::refcell_usage;
use refcyle_memleaks::memleak_examples;


fn main() {
  println!("# Box<T>");
  mybox_code();

  println!("\n# Rc<T>");
  ref_count();

  println!("\n# RefCell<T>");
  refcell_usage();

  println!("\n# Mem leaks");
  memleak_examples();
}

