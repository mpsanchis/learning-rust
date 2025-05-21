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