# Introduction to smart pointers

A *pointer* is a variable that contains an address in memory, which "points to" some other data. The most common pointer in Rust is a *reference* (`&`), which borrows the value it points to. They don't have any special capabilities other than referring to data, and have no overhead.

A *smart pointer*, on the other hand, is a data structure that:
* Acts as a pointer (stores an address that "points to" some other data)
* Has additional metadata
* Has additional capabilities

Some examples seen in previous chapter are:
* `String`
* `Vec<T>`
These types count as smart pointers because they own some memory and allow you to manipulate it. They also have metadata and extra capabilities or guarantees. For instance, `String` stores its capacity as metadata, and has the extra ability to ensure its data will always be valid UTF-8.

Smart pointers originated in C++, and exist in other languages. To explore the general concept, this chapter will look at some examples:
* `Box<T>`, to allocate values on the heap
* `Rc<T>`, a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces borrowing rules at runtime instead of compile time.

Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the traits:
* `Deref`, to allow an instance of the smart pointer struct to behave like a reference, so you can write your code to work with either references or smart pointers.
* `Drop`, to allow you to customize the code that's run when an instance of the smart pointer goes out of scope.
Libraries can design their own smart pointers. The ones covered here are the most common ones in the std lib.

Other topics discussed in this chapter are:
* *Interior mutability*, a pattern where an immutable type exposes an API for mutating an interior value
* *Reference cycles*, a error that can leak memory

# Box<T> to point to data on the heap

This is the most straightforward smart pointer, which allows to store some data on the heap, and keeps in the stack only its pointer (address). They are mostly used in the following situations:
* When you have a type whose size is unknown at compile time, and you want to use a value of that type in a context that requires an exact size
* When you have a large amount of data and you want to transfer ownership, while ensuring the data won't be copied
* When you want to own a value and you care only that it's a type that implements a particular trait, rather than being of a specific type. Note: chapter 18 dives deeper.

A new box can be created with:
```rust
let b = Box::new(5);
```
If we `println!("b = {b}")`, we'll get the number *5* contained in the box. This `i32` value will be allocated on the heap, and when it gets out of scope, rust will deallocate both the value (on the heap) and the Box pointer (on the stack).

## Recursive types with Box<T>

Recursive types are an issue for a compiler which wants to know how much space each type takes up: the types should be of a fixed size, but a recursive type could have any size. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

### Example: cons list

A *cons list* (construction function list) is Lisp's version of a linked list. By calling `cons` (the construction function) on a pair consisting on a value and another pair, we can construct cons lists made up of recursive pairs. For example, a cons list containing 1, 2, 3 could be represented as:
```
(1, (2, (3, Nil)))
```
Each element in a cons list consists of 2 elements:
* Value of the current item
* Next item
The last element of the item contains (Value, Nil), which stops the recursion.

A first approach (that does **NOT** compile) to construct such a type in Rust would be:
```rust
enum List {
  Cons(i32, List),
  Nil
}

let list = Cons(1, Cons(2, Cons(3, Nil)));
```

The error we get if we try to compile such code is:
```
error[E0072]: recursive type `List` has infinite size
1 | enum List {
  | ^^^^^^^^^
2 |     Cons(i32, List),
  |               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +

error[E0391]: cycle detected when computing when `List` needs drop
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^
  |
  = note: ...which immediately requires computing when `List` needs drop again
  = note: cycle used when computing whether `List` needs drop
```

The rust compiler calculates an *infinite* size for this `List` because it is recursively defined: it holds another value of itself directly. As a result, it cannot calculate how much space it needs to store a `List` value.

#### Calculating size of non-recursive enums

The rust compiler goes through all values of an enum, and calculates the one that takes up the most space. For instance, in a `Message` such as:
```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32)
}
```
Rust will calculate: no size, 2 x sizeOf(i32), sizeOf(pointer to string plus metadata), 3 x sizeOf(i32). Because only one variant is used at a time, the most space a `Message` will need is the space of the largest of the 4 variants.

#### Limiting size of recursive enums

The rust compiler mentioned *help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle*. In this suggestion, "indirection" means that, instead of storing a value directly, we should change the data structure to store the value "indirectly" by storing a pointer to the value instead.

Because `Box<T>` is a pointer, Rust knows how much space it needs: a pointer size. Contents of the Box might change, but the pointer has always the address of memory where the data starts (plus potentially some metadata).

Applying the changes suggested by the compiler means changing the type definition and its usage to:
```rust
enum List {
  Cons(i32, Box<List>),
  Nil,
}

let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

The rust compiler will calculate sizes:
* Negligible for `Nil`, since it stores no data
* sizeOf(i32) + sizeOf(Box) for `Cons`

The sizes are finite, so the compiler does not complain anymore.

Boxes provide only indirection and heap allocation, but they don't have any other features. That makes them also simpler, and they add less overhead. `Box<T>` is a smart pointer because it implements the `Deref` trait, which allows itself to be treated as a reference. They also implement the `Drop` trait, which allows its data to be deallocated from the heap when the Box gets out of scope.

# The Deref trait

The `Deref` trait allows you to customize the behaviour of the dereferencing operator: `*`. By implementing `Deref` in a way that a smart pointer can be treated like a regular reference (`&`), you can write code that operates on references, and use that code with smart pointers too.

## Following the pointer to the value

Rust complains if we even try to compile the following code:
```rust
let x = 5;
let y = &x;

assert_eq!(5, y);
```
The error thrown contains: `no implementation for '{integer} == &{integer}'`

This is because `y` is a *reference* to an integer, and in order to get its value we need to dereference it:
```rust
let x = 5;
let y = &x;

assert_eq!(5, *y);
```
A number and a reference are different types.

## Using Box<T> as a reference

With a `Box<T>`, we can do things like:
```rust
let x = 5;
let y = Box::new(x);

assert_eq!(5, *y);
```

This is because `Box<T>` implements the `Deref` trait, so we can get its value by using the `*` operator.

## Defining our own smart pointer

We can define something similar to a `Box`, but that stores its values on the stack (for simplicity). We just need the struct definition, and its associated `impl Deref`:
```rust
use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
  pub fn new(item: T) -> Self {
    MyBox(item)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}
```

This allows us to run code such as:
```rust
let mybox = MyBox::new(5);
assert_eq!(5, *mybox);
```

Note: The `type Target = T;` syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter, which will be covered in more detail in Chapter 20.

The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference. In fact, the code above is translated at compile time to:
```rust
assert_eq!(5, *(mybox.deref()));
```
The reason why the `deref` method must return a reference (and not a value, `T` in this case) is the ownership system: if `deref` had returned `T` directly, the value would be moved out of `self`. In most cases, we don't want to take ownership of the value we are dereferencing.

## Implicit Deref Coercions with Functions and Methods

*Deref coercion* converts a reference to a type that implements the `Deref` trait into a reference to another type. The most common case is converting `&String` to `&str`. This can be done because the `Deref` trait is implemented in the `String` type with a return type of `&str`. Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the `Deref` trait. It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs.

An example of deref coercion is the following code:
```rust
fn hello_print(name: &str) {
  println!("Hello {name}");
}

let world = MyBox::new(String::from("world"));

hello_print(&world);
```

At **compile time**, rust will figure out the following:
* `&world` is of type `&MyBox<String>`
* `MyBox<String>` implements a `deref` method that returns `&String`
* `String` implements a `deref` method that returns `&str`

Therefore:
* `world.deref()` is of type `&String`
* `*(world.deref())` is of type `String`
* `(*(world.deref())).deref()` is of type `&str`

Therefore:
* `hello_print(&world)` can be coerced to
* `hello_print((*(world.deref())).deref())`

If deref coercion did not exist, we would have to transform the types ourselves. In this example, we could write the (way more complicated and error-prone) code:
```rust
hello_print(&(*mybox)[..]);
```

This does:
1. dereferences the box into a `String` with `*`
2. gets a reference (`&`) to a slice (`[]`) of the string, that is equal to all its length (`..`)

## Deref coercion and mutability

Similarly to using `Deref` to obtain a reference to a value, there is the trait `DerefMut`, which allows to override the `*` operator on mutable references. This means that the following code does not compile, unless `MyBox` implements the `DerefMut` trait:
```rust
let mybox = MyBox::new(String::from("world"));
*mybox = String::from("42");
```

The compiler throws the following error:
```
help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `MyBox<String>`
```

We would need the followng code for it to compile:
```rust
impl<T> DerefMut for MyBox<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
```

Rust does deref coercion when it finds types and trait implementations in three cases:
1. From `&T` to `&U` when `T: Deref<Target=U>`. In other words, when `T` implements a `deref` function that returns `&U`. This is the case of `String`, which dereferences to `&str`.
2. From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`. Same as above, but with mutability.
3. From `&mut T` to `&U` when `T: Deref<Target=U>`. In other words, when passing a mutable reference to a function that takes an immutable one, Rust will coerce the value.

Note: there is not a 4th case (the opposite of 3), to preserve intentionality. If a variable cannot be mutably borrowed, it cannot be passed to a function that needs a mutable borrow. Because of the borrowing rules, a mutable reference must be the only reference to the data. Converting a mutable reference to immutable won't break the borrowing rules. However, if an immutable reference was converted to mutable, it would require that the initial immutable reference is the only reference to the data, but the borrowing rules don't guarantee that.

In code, rule 3 means that this works:
```rust
fn foo(input: &str);

let mut mybox = MyBox::new(String::from("hello"));
foo(&mybox)
```

# The Drop trait

The `Drop` trait is added to the code by the compiler once a value that implements it goes out of scope. It can be implemented with:
```rust
impl Drop for MyType {
  fn drop(&mut self) { ... }
}
```

## Dropping a value early with std::mem::drop

In some cases, a value can be cleaned before it goes out of scope (e.g.: smart pointers that manage locks, which could be dropped to release the lock). For those cases, Rust does not let you call the `drop()` method manually. The compiler crashes with:
```
explicit destructor calls not allowed
help: consider using `drop` function
```

The compiler error suggests you to call `std::mem::drop` provided by the stdlib. Rust does not let you call the destructor because it will be added again by the compiler when the value goes out of scope. This would call a *double free* error.
The `drop(myObject)` function, however, will prevent the double free error.

# Rc<T>, the reference-counted smart pointer

There are cases where a single value has multiple owners. For instance, in graph data structures, many edges "own" the node they point to. This means a node shouldn't be cleaned up unless it doesn't have any edges pointing to it.

In cases where we know which part of our program will be using the data last, we can make that part own the data, and the other parts are just references. The borrow checker supports this. However, if *at compile time* we don't know which part of the program will be the last one using the data, we can leverage `Rc<T>`.

Note: `Rc<T>` is only for use in single-threaded code. Chapter 16 discusses concurrency and reference counting in that case.

## Using Rc<T> to share data

A usage example of `Rc<T>` is two cons lists extending a third cons list, as so:
```
b: 3
    \
    a: 5 - 10 - Nil
    /
c: 4
```
Like branches in a git tree, these lists consists of:
- a: (5,10)
- b: (3,5,10)
- c: (4,5,10)

The following snippet attempts to encode it, but does **NOT** compile due to the list `a` being moved to `b` once `b` is created:
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a)); // 'a' moved here
    let c = Cons(4, Box::new(a)); // 'a' cannot be used here, because it's inside 'b'
}
```

We could change the definition of `Cons` to hold references, but that would mean specifying lifetime parameters. We could specify that every element in the list lives as long as the entire list, which is true in this case but not in all scenarios. Moreover, all elements would have to be explicitly declared in code (all elements in cons lists), so that the rust compiler knows how long they live. Declaring them in a one-liner results in an error `temporary value dropped while borrowed`. The following code compiles, but became unmanageable:
```rust
#[derive(Debug)]
enum List<'a> {
  Cons(i32, &'a Box<&'a List<'a>>),
  Nil,
}
// ...
let end_of_list = Box::new(&Nil);
let ten = Cons(10, &end_of_list);
let ten_boxed = Box::new(&ten);
let a = Cons(5, &ten_boxed);
let a_boxed = Box::new(&a);
let b = Cons(3, &a_boxed);
let c = Cons(4, &a_boxed);
println!("{c:?}");
```

A cleaner alternative with `Rc<T>` consists of changing the `List` enum, and having `Cons` contain a reference counter instead of a (simpler) *Box* counter:
```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

Calling `Rc::clone(my_rc: &Rc)` creates a new reference to the same object that `my_rc` is pointing to, and increases the number of counters to that object. The data in the heap will not be cleaned up until there are zero references to it. Note this is not a deep copy, unlike some other `my_object.clone()` calls in code: it only increases the number of counters.

## Cloning an Rc<T> increases the reference count

The function `Rc::strong_count(my_rc: &Rc)` returns the number of references to a certain `Rc<T>`. See code from `refcount` *mod*, which shows how this count increases/decreases as references get created/dropped.

The implementation of the `Drop` trait decreases the reference count automatically when an `Rc<T>` value goes out of scope.

By using **immutable** references, `Rc<T>` allows you to share data between multiple parts of your program for **reading only**. If `Rc<T>` allowed to have multiple mutable references too, there could be data races and inconsistencies.

However, being able to mutate data is often necessary. The next section discusses `RefCell<T>`, which can be used together with `Rc<T>` to work with this immutability restriction.

# RefCell<T> and the 'Interior Mutability' pattern

*Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data. The borrowing rules do not allow this, so this pattern uses `unsafe` code inside a data structure.

## Enforcing borrowing rules at runtime with RefCell<T>

Unlike `Rc<T>`, `RefCell<T>` represents single ownership over the data it holds. However, unlike `Box<T>`, it does not follow the basic borrow rules:
- At any time, you can have either (a) one mutable reference, OR (b) multiple immutable reference (but not both)
- References must always be valid

With `RefCell<T>`, breaking the borrowing rules does not cause compile-time errors, but rather runtime *panic!*. One compile-time error that it `RefCell` throws is if it is being used in a multithreaded context. More about this in chapter 16.

Here's a recap of reasons to choose `Box`, `RefCell` or `Rc`:
- `Rc`:
  - enables multiple owners of the same data
  - allows only immutable borrows, checked at compile time
- `Box`:
  - enables single owner
  - allows immutable or mutable borrows checked at compile time
- `RefCell`:
  - enables single owner
  - allows immutable or mutable borrows checked at runtime (you can mutate the value inside the `RefCell` even when the `RefCell` is immutable)

Mutating the value inside an immutable value is the *interior mutability* pattern.

## Interior Mutability: a mutable borrow to an immutable value

The borrowing rules don't allow to *mutably borrow* a variable that is not declared as `mut`. In other words, this does **NOT** compile:
```rust
let x = 5;
let y = &mut x;
```

However, there are situations in which it's useful for a value to mutate itself in its methods, but appear as immutable to other code. A practical example of this are mock objects: *test doubles* that mimic the behaviour of something, used to observe the behaviour of a system when interacting with another.

Imagine the following situation: an object of ours implements the `Messenger` trait:
```rust
pub trait Messenger {
  fn send(&self, msg: &str);
}
```

Now we want to mock it, and record the messages that it should have sent:
```rust
struct MockMessenger {
  sent_messages: Vec<String>;
}

impl Messenger for MockMessenger {
  fn send(&self, message: &str) {
    self.sent_messages.push(String::from(message));
  }
}
```

The borrow checker does not allow us to compile this code: the `send` function takes an immutable borrow of self (`&self`), but the code of our `MockMessenger` is modifying itself, by editing the attribute `sent_messages`. We cannot edit the function signature, because it comes from a trait (that might even be external to our code).
This is a situation in which interior mutability can help: we can store `sent_messages` in a `RefCell<T>`, and the `send` method will be able to modify it by getting a `borrow_mut` of it:
```rust
struct MockMessenger {
  sent_messages: RefCell<<String>>;
}

impl Messenger for MockMessenger {
  fn send(&self, message: &str) {
    self.sent_messages.borrow_mut().push(String::from(message));
  }
}
```

## Keeping Track of Borrows at Runtime with RefCell<T>

The methods `RefCell::borrow` and `RefCell::borrow_mut` are part of the safe API of `RefCell`, and are the equivalent of using `&` and `&mut` references:
* `borrow` returns the smart pointer type `Ref<T>`, and
* `borrow_mut` returns the smart pointer type `RefMut<T>`
Both types implement `Deref`, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many smart pointers are currently active, by:
* increasing the count when `borrow()` is called, and
* decreasing the count when a `Ref<T>` goes out of scope

Just like the compile-time borrowing rules, `RefCell` lets us have either of:
* many immutable borrows, OR
* one mutable borrow
at any given moment. The difference with compile-time borrowing rules is that `RefCell` will instead `panic!` at runtime and throw a message like:
```
already borrowed: BorrowMutError
```

## Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

A common pattern is to combine `Rc` and `RefCell`, since:
* `Rc<T>` lets you have multiple owners of some data, with immutable access, AND
* `RefCell<T>` lets you mutate without declaring a mutable reference

Therefore:
* `Rc<RefCell<T>>` lets you have multiple owners of some data `T`, and each one can mutate such data

This means that a data structure such as the following:
```rust
struct MyMutableStruct {
  data: Rc<RefCell<i32>>
}
```

can be cloned and each of the clones can borrow mutably the data, and modify it:
```rust
let val = Rc::new(RefCell::new(42));

let ref1 = MyMutableStruct { data: Rc::clone(&val) };
let ref2 = MyMutableStruct { data: Rc::clone(&val) };

// both modify the original data
*(val.borrow_mut()) = 10;
*(ref1.data.borrow_mut()) = 20;
```

Note: we are sacrificing speed (because borrow rules are checked at runtime) for flexibility. This works only for single-threaded programs, though. Chapter 16 will discuss `Mutex<T>` for multi-threading.

# Reference cycles can leak memory

Rust's memory safety makes it difficult, but not impossible, to have memory leaks (i.e., memory that is created but never cleaned up). One easy way to create them is to use `Rc<T>` or `RefCell<T>`, and have items refer to each other in a cycle. This creates a memory leak because the reference count of each item will never be 0 (because the other item is pointing to it), so the item won't be dropped.

## Creating a reference cycle

The code in `refcycle_memleaks` shows a scenario where two variables point at each other. Here's the simplified code:
```rust
enum List {
  Nil,
  Cons(i32, RefCell<Rc<List>>)
}

let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

if let Cons(_, list) = a.as_ref() {
  let mut foo = list.borrow_mut();
  *foo = Rc::clone(&b);
}
```

With this code, we create a situation in which:
```
a --> x (RefCell)
b --> y (RefCell)

x --> y
y --> x
```

We can notice two things:
1. When `a` and `b` are dropped, `x` and `y` (the `RefCell`s) will still hold a count of 1, so they won't be freed.
2. If we attempt to traverse these lists (e.g. by printing them), our program will crash with a stack overflow.

Rust does not offer more tools to prevent these situations: testing and having proper data structures and ownership of data in our code are the best ways to minimise the risks of a memory leak.

## Preventing reference cycles: turning an Rc<T> into a Weak<T>

A new method from `Rc<T>` is presented: `Rc::downgrade`. Similarly to how `Rc::clone` increments the `strong_count` of an `Rc<T>`, `Rc::downgrade` increments the `weak_count` by 1, and returns a smart pointer of type `Weak<T>`.

Some characteristics of `downgrade` and `Weak<T>` are:
* The `weak_count` does not prevent the original `Rc<T>` from being dropped
* This means that `Weak<T>` might not point to the `Rc<T>` it was "cloned" (downgraded) from
* The method `Weak::upgrade` returns an `Option<Rc<T>>`, that will contain `None` if the original `Rc<T>` has been dropped

An example using trees with parent and children nodes is:
* Declaring each of your children as `Rc<Node>`, to have multiple owners (parent, but also each children points to the same data)
* Declaring the vector containing all your children in a `RefCell<Vec>`, to be able to mutate your children
* Declaring your parent as a `Weak<Node>` in case it's deleted, and wrap it in `RefCell<Weak<Node>>` so that it can be mutated.

In rust, we could encode and use this as:
```rust
pub struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

let a = Rc::new(Node {
  value: 0,
  parent: RefCell::new(Weak::new()),
  children: RefCell::new(vec![]),
});

let b = Rc::new(Node {
  value: 1,
  parent: RefCell::new(Weak::new()),
  children: RefCell::new(vec![Rc::clone(&a)]), // Point to children
});

*a.parent.borrow_mut() = Rc::downgrade(&b); // Point to parent
});
```

Note that:
* the first `Node` (0) has two owners: `a` and `b`.
* having a `Weak` reference to the parent breaks the cycle

Now our dependencies look like:
```
a --> x (RefCell)
b --> y (RefCell)

y --> x (RefCell)
x --> y (Weak)
```

# Summary

Smart pointers make different guarantees and trade-offs from those made by default with regular references (`&`). The following topics were discussed:

* `Box<T>` has a know size, and points at data in the heap
* `Rc<T>` keeps track of the number of references to data on the heap, so that the data can have multiple owners
* `RefCell<T>` allows us to mutate an immutable type, and enforces borrowing rules at runtime
* The combination `Rc<RefCell<T>>` allows for many mutating references (can panic)
* The `Deref` trait defines what happens if we use the `&` operator on a smart pointer
* The `Drop` trait defines what happens when data gets out of scope
* Memory leaks can happen with circular pointers, and can be prevented in some cases with `Weak` vs `Rc` (strong) pointers