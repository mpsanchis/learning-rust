# Structs

## Definition and instantiation

Define a struct in this way (as a data interface):
```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}
```

Instantiate it using its name:
```rust
let u = User {
  active: true,
  username: String::from("bob"),
  email: String::from("bob@bob.com"),
  sign_in_count: 1
}
```

Use the `struct update syntax` if you want to copy values from another struct. This will move ownership of the values that don't implement the `Copy` trait:
```rust
let u2 = User {
  username: String::from("alice"),
  ..u1
}
```
In this example, u2 takes ownership of u1's email. This means that `u1.email` cannot be used after u2 is created.

Tuple structs can be created: no naming of the attributes
```rust
struct Color(i8, i8, i8)
```

Empty structs can also be created (use will be discussed in further chapters):
```rust
struct AlwaysEqual;
```

## Debugging and printing

In order to print, `println!({}, obj)` expects objects `obj` that implement `std::fmt::Display`. 
We can also print with `println!({:?}, obj)`, which will expect the `Debug` trait.

We can tell the rust compiler to assume a default Debug trait by adding a header to our struct:
```rust
#[derive(Debug)]
struct User {
  name: String,
  age: u8
}
```

We can also use the `dbg!` macro, which:
- takes ownership of the value passed
- prints the file and line number of where it occurs in the code along with the resultant value of that expression
- returns ownership of the value