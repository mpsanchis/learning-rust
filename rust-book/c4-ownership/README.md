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
