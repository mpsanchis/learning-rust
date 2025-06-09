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

#### 1. Dereferencing a raw pointer

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

#### 2. Calling an unsafe function or method

An unsafe function indicates that the function has requirements we need to uphold when calling it, that the compiler can't guarantee. Unsafe functions must be called inside an `unsafe` block:
```Rust
unsafe fn dangerous() {}

fn main() {
  unsafe {
    dangerous();
  }
}
```

Not all functions that use `unsafe` blocks are `unsafe`. In fact, many functions wrap `unsafe` blocks and provide a safe abstraction. An example is the `slice::split_at_mut` function defined on mutable slices. It taks one slice and makes it two by splitting the slice at the index given as an argument. It can be used as:
```Rust
// given
let mut v = vec![1, 2, 3, 4, 5, 6];
let r = &mut v[..]; // &mut [i32]
// when
let (a, b) = r.split_at_mut(3); // (&mut [i32], &mut [i32])
// then
assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

This function can't be implemented only with safe Rust. The function returns two **mutable** references to a slice, but the borrow checker only allows to have one mutable reference at a time.

Another use case of calling an unsafe function is using `extern` functions to call external code. The `extern` keyword in Rust facilitates the creation and usage of an FFI (foreign function interface), which is a way for a programming language to define functions and enable different programming languages to call those functions.

The following code runs a function from C's stdlib, `abs`:
```rust
unsafe extern "C" {
  fn abs(input: i32) -> i32;
}
// ...
unsafe {
  println!("Absolute value of -3 according to C: {}", abs(-3));
}
```

The `"C"` defines which ABI (application binary interface) the external function uses. The ABI defines how to call the function at the assembly level. The `"C"` ABI is the most common and follows the C programming language's ABI.

The Rust compiler, via LLVM, will look for the libraries in default directories (e.g. `/usr/lib`), or where specified by flags such as `-C link_args` passed to the compiler when invoked.

Note that we can annotate the function as `safe` explicitly (when in an `unsafe extern` block):
```rust
safe fn abs(input: i32) -> i32;
```

and we won't need to wrap it in an `unsafe` block. It becomes the developer's responsibility to ensure that the function is indeed safe, because the rust compiler cannot check foreign functions.

The opposite (using rust code in other languages) can also be done. In code, we would need to annotate our function with the `#[unsafe(no_mangle)]` macro, so that the compiler doesn't mangle (modify) the function's name:
```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}
```

The library would then need to be compiled as a dynamic library (via *rustc* or *Cargo* options), generating a `*.so` (Linux) / `*.dylib` (macOS) / `*.dll` (Windows) file.

#### 3. Accessing or Modifying a Mutable Static Variable

Global variables are supported in Rust (called *static*), but they can be problematic with ownership rules. If two threads access the same mutable global variable, it can cause a data race.

Static variables can only store references with the `'static` lifetime, which means the Rust compiler can figure out the lifetime and we aren't required to annotate it explicitly.

Differently from constants (`const`, briefly discussed in chapter 3):
1. Static variables have a fixed address in memory. Constants are allowed to duplicate their data whenever they are used.
2. Static variables can be mutable. Accessing and modifying a mutable static variables is *unsafe*, and must be done through raw pointers

```rust
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
  println!("name is: {HELLO_WORLD}");
  unsafe {
    COUNTER += 42;
    println!("COUNTER = {}", COUNTER);
  }
}
```

Note:
* Multithreading can cause data races. Where possible, it is preferrable to use the concurrency techniques discussed in chapter 16.
* By convention, unsafe functions and blocks have a comment starting with `SAFETY` that indicates what the caller needs to do when calling the function, or how the safety rules are upheld.

#### 4. Implementing an Unsafe Trait

A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.

The trait can be declared unsafe and used like this:
```rust
unsafe trait Foo {
  // methods
}

unsafe impl Foo for i32 {
  // method implementation, if needed
}
```

An example of usage is to mark a type as `Send` or `Sync` when it contains another type that isn't marked as `Send` or `Sync` (such as raw pointers). In these cases, we must use `unsafe` and guarantee that these types can be sent across threads or accessed from multiple threads.

#### 5. Accessing Fields of a Union

A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time. They are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

### Using Miri to check unsafe code

Miri is an official rust tool for detecting undefined behaviour. It is a dynamic tool that works at runtime and detects when the code violates the rules it understands about how Rust should work.

Using Miri requires a nightly build of Rust, and can be used by executing:
```shell
rustup +nightly component add miri
cargo +nightly miri run
cargo +nightly miri test
...
```

## Advanced traits

### Associated types

Associated types connect a type placeholder with a trait, such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used. That way, we can define a trait that uses some types without needing to know what those types are until the trait is implemented.

An example of it is the `Iterator` trait from the stdlib:
```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
```

This looks very similar to the version with generic types:
```rust
pub trait Iterator<T> {
  fn next(&mut self) -> Option<T>;
}
```

but there are two subtle and related differences:
1. Generic implementations allow for many re-implementations in the same codebase for the same type (`Iterator<String> for Counter`, `Iterator<i32> for Counter`, etc.), whereas associated types don't.
2. Because of (1.), generic implementations require type annotations when using the trait (e.g.: `let n: Option<i32> = c.next()`), to tell the compiler what function to use. Associated types prevent this.

### Default Generic Type Parameters and Operator Overloading

When using generic type parameters, we can define a default concrete type `<T=DefaultType>` for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default works.

A good example of this is operator overloading, which Rust allows for operations and corresponding traits listed in `std::ops`:

the default type of addition is declared as:
```rust
trait Add<Rhs=Self> {
  type Output;

  fn add(self, rhs: Rhs) -> Self::Output;
}
```

so we can do (for the default type):
```rust
use std::ops::Add;

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
```

or also (if the default is not useful in our use case):
```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}
```

This little feature can be useful in two cases:
1. Allowing customisation, but having a clear default for most users
2. Extending a type with generics, but not breaking existing consumers that don't declare a type (will use default)

### Disambiguating Between Methods with the Same Name

Two traits could implement a method with the same name:
```rust
trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}
```

and a `struct` could also define its own method that collides with the methods from the traits it wants to implement:
```rust
struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}
```

In these cases, we can use the trait's name when calling the method, in order to uniquely identify them:
```rust
let h = Human;
h.fly();
Pilot::fly(&h);
Wizard::fly(&h);
```

However, associated functions that are not methods don't have a `self` parameter. For instance, a trait could define a function such as:
```rust
trait Animal {
  fn baby_name() -> String;
}
```

And a struct could have:
- an own definition of the function, and
- an `imp` of the trait

In those cases, we need to disambiguate with `<Type as Trait>`:
```rust
Dog::baby_name(); // returns its own definition
<Dog as Animal>::baby_name(); // returns the definition in the Animal trait
```

### Using Supertraits

Sometimes a trait definition depends on another trait: for a type to implement T2, it is required that it also implements T1. This allows T2 to make use of the associated items of T1. In this case, T1 is called the *supertrait* of T2.

Supertraits are defined with the syntax:
```rust
trait MyTrait: MySupertrait {
  // ...
}
```

### Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10, we mentioned the orphan rule, which states we're only allowed to implement a trait on a type if either:
- the trait
- the type
- both trait and type

are local to our crate.

It is possible to get around this restriction using the *newtype pattern*, which involves creating a new type in a tuple struct. The tuple struct will have one field and be a thin wrapper around the type for which we want to implement the trait, and will be local to our crate. There is no performance penalty, as the wrapper type is elided at compile time.

An example: implementing `std::fmt::Display` on `std::vec::Vec`. None are local to our crate, so the orphan rule prevents this. However, we can do:
```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(", "))
  }
}

fn main() {
  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {w}");
}
```

The downside of this is that `Wrapper` is a new type, so it doesn't have the methods of the value it's holding. We could solve this issue by implementing `Deref` on the `Wrapper` (see Chapter 15). Otherwise, we could restrict the methods `Wrapper` has by implementing only some.

## Advanced Types

### Using the newtype pattern for type safety and abstraction

Newtypes are used for several purposes, such as:
- Ensuring correct units are passed to a function (e.g.: accept only `Millimeters(u8)`, and not any `u8` or other wrappers such as `Meters(u8)`)
- Exposing an API that's different from the original type (e.g.: `People` type that wraps `HashMap<i32, String>` storing a person's ID and name, exposing methods to add a name to the collection, without exposing the internal usage of `i32`s)

### Creating type synonyms with type aliases

Type aliases can be used with the `type` keyword, as in:
```rust
type Kilometers = i32;
```

In this case, the type is an alias for the basic type `i32`, so it can be interchanged (and confused) with it in the code.

The main usage of type synonyms is to reduce repetitions. For example, we could simplify code that uses a very long type:
```rust
type Thunk = Box<dyn Fn() + Send + 'static>; // thunk: code to be evaluated at a later time

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

or simplify code that uses a certain type many times, leveraging generics:
```rust
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
  fn write(&mut self, buf: &[u8]) -> Result<usize>;
  fn flush(&mut self) -> Result<()>;

  fn write_all(&mut self, buf: &[u8]) -> Result<()>;
  fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

### The Never type that never returns

Rust has a special type named `!`, known as the *empty type* because it has no values. In rust it is called *never type*, because it stands in the place of the return type when a function will never return. Functions returning never are called *diverging functions*.

Some expressions and functions that return `!` are:
- `continue`, in loops
- `panic!`
- infinite `loop`s

The compiler understands the never type in `match` expressions, so it can assign values to variables with a `match` expression that has one or more arms that return never, such as:
```rust
for foo in bar {
  // ...
  let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
  };
}
```

Note that this is only possible due to the never type. Any other type is not allowed, because Rust doesn't have "or" types, only enums. This DOES NOT compile:
```rust
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};
```

### Dynamically sized types (DSTs) and the Sized trait

In general, Rust needs to know variable sizes at compile time, in order to be able to allocate memory for them.

An example of DST is `str`, which is rarely used, as `String` and `&str` are way more common. The following DOES NOT compile:
```rust
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

because `s1` needs 12B of storage and `s2` needs 15B, but both are the same type.

The typical handling with strings are `&str` references (slices), which store:
- starting position of the slice in memory, and
- length of the slice

This means that `&str` is some kind of tuple, with total size `2 * sizeof(usize)`. The golden rule of DSTs is that we must always put values of DSTs behind a pointer of some kind.

We can combine `str` with all kinds of pointers, such as `Box<str>` or `Rc<str>`. In fact, this has been shown before in this book with a different dynamically sized type: traits. Every trait is a dynamically sized type we can refer to by using the name of the trait. In Chapter 18, we mentioned that to use traits as trait objects, we must put them behind a pointer, such as `&dyn Trait`, `Box<dyn Trait>` or `Rc<dyn Trait>`.

To work with DSTs, rust provides the `Sized` trait to determine whether or not a type's size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on `Sized` to every generic function. That is, a generic function definition like this:
```rust
fn generic<T>(t: T)
```
is actually trated as:
```rust
fn generic<T: Sized>(t: T)
```

By default, generic functions will only work with types that have a known size at compile time. However, the following syntax can be used to relax this restriction:
```rust
fn generic<T: ?Sized>(t: &T)
```

A trait bound on `?Sized` means that `T` might not be `Sized`. The `?Trait` syntax with this meaning is **only** available for the `Sized` trait. Note that now `t` is of type `&T`, because `T` might not be typed, so we need to use it behind some kind of pointer, such as `&` (but could be another one).

## Advanced Functions and Closures

Rust allows pure functional programming, by allowing to pass closures and functions to other functions. Some notes:
- Functions coerce to the type `fn`, called *function pointer*
- Function pointers can be used to pass functions to other functions
- `Fn` is the closure trait (not to be confused with `fn`)

A simple usage example is:
```rust
fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(f(arg))
}

fn main() {
  let answer = do_twice(add_one, 5);

  println!("The answer is: {answer}"); // prints 5 = ((3+1) + 1)
}
```

Unlike closures, `fn` is a type rather than a trait. This means that we specify `fn` as the parameter type directly, instead of declaring "a generic type that implements the `Fn` trait", as done with closures.

Function pointers `fn` implement all three closure traits:
- `Fn`
- `FnMut`
- `FnOnce`

This means we can always pass function pointers as arguments for a function that expects a closure. It's best to write functions that accept "generic types that implement `Fn/FnMut/FnOnce`", so that they can take either closures or functions as arguments. In some cases (such as interacting with C), functions declare only `fn` as arguments because C doesn't have closures.

// WIP: listing 20-29