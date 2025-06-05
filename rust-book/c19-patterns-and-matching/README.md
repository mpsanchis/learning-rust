# Patterns and matching

Patterns are a special syntax in Rust for matching against the structure of types. A pattern consists of some combination of the following:
- Literals
- Destructured:
  * arrays
  * enums
  * structs
  * tuples
- Wildcards
- Placeholders

Some example patterns include:
- `x`
- `(a, 3)`
- `Some(Color::Red)`

In the contexts in which patterns are valid, these components describe the shape of data.

## All the places patterns can be used

All 6 cases are:
- match VALUE { PATTERN => EXPRESSION }
- if let PATTERN = EXPRESSION
- while let PATTERN = EXPRESSION
- for PATTERN in EXPRESSION
- let PATTERN = EXPRESSION
- function and closure parameters: fn my_function(PATTERN)

### match arms

Match expressions have the following structure:
```
match VALUE {
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  ...
}
```

One requirement for `match` expressions is that they need to be exhaustive.

The pattern `_` will match anything, but never binds to a variable, so it's often used in the last match arm.

### conditional 'if let' expressions

Note that `if let` expressions can be combined with `else`, `else if`, and `else if let`.

Example:
```rust
if let Some(color) = favorite_color {
  // ...
} else if is_tuesday {
  // ...
} else if let Ok(age) = optional_age {
  if age > 30 {
    // ...
  }
} else {
  // ...
}
```

### conditional 'while let' loops

Allows the loop to:
- assign a variable at each iteration, leveraging deconstruction
- end when the pattern doesn't match

Example (destructures when finds `Ok`, and ends when finds `None`):
```Rust
while let Ok(value) = rx.recv() {
  println!("{value}");
}
```

### for loops

In a `for` loop, the value that directly follows the keyword `for` is a pattern:
```Rust
for (index, value) in v.iter().enumerate() {
  println!("vec[{index}] = {value}");
}
```

### let statements

All `let` statements use patterns:
```
let PATTERN = EXPRESSION;
```

however, most cases are very simple, such as `let x = 5`. This can be used to destructure:
```Rust
let (x, y, z) = (1, 2, 3);
```

and will be checked by the compiler and fail if pattern doesn't match. The following code does **NOT** compile:
```Rust
let (x, y) = (1, 2, 3);
```

### function and closure parameters

As with `let`, we can match for instance a tuple in a function's argument to the pattern:
```Rust
fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Location: ({x}, {y})");
}
```

## Refutability

Patterns that match for any possible value are *irrefutable* (e.g.: `let x = 5`)
If the compiler can reject a PATTERN = EXPRESSION assignment, it is because the pattern is *refutable* (e.g.: `if let Some(x) = Option::None`)

Irrefutable patterns should be used in:
- `let` statements
- `for` loops
- function parameters

in these three cases, there is no condition to evaluate, so it doesn't make sense to have a refutable pattern anyways. If a refutable pattern is passed, Rust won't compile.

The only way to add a refutable pattern in those cases is to provide an alternative to the code logic, such as:
```Rust
let Some(x) = some_option_value else {
  println!("some_option_value was None");
  return;
}
```

In the other 3 cases (`match`, `if let`, `while let`) accept any pattern, but the compiler will throw a warning if they use irrefutable patterns. If they always evaluate to `true` in the conditional logic, it doesn't make sense to use conditional logic at all.

## Pattern syntax

This section discusses all usage cases of patterns.

### Matching literals

Patterns can be matched directly against literals, such as numbers. This is similar to a *switch-case* in other languages:
```Rust
let x: u8 = // ...
match x {
  1 => println!("one"),
  _ => println!("not one")
}
```

### Matching named variables

Note that each arm in a `match` expression starts its own scope, so defining new variables in the pattern will shadow variables that have the same name and live in an outer scope. In other words, the following code will print 42:
```Rust
let x = Some(42);
let y = 10;
match x {
  Some(y) => println!("x = {y}"),
  Some(50) => println!("x = 50"),
  _ => println!("None"),
}
```

### Multiple patterns

The pattern *or* operator, `|`, can be used to match multiple patterns:
```Rust
let x = 2;
match x {
  1 | 2 => println!("x == 1 or x == 2");
  _ => println!("x != 1 and x != 2");
}
```

this code will print `x == 1 or x == 2`.

### Matching ranges of values with '..='

The following code checks against a range:
```Rust
let x = 5;

match x {
  1..=5 => println!("one through five"),
  _ => println!("something else"),
}
```

and prints `one through five`.

Note: ranges are types (`Range<i32>` and `RangeInclusive<i32>` are the inferred types when writing examples for this book).

The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are `char` and numeric values, ranges are only allowed with numeric or `char` values. This code is therefore also valid:
```Rust
let x = 'c';

match x {
  'a'..='j' => println!("early ASCII letter"),
  'k'..='z' => println!("late ASCII letter"),
  _ => println!("something else"),
}
```

and will print `early ASCII letter` as expected.

### Destructuring to break apart values

Patterns can be used to destructure structs, enums and tuples.

#### Destructuring structs

An example of variable creation via destructuring is:
```Rust
struct Point {
  x: i32,
  y: i32,
}
// construction
let p = Point { x: 0, y: 7 };
// deconstruction: setting variables 'a' and 'b' implicitly
let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);
// deconstruction when using the same names as the struct defines
let q = Point { x: 1, y: 8 };
let Point { x, y } = q;
assert_eq!(1, x);
assert_eq!(8, y);
```

Destructuring can also be used in a `match` expression:
```Rust
let p = Point { x: 0, y: 7 };
match p {
  Point { x, y: 0 } => println!("On the x axis at {x}"),
  Point { x: 0, y } => println!("On the y axis at {y}"), // will match
  Point { x, y } => {
    println!("On neither axis: ({x}, {y})");
  }
}
```

#### Destructuring enums

Enums potentially contain other structures inside. This means the destructuring and/or matching must consider them. For instance, the enum:
```Rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
```

can be matched against different structures such as:
```Rust
match msg {
  Message::Quit => {
    println!("The Quit variant has no data to destructure.");
  }
  Message::Move { x, y } => {
    println!("Move in the x direction {x} and in the y direction {y}");
  }
  Message::Write(text) => {
    println!("Text message: {text}");
  }
  Message::ChangeColor(r, g, b) => {
    println!("Change the color to red {r}, green {g}, and blue {b}");
  }
}
```

#### Destructuring nested enums and structs

Destructuring applies to nested structures with any depth. The following two enums:
```Rust
enum Color {
  Rgb(i32, i32, i32),
  Hsv(i32, i32, i32),
}
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(Color),
}
```

can be combined in nested `match` statements like this:
```rust
match msg {
  Message::ChangeColor(color) => {
    match color {
      Color::Rgb(r, g, b) => println!("Color = (r: {r}, g: {g}, b: {b})"),
      Color::Hsv(h, s, v) => println!("Color = (h: {h}, s: {s}, v: {v})"),
    }
  }
  // ... other arms
}
```

but also in a single `match` statement like this:
```Rust
match msg {
  Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change color to R{r} G{g} B{b}"),
  // other arms
}
```

#### Destructuring structs and tuples

We can mix, match, and nest destructuring patterns in even more complex ways. The following example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:
```Rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring values in a pattern

In order to get only parts of the pattern, we can use different strategies:
- using the `_` pattern
- using the `_` pattern within another pattern
- using a name that starts with underscore
- using `..` to ignore the remaining parts of a value

In cases where a trait requires multiple parameters, but your struct won't use them all, the trait function(s) can declare parameter(s) unused with `_`:
```Rust
trait SomeTrait {
  fn trait_action(&self, foo: &str, bar: i32) -> String;
}

impl SomeTrait for MyStruct {
  fn trait_action(&self, _: &str, bar: i32) -> String {
    format!("Number: {bar}")
  }
}
```

We can also ignore some values when many might match:
```Rust
let numbers = (2, 4, 8, 16, 32);
match numbers {
  (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
  // ...
}
// ...
let foo = Some(5);
match foo {
  Some(_) => println!("Not None, but I don't care what value"),
  //...
}
```

Variable names starting with underscore (e.g. `_velocity`) won't trigger a warning in the compiler. This can be useful when prototyping, for instance.

Remaining parts of a value can be ignored with `..`, like in:
```Rust
let origin = Point { x: 0, y: 0, z: 0 };
match origin {
  Point { x, .. } => println!("x is {x}"),
}
```

or in:
```Rust
let numbers = (2, 4, 8, 16, 32);
match numbers {
  (first, .., last) => {
    println!("Some numbers: {first}, {last}");
  }
}
```

However, this notation can be ambiguous in some cases. In those cases, the program **will NOT** compile, such as in:
```Rust
match numbers {
  (.., n_th, ..) => {
    println!("Some numbers: {n_th}")
  },
}
```

### Extra conditionals with match guards

A *match guard* is an additional `if` condition specified after the pattern in a `match` arm. An example is:
```Rust
let num = Some(4);
match num {
  Some(x) if x % 2 == 0 => println!("The number {x} is even"), // will match here
  Some(x) => println!("The number {x} is odd"),
  None => (),
}
```

or also:
```Rust
let x = 4;
let y = false;

match x {
  4 | 5 | 6 if y => println!("yes"), // won't match because 'y' is 'false'
  _ => println!("no"), // will match
}
```

### '@' Bindings

The `@` operator lets us create a variable that holds a value at the same time we're testing that variable for a pattern match. The following example shows three arms of a `match` that do:
1. Binding value to a variable with a new name, testing variable for a range, and using variable in arm
2. Testing a value for a range, without saving it into a variable
3. Saving value into a variable with default name, without testing it against a range

```Rust
enum Message {
  Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };
match msg {
  Message::Hello {
    id: id_variable @ 3..=7,
  } => println!("Found an id in range: {id_variable}"),
  Message::Hello { id: 10..=12 } => {
    println!("Found an id in another range")
  }
  Message::Hello { id } => println!("Found some other id: {id}"),
}
```