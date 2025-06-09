# Advanced features

This chapter covers features that are not so used, but appear every now and then. They are:
- Unsafe rust
- Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
- Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types
- Advanced functions and closures: function pointers and returning closures
- Macros: ways to define code that defines more code at compile time

## Unsafe rust

Unsafe rust exists because static analysis is conservative. Rust will only compile if it can guarantee the code upholds some guarantees. In some cases, the code might be safe, but the compiler can't know it, so it does not compile.

Another reason for it to exist is that certain tasks related to the hardware, or interactions with the OS are inherently unsafe.

### Unsafe superpowers

An `unsafe` block allows 5 extra actions (*superpowers*) that aren't allowed by the compiler in the rest of code:
1. Dereferencing a raw pointer
2. Calling an unsafe function or method
3. Accessing or modifying a mutable static value
4. Implementing an unsafe trait
5. Accessing fields of a `union`

Note that `unsafe` doesn't turn off the borrow checker, neither does it disable any other safety check. It only gives you access to those 5 actions.

### Dereferencing a raw pointer

Unsafe rust has two new types called *raw pointers* that are similar to references. As with references, raw pointers can be immutable or mutable and are written as:
- `*const T`, and
- `*mut T`

Note the asterisk `*` isn't the dereference operator: it is part of the type name. In the context of raw pointers, *immutable* means that the pointer can't be directly assigned to after being referenced.

Different from references and smart pointers, raw pointers:
- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers, or multiple mutable pointers to the same location
- Aren't guaranteed to point to valid memory
- Are allowed to be null
- Don't implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can achieve greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply.

Two ways of creating raw pointers are:
1. By using the *raw* borrow operators
```Rust
let mut num = 5;

let r1 = &raw const num; // *const i32
let r2 = &raw mut num; // *mut i32
```

2. By casting (with `as`):
```Rust
let address = 0x012345usize;
let r = address as *const i32;
```

Creating raw pointers does not require `unsafe` rust, only dereferencing them:
```Rust
let mut num = 5;

let r1 = &raw const num;
let r2 = &raw mut num;

unsafe {
  println!("r1 is: {}", *r1);
  println!("r2 is: {}", *r2);
}
```

Note that these two pointers would not be allowed with references or smart pointers: a mutable (`*mut i32`) and an immutable (`*const i32`) pointer are both pointing to the same place in memory, which is forbidden by the borrow checker.