# Generic Data Types

## In function definitions

The syntax is similar to other languages:
```rust
fn largest<T>(list: &[T]) -> &T
```

This example function takes a slice of values of type `T` and returns a reference to a value of the same type `T`.

## In struct definitions

Fields of the struct can have a generic type:
```rust
struct Point<T> {
  x: T,
  y: T,
}
```

or many types:
```rust
struct Point<T, U> {
  x: T,
  y: U,
}
```

## In enum definitions

Analogous to the previous cases. There are examples from the standard lib often used:
```rust
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

## In method definitions

We can define methods for a struct that is of a generic type, so that all instances of the struct implement the type.
For instance:
```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
```
this will add the method `.x()` to all instances of `Point<T>`.

The implementation could be restricted to a specific type. This would make the method available only for instantiations of the struct for that specific type.
For instance:
```rust
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
```

The methods can also declare other generic types, different from the ones in the struct they belong to:
```rust
impl<X1, Y1> Point<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
    Point {
      x: self.x,
      y: other.y,
    }
  }
}
```

## A note about performance

Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.

# Traits: Defining shared behaviour

## Defining a Trait
Traits define behaviours of types, in a similar way that *interfaces* do in other languages. For instance, we could define the behaviour *summarize* that several types should implement:
```rust
pub trait Summary {
  fn summarize(&self) -> String;
}
```

Note that a trait could have several methods.

## Implementing a Trait on a Type

The syntax `impl <Trait> for <Type>` allows us to add the methods of the Trait to a Type. For instance, following the same example as above, we could have:
```rust
pub struct Tweet {
  pub username: String,
  pub content: String,
  // ... others
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}
```

### Restriction

One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate. This means that we can:
* Implement traits from `std::*` in our code
* Implement our self-defined traits on Types from other traits (even like `Vec<T>`)

What we can't do is implement external traits on external types. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

## Default implementation

Instead of declaring the method signature, a Trait could define a default implementation of its method(s), so that Types don't need to write code to implement the Trait.

For example:
```rust
pub trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read more...)")
  }
}

impl Summary for NewsArticle {}
```

would provide `NewsArticle` with the default implementation. However, we could still keep the "custom" implementation in `Tweet`, and it would be overridden.

Methods in the Trait can call other methods in the Trait, and we can have default implementation only for some of them:
```rust
pub trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}
```

Types only need to explicitly define the methods that don't have a default value. For the example above, only the following is required:
```rust
impl Summary for Tweet {
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}
```

## Traits as parameters and return types

If we know that a type implements a trait, we can define parameters as "something that implements trait T", and the function already knows something about its argument. Example:
```rust
pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}
```
### Trait bound syntax

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a *trait bound*; it looks like this:
```rust
pub fn notify<T: Summary>(item: &T) {
  println!("Breaking news! {}", item.summarize());
}
```
For cases with multiple arguments, the syntax `<T: SomeTrait>` will force all arguments of type `T` (`argN: &T`) to be of the same type. However, if declaring them as implementing the trait (`argN: &impl SomeTrait`), they could be of different types, as long as these types implement the trait.

### Implement multiple traits (+)

Arguments can be declared as implementing multiple traits with the plus `+` symbol:
```rust
pub fn notify(item: &(impl Summary + Display)) { ... }
```

works in both syntaxes:
```rust
pub fn notify<T: Summary + Display>(item: &T) { ... }
```

### Where clause

A separate `where` clause can be declared after a function, to give more details about its parameters. This is equivalent to the "normal" syntax- it is just added for readability.
```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{ ... }
```

### Returning a Trait
We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait:

```rust
fn returns_summarizable() -> impl Summary { ... }
```

#### Restriction: single type

If the function that returns a Trait can return different types, Rust won't compile. For instance, this doesn't compile:
```rust
fn returns_summarizable(switch: bool) -> impl Summary {
  if switch {
    return TypeA
  }
  return TypeB
}
```

even if `TypeA` and `TypeB` both implement `Summary`.

More about this restriction in chapter 17.

## Conditionally implement methods on generic types based on trait bounds

For a generic struct, such as this one:
```rust
struct Pair<T> {
  x: T,
  y: T,
}
```

we can conditionally implement a method, depending on the underlying type:
```rust
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("The largest member is x = {}", self.x);
    } else {
      println!("The largest member is y = {}", self.y);
    }
  }
}
```

This restriction after the type (`T: Display + PartialOrd`) ensures that the method `cmp_display` is only available for `Pair` objects whose `T` implements the traits `Display` and `PartialOrd`.

### Blanket implementations

We can implement a trait for *any type that implements another trait*. An example is:
```rust
impl<T: Display> ToString for T { ... }
```
 This snippet from the standard library implements the `ToString` trait (which defines the `to_string` method) on any type that implements `Display`. This means that even integers can be converted to strings:
```rust
let s = 3.to_string();
```

# Validating References with Lifetimes

