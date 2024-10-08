# Ownership

## Basic ownership rules
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Move

If an assignment is made to a previous pointer (let ptr2 = ptr1), then the new pointer will take ownership of the data.
This means that the original pointer can't be used.

If a variable is used in a function, the function takes the ownership of the variable, which cannot be used in the main code anymore.

## Copy trait

Variables that live in the stack (such as ints), implement a Copy trait. This makes assignments behave like a "deep copy" instead of like a "Move".
Doing something like `let x = y` does not leave `y` out of scope if it is an int. Types that implement `Copy` are the basic ones:
- ints such as `u32`
- `bool`
- floating points such as `f64`
- `char`
- tuples (if all their types implement `Copy`)

## References

Passing by reference requires defining the reference both when passing it and in the function declaration:
```rust
...
let len = calculate_length(&s1)
...
fn calculate_length(s: &String) -> usize { ... }
```

The functions that receive a reference don't take ownership of the variable. This means the variable can be used after calling the function.
Also, if the reference is not declared as mutable, the value will not be able to be modified in the function.

### Mutable references

Declaring a mutable reference allows us to modify the value of an object or variable:
```rust
...
let mut s = String::from("hello");
change_str(&mut s);
...
fn change(s: &mut String) { ... }
```

Restrictions: 
- only one mutable reference to a variable can exist at a time
- if there is a mutable reference, it can only be used after all the immutable references have used, or in another scope
  - Obs: the scope can be thought as either (1) what is defined with brackets, or (2) from declaration since last use

Example 1 (Compiles): immutable references are in scope, but no longer used => mutable reference can be used
```rust
let mut s = String::from("hello");
let p1 = &s;
let p2 = &s;
println!("p1: {p1}, p2: {p2}");
let p3 = &mut s;
println!("p3: {p3}");
```

Example 2 (does not compile): immutable references are in scope, and are used later on
```rust
let mut s = String::from("hello");
let p1 = &s;
let p2 = &s;
println!("p1: {p1}, p2: {p2}");
let p3 = &mut s;
println!("p3: {p3}");
println!("p1: {p1}, p2: {p2}");
```

### Dangling references

In Rust, a function cannot return a pointer to an object it the heap it created, because the owner of the object will be dropped when the function ends. 

Example (doesn't compile):
```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
  let s = String::from("hello");
  return &s;
}
```

A pointer is returned, but there must be one owner of the String. If function ends, owner (s) will be dropped. The solution is to return the owner to the object in the heap.

Example (compiles):
```rust
fn main() {
  let a_proper_string = no_dangle();
}

fn no_dangle() -> String {
  let s = String::from("hello");
  return s;
}
```

## Slices

Lists such as arrays and strings can be "sliced" and get a pointer to a section of them:
```rust
let my_string = String::from("hello world");
let hello: &str = &my_string[0..6];
```

String slices are of type `&str`. If a function wants to find something in a string, it can return `&str`, which will be an immutable pointer to the string. This means that if some instruction after the function call modifies the string before the function result has been used, the Rust compiler will complain: the immutable `&str` reference must be used before some mutable reference to the string modifies the string.

Note that for function calls, `&str` is a superset of `&String`. Declaring `&str` as a parameter type is more general than `&String`.

For arrays, the functioning is similar:
```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];
```