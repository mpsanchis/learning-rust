# Closures

Def: anonymous functions that can be saved in a variable or passed as argument to another function. Unlike functions, closures can capture values from the scope in which they're defined.

For instance, in this snippet:
```rust
impl MyStruct {
  pub fn foo(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }
```

there is an anonymous function (closure/lambda): `|| self.most_stocked()` that takes no argument and returns `self.most_stocked()`. It can see elements in its scope (`self`).

## Type annotation

Closures don’t usually require you to annotate the types of the parameters or the return value like `fn` functions do. Typically the compiler can infer types on parameters and return type. As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary:
```rust
let expensive_closure = |num: u32| -> u32 {
  println!("calculating slowly...");
  thread::sleep(Duration::from_secs(2));
  num
};
```

These 4 syntaxes are equivalent (one function and three closures):
```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Type inference works similarly to that of a `Vec::new()`:
- _either_ we annotate the type of the elements stored in the Vec
- _or_ we make a first insertion, and from then on all other insertions must be of the same type

In the case of closures, the first call to the closure will define its types (arguments and return).

## Ownership

Closures that use objects in their scope have an implicit relationship with these objects. As function parameters, this ownership relationship can be one of:
1. Borrowing immutably (as: `let imm_borr = &my_object`)
2. Borrowing mutably (as: `let mut_borr = &mut my_object`)
3. Taking ownership (as: `let new_own = my_object`)

If the closure needs to mutate objects of its scope, it must be declared with the keyword `mut` as well:
```rust
let mut my_list = vec![1,2,3];

let mut closure_mut = || {
  my_list.push(4);
};
```
otherwise, the compiler will throw an error:
```
calling `<closure>` requires mutable binding due to mutable borrow of `<object>`
```

If the closure needs to own variables in its scope, the keyword `move` must be added before the parameter list. This is useful in some cases, such as when spawning a new thread to do a task, and giving it the ownership of variables:
```rust
use std::thread;

fn main() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {list:?}");

  thread::spawn(move || println!("From thread: {list:?}"))
    .join()
    .unwrap();
}
```

## Fn traits

The way a closure captures and handles values from the environment affects which `Fn` traits the closure implements. Closures will automatically implement one, two, or three of the `Fn` traits, in an additive fashion, depending on how the closure's body handles values:
* `FnOnce`: applies to closures that can be called once. All closures implement at least this trait. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits. Example: `let move_and_return_x = move || x;`.
* `FnMut`: applies to closures that don’t move captured values out of their body, but that might mutate the captured values. Example: `let mutates_x = x.append(4)`.
* `Fn`: applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.

Examples of `Fn` traits:

**Example 1**
The `Option.unwrap_or_else` function takes a `FnOnce` closure as parameter:
```rust
impl<T> Option<T> {
  pub fn unwrap_or_else<F>(self, f: F) -> T
  where
    F: FnOnce() -> T
  {
    match self {
      Some(x) => x,
      None => f(),
    }
  }
}
```

In this case, `F` must be able to:
- be called once
- take no arguments, and
- return a T
Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is only going to call `f` at most one time. This is the most generic closure requirement.

**Example 2**
The `sort_by_key` function of `Vec` accepts an `FnMut`:
```rust
pub fn sort_by_key<K, F>(&mut self, f: F)
where
    F: FnMut(&T) -> K,
    K: Ord,
```

The reason is that the closure might need to be called several times.

If we passed a function such as:
```rust
let fn_once = |o: &MyObject| {
  some_list.push(some_value);
  return o.some_attribute;
}
```

the compiler would identify that `some_value` is moved out of its environment. This is because the `push` function takes ownership of the value passed to it (`pub fn push(&mut self, value: T)`). Therefore, `fn_once` is changing the ownership of the value of `some_value` into `some_list`, and thus it can only be called once. This means that the compiler inferes that `fn_once` is of type `FnOnce`, and the compiler throws:
```
^^^^^^^^^^^^^^^ this closure implements `FnOnce`, not `FnMut`
----- closure is `FnOnce` because it moves the variable `some_value` out of its environment
```

# Iterators

The `.iter()` method, defined on `Vec<T>`, returns an iterator that can be used to loop through the elements of a vector:
```rust
let list_iter = list.iter();

for val in list_iter {
  // do something with val
}
```

This is the most common case, but it's just a specific implementation of the `Iterator` trait:
```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // methods with default implementations elided
}
```

This definition uses the syntax `type Item` and `Self::Item`, not yet explained in this book. They define a type *associated* to the trait.
The `Iterator` trait only requires implementors to define one method (`next`), which returns:
- one item of the iterator at a time wrapped in `Some` and
- when iteration is over, returns `None`

## Consuming the iterator

Note that an iterator needs to be declared as `mut` in order to use its `next()` method, since each call to it changes its internal state. It is not needed when using the iterator in a `for` loop, because the loop takes ownership of the iterator, and makes it mutable for us.

Methods that call `next` are called *consuming adapters*, because calling them uses up the iterator. An example is the `sum()` method, that takes ownership of the iterator, iterates it, and sums its elements:
```rust
let v1 = vec![1, 2, 3];
let iter = v1.iter();
let total = iter.sum();
assert_eq!(total, 6);
```

Note: we aren't allowed to use `iter` after the call to `sum()`, because `sum` takes ownership of the iterator we call on it. Note the difference in the `self` parameter between:
* `fn next(&mut self) -> Option<I::Item>`, and
* `fn sum<S>(self) -> S`

## Producing other iterators

*Iterator adapters* are methods defined on the `Iterator` trait that don't consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

A common example are maps:
```rust
let v1: Vec<i32> = vec![1, 2, 3];
let shift_iter = v1.iter().map(|x| x + 1);
```

Note that iterator are *lazy*, so creating a new iterator has no effect until methods that consume the iterator are called. Another method that consumes the iterator is `collect`, which uses the iterator and puts its items into a collection data type:
```rust
let v2: Vec<_> = shift_iter.collect();
assert_eq!(v2, vec![2, 3, 4]);
```

## Iterators using Closures: capturing the environment

Many iterator adapters take closures as arguments, and often times these closures capture their environment.
An example of this, where the closure uses the value `shoe_size`, which is not defined inside the closure:
```rust
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

# A note about iterator performance

Iterators get compiled to practically the same code as loops, making them a *zero-cost abstraction*. This makes them more worthwhile, since they provide an abstraction (that leads to simpler code), but keep the performance.