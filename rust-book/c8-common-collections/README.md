# Common Collections

Collections can contain multiple values. Unlike array/tuple (built-in), the data is stored on the *heap*. This means the amount of data can be unknown at compile time, and can grow/shrink.
This chapter discusses 3 types of collections:
- Vectors: store values
- Strings: store characters
- HashMaps: particular implementation of Map (key-value pairs)

## 8.1 Vectors

Can only store values of the same type. Can be created, defining the type of `Vec<T>`, with:
```rust
let v: Vec<i32> = Vec::new();
```

Otherwise, Rust will do type inference if we create the vector with the `vec!` macro:
```rust
let v = vec![1,2,3]
```
This will infer `i32` as data type, as it is the default integer type.

### Updating a vector

The vector must be declare as mutable (`mut`) in order to modify it, for instance by pushing elements to it: `v.push(5);`

Borrow checking rules apply to it as well: if a reference to the vector exists, modifying the vector is not allowed between the reference declaration and its usage. Example of code that *does not* compile:
```rust
let mut v = vec![1,2,3];
let first = &v[0];
v.push(4);
println!("The first element of the vector is: {first}");
```
Because of the dynamic nature of vectors, pushing an element to it might require to move the vector in memory. This would cause the "first" (immutable) reference to `v[0]` to point at the wrong position in memory. This is why the code above does not compile.

### Accessing a vector

The getter method will return an optional, as element might not exist (type annotation for clarity):
```rust
let v = vec![1, 2, 3, 4, 5];
let third: Option<&i32> = v.get(2);
```

The direct access with `v[index]` will return the type directly => ❗ program can panick if out of bounds:
```rust
let fifty_fifth: &i32 = v[55];
```

### Iterating a vector

Vectors can be iterated using enhanced `for ... in` loops. Typically, references to the vector are used:
```rust
let v = vec![1,2,3];
for i in &v {
  println!("{i}");
}
```
both the vector and the reference must be declared as mutable if the vector is to be modified in the loop:
```rust
let mut v = vec![1,2,3];
for i in &mut v {
  *i = *i + 1;
}
```

The borrow checker would complain if vector was modified during the loop (e.g. with `v.push(42)`), since that would imply having 2 references to the vector interfering with each other:
* one in the loop declaration: `&v` or `&mut v`
* one in the loop, modifying the vector: `v.push(42)`

### Using an enum to store multiple types

Equivalently to the or-type in Typescript (`type Foo = Bar | Baz`), enums can be used in Rust to define a single type that can contain different sub-types. As seen in chapter 6, we can define:
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
```
this will allow us to define `Vec<SpreadshetCell>`:

Note: If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object, covered in Chapter 17.

## 8.2 Strings

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

### String creation

If no data at creation time:
```rust
let mut s = String::new();
```

if starting with some text (equivalent):
```rust
let s1 = "initial contents".to_string();
let s2 = String::from("initial contents");
```


### String update

There are several methods we can use:

```rust
let mut s0 = String::from("foo");
s.push_str("bar");
s.push('c');
let s1 = String::from("baz");
s0 += &s1;
let original_string = String::from("original string: ");
let s2 = format!("{original_string}{s0} !! :D");
```

### String indices

Accessing a String by an index like a vector (e.g. `s[0]`) is not allowed in Rust.
This is because strings store UTF-8 data, and not all characters the same number of bytes:
- `a` takes 1 Byte of memory
- `й` takes 2 Bytes of memory

Therefore, accessing a certain position is not a good API. Direct access does not compile in Rust. Furthermore, some Unicode scalar values (Rust's `char`s) store only diacritics that modify a previous `char`, and they don't make sense on their own.

Slices of the String, interpreted as scalar values, can be obtained from the String with either of:
```rust
let hello = "Hola";

let h_opt: Option<&str> = h.get(0..1);
let h: &str = &h[0..1];
```

⚠️ However, this might cause the program to panic at runtime, if the access is halfway through a character that takes several bytes, such as in:
```rust
let hello = "дравствуйте";
let h = &h[0..1]; // this will panic
```

### String iteration

Strings provide the methods `.chars()`
```rust
for c in "Зд".chars() {
    println!("{c}"); // prints first "З" and then "д"
}
```
and `.bytes()`
```rust
for b in "Зд".bytes() {
    println!("{b}"); // prints "208", "151", "208" and "180"
}
```

The standard library does not provide methods to get the "grapheme clusters" from strings (needed in some languages), and delegates this responsibility to other crates.

## 8.3 Hashmaps

Equivalent to objects/maps/dictionaries in other languages.

### Creating Hashmaps

It has to be explicitly imported, since it is not brought into scope automatically by Rust.
The types can be inferred by the compiler if not declared in the constructor:
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

### Accessing values in hashmaps

Hashmaps `<K,V>` provide a `get(key: &K): Option<&V>` method.

They can also be iterated with references in a for loop:
```rust
for (key, value) in &map {
  // key: &K
  // value: &V
}
```

### Hashmaps and ownership

When adding elements to the HashMap that are of a type that does not implement the Copy trait, those elements will be owned by the HashMap. For instance:
* a `i32` will be copied, and the HashMap will have a copy of it
* a `String` will be moved, and the HashMap will own it

As explained in chapter 4, those elements cannot be used after the ownership.

Note: this does not apply to references. In other words, this is acceptable:
```rust
let some_string = String::from("some_string");
let mut hm = HashMap::from([&some_string, 42]);
println!("I can still use '{some_string}' because the hashmap doesn't own it");
```

### Updating Hashmaps

Since keys are unique, there's different alternatives when inserting new values:

1. Overriding old value with *insert*:
```rust
map.insert(old_key, new_value);
```
2. Adding new value *only* if key does not exist, using the `Entry` auxiliary object:
```rust
map.entry(some_key).or_insert(new_value); // map.entry: Entry<'_, K, V>
```
3. Adding new value based on the old value, by retrieving the inserted/obtained value:
```rust
let value = map.entry(some_key).or_insert(0);
*value += 1;
```
Note the pointer dereference to modify the value in the map.
Note as well that `.or_insert` returns a `&mut V`, a mutable pointer to a part of the map. This means that until the result has been used, we cannot use other references to the map (e.g. a `print!`).

### Hashing functions

Hashmap implements *SipHash*, but other hashers can be chosen as long as they implement the `BuildHasher` trait. More about traits in chapter 10. 