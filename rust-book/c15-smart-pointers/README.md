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
