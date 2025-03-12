use std::cell::RefCell;
use std::rc::{Rc, Weak};

mod list;
use list::{List::Cons as Cons, List::Nil as Nil};

mod node;
use node::Node;

pub fn memleak_examples() {
  println!("## Memleaks example with a cons list");
  memleak_cons_list();

  println!("\n##Memleaks example with a tree");
  memleak_tree();
}

fn memleak_cons_list() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("Created cons-list a = (5, Nil)");

  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("Created cons-list b = (10, a)");
  println!("Rc count now:\n\ta: {}\n\tb: {}", Rc::strong_count(&a), Rc::strong_count(&b));

  if let Some(link) = a.tail() {
    println!("Modifying a to be a = (5, b)");
    *link.borrow_mut() = Rc::clone(&b);
    println!("Now: \n\ta = (5, b)\n\tb = (10, a)");
  }

  println!("Rc count now:\n\ta: {}\n\tb: {}", Rc::strong_count(&a), Rc::strong_count(&b));

  println!("⚠️ We have created a cycle, and we cannot travel the lists anymore (e.g. printing them leads to a stack overflow)");
  // Uncomment the next line to see that we have a cycle: it will overflow the stack
  // println!("a next item = {:?}", a.tail());
}

fn print_node_info(node_name: &str, rc: &Rc<Node>) {
  println!(
    "{node_name}'s data:\n\tvalue (id):\t\t\t\t{}\n\tparent:\t\t\t\t\t{:?}\n\tstrong rc (owners):\t\t\t{}\n\tweak rc (children pointing to me):\t{}",
    rc.value,
    rc.parent.borrow().upgrade(),
    Rc::strong_count(&rc),
    Rc::weak_count(&rc),
  );
}

fn memleak_tree() {
  println!("Creating a tree with\nbranch (id: 0) -> leaf (id: 1) (Rc), but also\nleaf -> branch (Weak)");
  let leaf = Rc::new(Node {
    value: 0,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  print_node_info("leaf", &leaf);

  {
    let branch = Rc::new(Node {
      value: 1,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("modifying leaf.parent to weakly point at 'branch', with a mutable borrow: *leaf.parent.borrow_mut()");
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    print_node_info("leaf", &leaf);
    print_node_info("branch", &branch);

    println!("dropping branch...");
  }

  print_node_info("leaf", &leaf);
}