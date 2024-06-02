# Ownership

## Basic ownership rules
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Move

If an assignment is made to a previous pointer (let ptr2 = ptr1), then the new pointer will take ownership of the data.
This means that the original pointer can't be used.

## Copy trait

Variables that live in the stack (such as ints), implement a Copy trait. This makes assignments behave like a "deep copy" instead of like a "Move".
Doing something like `let x = y` does not leave `y` out of scope if it is an int. Types that implement `Copy` are the basic ones:
- ints such as `u32`
- `bool`
- floating points such as `f64`
- `char`
- tuples (if all their types implement `Copy`)

