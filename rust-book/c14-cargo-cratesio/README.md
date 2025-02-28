# Customizing builds with Release Profiles

Release profiles are sets of configurations used when compiling code. By default, Cargo comes with 2 default profiles:
* `dev` (default, used when `cargo build`)
* `release` (used when `cargo build --release`)

More profiles can be added to the `Cargo.toml` file:
```toml
[profile.foo]
opt-level = 2
```
overriding existing profiles is also possible.

# Publishing a crate to crates.io

## Adding documentation

Comments with three slashes `///` instead of two are considered documentation comments, and markdown is accepted. For instance:
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can generate the HTML documentation from this documentation comment by running `cargo doc`. Generated HTML will be placed in `target/doc`. For convenience, `cargo doc --open` will also serve the HTML to a browser.

Some common sections to document are:
* `# Panics`
* `# Errors` (when returning a `Result<T,E>`)
* `# Safety` (if a function is `unsafe`)

Note 👀: the code examples will be executed by Cargo when running `cargo test`, even if they are "just a comment". This helps keeping documentation and testing all together.

Comments that start with `//!` are used for documenting the whole crate (e.g. in *src/lib.rs*), or a whole module. They are typically written at the top, to add a more general comment:
```
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// ... start describing functions as done above
```

## Re-exporting internal modules

As explained in CH7, the keyword `pub use` allows crate developers to export internal items (Structs, Functions, etc.) in places different from where they are defined. This changes the API of the crate, and can simplify its usage for consumers. The two main implications of it are:
* Simpler imports: users might need to import something like `coolcrate::MyStruct`, instead of `coolcrate::util::types::MyStruct`
* Clearer documentation: the docs created by `cargo doc` will show re-exported items, and users don't have to click into inner modules to see what items they export

## Setting up a Crates.io account

An account can be created in `crates.io` by logging it through GitHub, and an API key provided. Then, `cargo login` will prompt you for the key, and will store it under `~/.cargo/credentials`.

## Adding metadata to a crate

The `[package]` section of the `Cargo.toml` contains metadata about the package. Some of it is necessary to publish the package. Some common fields are:
* **name**: name of the package
* **version**: version of the package
* **edition**: which edition the compiler should use for your code (2015, 2018, 2021, 2024)
* **description**: free text
* **license**: matching [SPDX](https://spdx.org/licenses/)

## Publishing crates

The command `cargo publish` makes a *permanent* publish to crates.io. The code cannot be deleted, but new versions (using semver) can be added without limit.

## Deprecating (yanking) crates

If a version is broken, it still cannot be deleted, but you can prevent new projects from depending on that version. The command:
```
cargo yank --vers <version>
```

yanks a crate. If it is run by mistake, it can be undone with:
```
cargo yank --vers <version> --undo
```

Note ⚠️: a yank does not delete any code. If there is information published in the crate that must not be public, that information (e.g. passwords) must be changed, because it will forever remain in crates.io.

# Cargo Workspaces

Cargo workspaces is a Cargo feature that allows to manage multiple related packages that are developed together. A workspace is a set of packages that share the same `Cargo.lock` and same output directory.
A parent `Cargo.toml` contains a `[workspace]` section (instead of a `[package]` section), where it lists all the (sub-)packages in it. Sub-packages can be created with `cargo new <binary-crate-name>` or `cargo new <library-crate-name> --lib`. Example:

```toml
[workspace]
members = [ "add_one","adder"]
```

The `target/` directory at the top level is shared: if each crate had its own, each crate would have to recompile the others and move compiled code in its own target directory. By sharing one, crates avoid unnecessary rebuilding.

## Dependencies

Workspaces can depend on each other, by adding a relative path in the `[dependencies]` section of the respective *Cargo.toml*:
```toml
[dependencies]
add_one = { path = "../add_one"}
```

External dependencies are managed in the individual `Cargo.toml` files.

The `Cargo.lock` file is also shared at the top-level, to ensure that all crates are using the same version of all dependencies. Workspaces are able to declare different versions of a library they depend on (alignment is not enforced), and Cargo will try to minimise the number of versions downloaded.

## Testing

Running `cargo test` from the top directory runs tests for all crates in the workspace. The flag `-p` (`--package`) can be used to specify the particular crate you want to run the tests for.

## Publishing

Each crate in the workspace will be published separately (independently). The `-p` flag is also available for publishing.

# Installing Binaries (cargo install)

The command `cargo install` can be used to install rust crates that contain *binary targets* (i.e., a *main.rs* and not only a *lib.rs*) into your computer.
Cargo's default binaries live in `$HOME/.cargo/bin`, and this is normally added to the `$PATH` when installing Cargo locally.

For instance, running `cargo install ripgrep` will add the *ripgrep* binary to our Cargo folder, and will log:
```
...
Installed package `ripgrep v13.0.0` (executable `rg`)
```
From the moment of installation, the binary `rg` will be available.

# Extending Cargo with custom commands

The Cargo CLI can be extended to have more commands. By default, binaries named `cargo-<something>` in the `$PATH` are interpreted by Cargo as sub-commands, and users can execute them with `cargo something`.
Cargo commands can be listed with `cargo --list`.