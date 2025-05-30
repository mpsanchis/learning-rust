# OOP features of Rust

## Characteristics of OOP languages

### Having objects
There is no unique definition of what an OOP language is, but the gang of 4 defines an OOP program as:
> Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations.

Using this definition, Rust is object oriented: structs and enums have data, and `impl` blocks provide methods (procedures).

### Encapsulation

Encapsulation means that implementation details of an object aren't accessible to code using that object. Therefore, the only way to interact with the object is with its public API.

Using this definition, Rust meets the requirement, because it provides the `pub` keyword to define what is public API.

### Inheritance as a type system and as code sharing

Inheritance is a mechanism whereby an object can inherit elements (data and/or methods) from another object's definition. Rust does not allow this.

The use cases for inheritance are:
1. Code reuse (not redeclaring data and methods again)
2. Type system (being able to use different objects in the same places, because they have a somehow equivalent API)

To many people, polymorphism = inheritance. However, it is a more general concept that refers to code that can work with data of multiple types.
- For inheritance, those types are generally subclasses
- In rust, generics abstract over different types and trait bounds to impose constraints on what those types must provide. This is sometimes called *bounded parametric polymorphism*.

Inheritance has some issues:
- Subclasses can be inheriting more than necessary
  - Introduces possibility of calling methods that don't make sense on the subclass
  - Reduces flexibility in design (subclass comes with a lot pre-defined and fixed)
- In some languages, only single inheritances are allowed, which restricts flexibility in design

For these reasons, Rust takes the different approach of using trait objects instead of inheritance.

## Using trait objects that allow for values of different types

For a fixed-length, known-at-compile-time amount of different types, an `enum` is typically the preferred option.

However, in some cases we offer a library that can be extended by users, and in those cases the custom types created by users is unknown to our code. An example is a GUI library with a tool that iterates through a list of items, and calls `.draw()` on all of them. The library might offer types such as `Button` or `TextField` for users, but someone might want to create a custom `SelectBox` that can also be drawn.

In languages with inheritance, we could define a `Component` class with a `draw` method in it. Classes that inherit from `Component` would also inherit the `draw` method, and a `Vec<Component>` would suffice to store any number of any number of different component types. Because Rust doesn't have inheritance, we need different strategies.

### Defining a trait for common behaviour

We can define a trait `Draw` that has one `draw()` method. If we want a vector that stores types that implement `Draw`, we need that the vector takes a *trait object*. A trait object points to both an instance of a type implementing a trait and a table used to look up trait methods of that type at runtime. We create a trait object by specifiying:
- Some sort of pointer or smart pointer, such as `&` or `Box<T>`
- The `dyn` keyword
- The relevant trait

We can use trait objects in place of a generic or concrete type. Wherever we use trait objects, Rust's type system will ensure at compile time that any value used in that context will implement the trait specified in the trait object. Consequently, we don't need to know all possible types at compile time.

In Rust, we never refer to `struct`s or `enum`s as objects, because data and methods are separate. Trait objects, however, are more like objects in pure OOP languages: they combine data and behaviour. The difference with "classic" objects is that we can't add data to a trait object. Their purpose is to allow abstraction across common behaviour.

A code example (with more detail in the `trait_object` module in `src/`) is:
```Rust
pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // trait object, allowing any Draw object at runtime
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
```

⚠️ Note: this is **NOT** equivalent to working with generics. The following code restricts all items in `components` to be of any type, but of the same type:

```Rust
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen<T>
where
  T: Draw,
{
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
```

### Trait objects perform dynamic dispatch

In chapter 10, we discussed the process of *monomorphization* performed on generics by the compiler: the compiler generates non-generic implementations of functions and methods for each type that we use, in place of a generic type parameter. The code that results from monomorphization is doing *static dispatch*, because the compiler knows what method you're calling at compile time.

This is opposed to *dynamic dispatch*, which is used when the compiler doesn't know at compile time which method you're calling. In these cases, the compiler emits code that will figure out only at runtime which method to call. At runtime, Rust uses the **pointers inside the trait object** to know what method to call. This lookup incurs a runtime cost that doesn't occur with static dispatch.

Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations. Rust has also some rules, called *dyn compatibility*, about where you can and cannot use dynamic dispatch. Those rules are out of the scope of this book, but are listed in the [Rust reference](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).

## Implementing an object-oriented design pattern (state pattern)

The *state pattern* is an OO design pattern. The crux of it is:
- There is a set of states a value can have internally
- The value's behaviour changes based on its state
- States are represented by state objects
- Each state is responsible for its own behaviour, and for governing when it should change to another state
- The value that holds the state object knows nothing about the different behaviours, or changes of state

The advantage of this pattern is that when business requirements change, the code of the value holding the state object doesn't need to change. We only need to update the affected state(s), and maybe add more states.

This book exemplifies the state pattern with a blog post (`Post`) object, which can be in states "draft", "review" or "published". The functionality looks like this:
* A blog post starts as an empty draft
* When the draft is done, a review is requested
* When the post is approved, it gets published
* Only published posts return the content to print, to prevent publishing unapproved content
* Any other changes (e.g.: approving a draft before requesting review) should have no effect


// WIP. TODO:
// - quick summary of OOP state pattern here (code done)
// - clarify why "state: Some(Box::new(Draft {}))" instead of "state: Box::new(Draft {})"
// - write the "rust" implementation (as opposed to OOP)