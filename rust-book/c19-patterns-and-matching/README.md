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