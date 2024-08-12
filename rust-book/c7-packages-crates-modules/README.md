# Managing growing projects with Packages, Crates and Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

* _Packages_: A Cargo feature that lets you build, test, and share crates
* _Crates_: A tree of modules that produces a library or executable
* _Modules_ and _use_: Let you control the organization, scope, and privacy of paths
* _Paths_: A way of naming an item, such as a struct, function, or module

## 7.1 Packages and Crates

*Crate*: smallest amount of code that the rust compiler considers at a time. Crates can contain modules, which may be defined in other files that get compiled with the crate.

*Crate root*: source file that the Rust compiler starts from and makes up the root module of your crate.

Crates can be of one of two types:
* _Binary crates_: have a _main_ function, and can be run
* _Library crates_: have utility functions/structs to be used by some binary crate, or another library crate.

A _package_ is a bundle of one or more crates that contains a `Cargo.toml` file and provides a set of functionality. The default (convention) files for Cargo are:
* `src/main.rs`: for the crate root of a binary crate with the same name as the package
* `src/lib.rs`: for the crate root of a library with the same name as the package

A package can only have one library crate. It may contain several binary crates by placing files in the `src/bin` directory.

## 7.2 Defining Modules to control scope and privacy

Example in `restaurant-lib` directory. It is a library crate with modules and sub-modules:
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Note that the parent (in this case, `src/lib.rs`) is an implicit module with name `crate`. This is why it is called "root crate".

## 7.3 Paths for referring to an item in the Module tree

Paths are used to refer to items (functions, methods, structs, enums, modules, and constants) in the module tree. They can be absolute (from crate root, called `crate`) or relative (by using keywords such as `super` or `self`), and are separated with `::` the same way paths in the filesystem are separated with `/`.

### Private by default

In Rust, all items are private to parent modules by default. Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor and sibling modules.

### Exposing Paths with the 'pub' keyword

The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. This is useless unless we add `pub` to (some of) its items.

### Relative paths with 'super'

`super` is the equivalent of `..` in a filesystem. It can be used to refer to the path of the parent. Use when the exports of the parent will always live one level above.

### Structs and enums

By default, struct fields are private, even if the struct is declared as `pub`. Each field must define its visibility.

Note that if a struct has private fields, it needs to provide a public associated "constructor" to generate instances of it. Otherwise, external code can't create the private fields. See the `summer` function of `crate::back_of_house::Breakfast` in the example code of `restaurant-lib`.

In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.