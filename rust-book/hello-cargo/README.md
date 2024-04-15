# Hello Cargo


## Cargo project structure

```
├── src
│   ├── main.rs
│   ├── <code>
├── target
│   ├── debug/
│   │   ├── .fingerprint/
│   │   ├── build/
│   │   ├── deps/
│   │   ├── ...
│   │   ├── .cargo-lock
│   │   ├── <executable>
│   ├── .rustc_info.json
│   ├── CACHEDIR.TAG
├── Cargo.lock
├── Cargo.toml
└──  README.md
```

## Commands

* `cargo new <project-name>`: creates a new project, adding the `Cargo.toml`, a `src/` folder, and an example _Hello World_.
* `cargo build`: builds the code in `target/debug` and generates a `Cargo.lock`. Must be run from the folder containing `Cargo.toml`.
  - `cargo build --release`: builds the code in `target/release`, and optimizes code further. Takes longer to compile, but produces a faster runtime.
* `cargo run`: does the same as `cargo build`, and then executes the code.
* `cargo check`: makes sure code compiles, but doesn't generate output. Runs faster than `cargo build`.