# Writing Automated Tests

## Code utilities

Typically, test blocks are organised in modules (`mod tests`). In order to use the exports of the module under test, we can include a `use super::*` or similar statement in the module. This will make exports visible in the test module.
Test functions are annotated with `#[test]`. We might have other auxiliary functions in the tests module(s): annotations tell Cargo which ones are tests.

### Assertion macros

Assertion macros can be used to make the test function panic if the obtained result doesn't match the expected, or if a result is not true. They include:
* `assert!(some_bool)`
* `assert_eq!(a, b)`
* `assert_ne!(a, b)`

Note: `assert_eq!` and `assert_ne!` use the operators `==` and `!=`, and print the *left* and *right* arguments when the assertion fails. For that, they need that the arguments passed implement the traits:
* `PartialEq`, for (in)equality evaluations
* `Debug`, for printing
Most of the times, they can be derived with the annotation: `#[derive(PartialEq, Debug)]`.

Assertion messages can be added as a last parameter to the `assert` functions. This parameter is then passed to the `format!` macro, so placeholders in curly braces `{}` can be passed. This helps understanding errors better. For instance, one could write:
```rust
let result = greeting("Carol");
assert!(
  result.contains("Carol"),
  "Greeting did not contain name. Instead, value was `{result}`"
);
```

### Tests either panic or return Result<T, E>

The assertion macros covered above will make the test function panic, which will make the testing framework realise they failed. However, another contract is valid: returning a `Result`. In other words, the following function is also a valid test:
```rust
#[test]
fn it_works() -> Result<(), String> {
  let result = add(2, 2);

  if result == 4 {
    Ok(())
  } else {
    Err(String::from("two plus two does not equal four"))
  }
}
```
Writing tests so they return a `Result<T, E>` enables you to use the question mark (`?`) operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.
Note that the annotation `#[should_panic]` cannot be used with tests that return a `Result`.

### Test annotations

Apart from `#[test]`, other annotations can be added to the test functions. Here are the most useful:
* `#[should_panic]`: passes the test if the tested function panics
* `#[should_panic(expected = "some substring")]`: passes the string if (1) the tested function panics and (2) if the panic message contains "some substring" in it.
* `#[ignore]`: skips the test execution. Will be notified in Cargo's test summary.
* `#[bench]`: indicates that a function is a benchmark to be run. It must only take one argument of type `test::Bencher`.

## Cargo utilities

The command `cargo test` runs the functions annotated with `#[test]` and reports the results. It indicates functions that:
* passed
* failed
* were ignored (marked to not be executed)
* were filtered out (skipped when running the `test` cli command)
* measure performance
* test that API documentation matches code (*Doc-tests*)

Cargo runs tests in different threads, and marks dead threads (e.g. threads that panic) as failed.

The default behaviour of `cargo test` is to run all the tests in parallel and capture the output of the tests. It is possible, however, to specify command line options passed to the tests (as opposed to options passed to cargo) in the CLI. The top-level command would be:
```
cargo test <args-for-cargo> -- <args-for-test>
```

### Parallelism

The flag `--test-threads=<number>` controls the number of threads used to run the tests. For example, for running the test in series, one would write:
```
$ cargo test -- --test-threads=1
```

### Logging

Rust's test library captures all the output printed to *stdout* during tests, and only prints to the terminal the output of a test that fails. The flag `--show-output` shows all values, even for tests that passed:
```
$ cargo test -- --show-output
```

### Filtering tests

Filtering by test name requires only to pass a string that matches the name of the test(s) that we want to filter. This is, the following command:
```
$ cargo test <str>
```
will only run the test(s) that contain the string `<str>` in their name.

Another way of filtering is to use the `--ignored` or flag. This will run only the tests annotated with `#[ignore]`.
In order to run all tests, the flag `--include-ignored` can be used.

## Organising tests

### Unit tests

The convention is to co-locate tests in the same file as the code they're testing, in a module called `tests` annotated with `#[cfg(test)]`. This annotation tells cargo to compile and run the code only when running *cargo test*. Compiling tests only for testing saves compilation time for other code.
More in detail: the `cfg` in the annotation means *configuration*, and the rust compiler will only compile code annotated with it if the `test` configuration is passed by Cargo. Note that other helper functions in the module (used at test time), don't have to be annotated with `#[test]` but are anyways compiled if living under the `#[cfg(test)]` module.

Note about testing private functions: unlike other languages, Rust allows you to test private functions. This is due to the property mentioned in chapter 7: child modules can use items from their ancestor modules. This means that a child test module can import its parent with `use super::*`, and private functions will be included as well.

### Integration tests

These tests use the library as any other code would: externally, and calling only the public API. They are usually located under a `tests/` directory, next to the `src/` directory:
```
adder
├─ Cargo.lock
├─ Cargo.toml
├─ src/
│   └─ lib.rs
└─ tests/
    └─ integration_test.rs
```

Because of being external to the library, they must import the functions to be tested as any other user. Each file under the tests directory is a separate crate.
Note that they do not need the annotation `#[cfg(test)]`, since they are living outside the codebase. Cargo knows about the convention of the `tests/` directory.

So far, we have seen 3 kinds of tests showing up in the terminal when runing *cargo test*:
* unit tests
* integration tests
* documentation tests

Note that if any test in a section fails, the following sections will not be run. For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.

Each file in the `tests/` directory corresponds to an integrations test section. To run all the tests in a specific integration test file, we can use the `--test` argument of cargo test, followed by the name of the file:
```
$ cargo test --test <integration_test_file>
```

#### Submodules in integration tests

Because of being treated as *one crate per file*, files corresponding to integration tests don't follow the same rules as other files in Rust. In particular:
* Every file under `tests/` is one crate. Functions in these files will be executed as integration tests.
* Files in subdirectories of the tests directory don’t get compiled as separate crates and don't have sections in the test output.

This means that we can group "common" test utilities this way:
```
├─ Cargo.lock
├─ Cargo.toml
├─ src
│   └─ lib.rs
└─ tests
    ├─ common
    │   └─ mod.rs
    └─ integration_test.rs
```

In the `integration_test`, we can use the `common` functions as:
```rust
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
  common::setup();
  let result = add_two(2);
  assert_eq!(result, 4);
}
```

#### Integration tests for binary crates

Binary crates (i.e., those with a `main.rs` instead of a `lib.rs`), don't export an API that can be imported and tested in a `tests/` folder.
Many binary traits have a very lean `src/main.rs` that calls logic that lives in `src/lib.rs`. This makes it possible to test all the code in `src/lib.rs` with integration tests. The only code left untested is in `src/main.rs`, but it should be minimal.