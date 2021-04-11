# Cargo

## commands

`cargo build` builds your application and also installs uninstalled dependencies. The package version that is installed during execution will be written to the `cargo.lock`. Any later `cargo build` call will install the exact version mentioned in the lock file.

`cargo update` updates a provided package name and writes the newly installed version to the lock file.

`cargo run` builds and executes your `main.rs` directly in the console.

`cargo doc --open` opens a local web server with doc pages for every dependency (in rust ecosystem called `crate`) of your application

# General stuff (Getting Started)

Associated methods === static methods

match statement === switch statement. The match statement can be used on any function that returns an enum. You can then implement specific logic for each value in the Enum. You can also use this match expression to silently ignore thrown errors like this:

```rust
let number: u32 = match string.trim().parse() {
  Ok(number) => number,
  Err(_) => continue,
};
```

It is possible to reassin previously instantiated let variables by assigning another let with the same name. This is called shadowing and is mostly used for changing the type of the value in a variable.

Use `break;` to stop a `loop`.

# Chapter 3: Common programming concepts

## Variables and Mutability

There's two keywords for assigning variables in Rust: `let` and `const`.

A `let` is immutable by default, while a `let mut` variable is mutable. You can also change such variables to become mutable later on.

`const` variables should be named in all uppercase with underscores separating words. They are always immutable and this fact cannot be changed. Use these for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn or the speed of light.

### Shadowing

Is mostly used for changing the tpye of the value in a variable. You can use shadowing to reassing a `let` and have the result of that assignment be immutable still. 
You cannot assign a value of a different data type in to a `let mut` variable without using the `let` keyword again.

## Data Types

### Scalar Types

Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. If I've found anything interesting about these individual types, you will find a dedicated heading for them below.

#### Integers

There's unsigned and signed integer types in Rust. Unsigned integers (can be represented without a sign) can hold a larger positive number, and no negative numbers (e.g. `u8` can contain 0 - 255). Signed integers can hold both positive and negative numbers (e.g. `i8` can contain -128 to 127).

`i32` is the default integer type in Rust and is generally the fastest, even on 64-bit systems.

#### Floating-Point numbers

Rust’s has two floating-point types: `f32` and `f64`, which are 32 bits and 64 bits in size. The default type is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision.

#### char type

`char` literals are specified with single quotes, as opposed to string literals, which use double quotes. Rust’s `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid `char` values in Rust.

### Compount Types

*Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### Tuple type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

You can access elements in a tuple by destructuring `let (x, y, z) = tup;` or by their position like `let one = x.2;`.

#### Array type

Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples. A vector is a similar collection type provided by the standard library that *is* allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector.

```rust
// i32 is the type of each element, 5 is the fixed length
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can access elements in an array by indexing like `a[1]`.

The fixed length of arrays in rust help with security by detecting an out of bound elmenet access during compilation.

## Functions

The keyword for declaring functions is `fn`.

Rust code uses *snake case* as the conventional style for function and variable names (all characters lowercase and words separated by underscore).

Function bodies are made up of a series of statements optionally ending in an expression. *Statements* are instructions that perform some action and do not return a value.

```rust
fn main() {
  let y = 6; // defining a let is a statement -> returns no value
  let x = (let y = 6); // this will throw an error, as a let declaration does not return the assigned value
}
```

*Expressions* evaluate to a resulting value. Examples for expressions:

- a math operation

- calling a function

- calling a macro

- a block {} for declaring new scopes

Example for the curly brace expression:

```rust
fn main() {
  let x = 5;
  let y = { // everything in here is a separate scope
    let x = 3;
    x + 1 // !!! no semicolon at the end means expression, while semicolon at the end means statement
  };
  println!("The value of y is: {}", y);
  println!("The value of x is: {}", x);
}
// Will print:
// The value of y is: 4
// The value of x is: 5
```

The return value of a function is specified after a `->` in the function signature:

```rust
fn five() -> i32 { // return type is i32
  // The return keyword is optional.
  // Any function returns the last expression by default.
  5 
}
```

# Chapter 4: Understanding Ownership

Some languages manage a computer’s memory while running using garbage collection during runtime, others have prgraommers explicitly allocating and freeing memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

Keep in mind: when a function returns a value, the ownership of that value is moved to the variable we assign the function call to.

## Ownership Rules

- Each value in Rust has a variable that’s called its *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

**About the "value borrowed here after move" error**:

This error occurs when you use a String variable  "a" that has been invalidated by Rust because a later String variable "b" has a reference to the same ponter that used to belong to "a" (it was copied).

From a JavaScript perspective the variable "b" would be a shallow copy of variable "a", and you could still use both while mutating the same data on the heap. Rust's copying style is called "move".

When we reach the next curly brace after variable "a" and "b", only the memory that "b" points to will be freed.

Rust does this to avoid the so called "double free" error, which can lead to memory safety bugs. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

Types that implement the `Copy` trait are still usable after assignment, which does not apply to `String`.

Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## deep cloning

If you do want to create a deep clone of a String, you use the `.clone()` method. Keep in mind that this can get expensive.

## copying fixed size types

The size of integers is known at compile time so they are always stored on the stack (where copying is fast). So if you assign a "let x = 5" and a "let y = x", the value of x is copied into y, which is not an expensive thing to do during compilation. Both "x" and "y" remain valid variables.

## references and borrowing

If you want to pass a variable into a different scope (such as a function) but don't want that function to take ownership, you pass it as a &reference. This is called *borrowing*.

References are immutable by default, but can be made mutable by declaring them like this: `fn foo(bar: &mut String)` if your source variable is also mutable.

There's one limitation to mutable references: At any given time, you can have *either* one mutable reference *or* any number of immutable references.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
println!("{}, {}", r1, r2);
// -> cannot borrow `s` as mutable more than once at a time
```

This limitation allows the compiler to prevent data races at compile time.

Also if there could be a mutable reference to a variable while an immutable one exists as well, the compiler could no longer guarantee that the immutable one won't change.

### dangling references

In languages with pointers, it’s easy to erroneously create a *dangling pointer*, a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

## Slices

Slices in short are immutable references to parts of a `String` or `Array`, with a syntax like `&array[1..3];` (this is an immutable reference to the first three elements of `array`).

# Chapter 5: Structs

Assigning/updating/merging structs works quite similar to JavaScript Objects. See this code for example, which sets new values for `email` and `username` and takes the others from an existing object:

```rust
let user2 = User {
  email: String::from("another@example.com"),
  username: String::from("anotherusername567"),
  ..user1
};
```

Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

# Chapter 6: Enums and Pattern Matching

Enums are cool! Want to have variables that can be null? Doesn't exist in Rust!

Instead you use the `Option` enum.

In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

> I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

Here's a direct link to the talk on the QCon Website: https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/

**Key takeaways from the talk** (Source QCon Webiste)

- Null references have historically been a bad idea
- Early compilers provided opt-out switches for run-time checks, at the expense of correctness
- Programming language designers should be responsible for the errors in programs written in that language
- Customer requests and markets may not ask for what's good for them; they may need regulation to build the market
- If the billion dollar mistake was the null pointer, the C gets function is a multi-billion dollar mistake that created the opportunity for malware and viruses to thrive

### TypeScript vs Rust

In JavaScript there exist not one way to represent a value that is currently invalid or absent for some reason but two: `undefined` and `null`.

Let's compare an implementation of a function that trims whitespace on user input, where one input is optional:

**Typescript**

```typescript
interface FormData {
  mandatoryField: string;
  optionalField: null | string;
}
function sanitizeFormData(mandatoryField: string, optionalField: null | string = null): FormData {
  const formData: FormData = { mandatoryField: mandatoryField.trim(), optionalField }
  if (optionalField !== null) {
    // since optionalField can be null, we check this before calling trim().
    formData.optionalField = optionalField.trim()
  }
  return fromData;
}
```

**Rust**

```rust
struct FormData {
    mandatory_field: String,
    optional_field: Option<String>
}

fn sanitize_form_data(mandatory_field: &String, optional_field: &Option<String>) -> FormData {
    let sanitized_mandatory_field = mandatory_field.trim().to_owned();
    let sanitized_optional_field: Option<String> = match optional_field {
        Some(field_value) => {
            Some(field_value.trim().to_owned())
        },
        None => None
    };
    return FormData {
        mandatory_field: sanitized_mandatory_field,
        optional_field: sanitized_optional_field
    }
}
```

## Defining methods on enums

```rust
enum Message {
    Quit,
    Custom(String),
    Confirm
}
impl Message {
    fn printValue(&self) {
        // since &self is an enum, you would use match here again to write specific logic.
        println!("{:?}", self); // prints "hello" because that's what we used below.
    }
}
let m = Message::Custom(String::from("hello"));
m.printValue();
```

Also see the code example [here](./chapter6/enums/src/message.rs) for a more advanced example of enum methods where I also used the `Option` enum for hadling `null` values (in rust: `None`).

# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- **Packages:** A Cargo feature that lets you build, test, and share crates
- **Crates:** A tree of modules that produces a library or executable
- **Modules and use:** Let you control the organization, scope, and privacy of paths
- **Paths:** A way of naming an item, such as a struct, function, or module

Any rust project that contains a `Cargo.toml` file is a package.

**The pub keyword**

The `pub` keyword. Everything in rust is private by default
You can write `pub` in front of `mod`, `fn`, `struct` and `enum` to make them public.
If you make a `struct` public, its properties are still private by default and you can control their visibility individually.
If you have a `struct` with private properties, it is mandatory to have some sort of associated constructor method.

**Module paths**

You can navigate the contents within a module using paths. There are relative and absolute paths. Absolute paths start with `crate::`. You can use `super::` to call a function from the parent module.
Wonder if you should use absolute or relative paths? Absolute paths are usually the way where you'll end up fixing less paths when moving things around.
Here's an example path: `crate::front_of_house::hosting`.

**The use keyword**

We can bring a path into a scope once and then call the items in that path as if they’re local items with the `use` keyword. Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem. By adding `use crate::front_of_house::hosting` in the crate root, `hosting` is now a valid name in that scope.

You shouldn't use `use` to bring functions into a named scope, as this makes it hard to understand if a function comes from the current/root module or from a different one.

On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. Example: `use std::collections::HashMap;`

By adding the `pub` keyword in front of `use`, you are reexporting the module you are importing in your current module. Example: `pub use crate::front_of_house::hosting;`

**The as keyword**

You can use the `as` keyword to define import aliases. This is useful for when you're importing to structs with the same name from different modules:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
```

**Nested imports**

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files. Nested imports come to the rescue:

```rust
// we can turn this:
use std::cmp::Ordering;
use std::io;
// into this:
use std::{cmp::Ordering, io};
```

**The glob operator**

`use std::collections::*;`  brings *all* public items defined in a path into scope. Be careful when using this operator as it makes it harder to see which depencies your crate actually has.

### Organizing modules in separate directories and files

If you write a bunch of rust code in different files inside the `spaghetti` directory in your `src` folder. You can import the code within that directory in your `main.rs` by using `mod spaghetti` to bring it into scope.

# Chapter 8: Common Collections

The data that collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

## Vectors

- [Link to the API docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Link to my code examples](./chapter8/vectors/src/main.rs)

The `Vec<T>` type is implemented using generics, so when you initialize an empty vector, you can define the type that each item will have like this: `Vec<i32>`.

Rust will also infer the type for the values inside a Vector. For defining a vector that has initial values, use the built-in `vec!` macro like this: `let v = vec![1, 2, 3];`.

You can add elements to a vector using `.push(element)`. This is only possible on mutable vectors.

There's two ways to access an element in rust:
1. indexing syntax `vector[index]`. This will cause the program to panic when trying to access an index that is out of bounds. Personal note: I find it surprising that rust doesn't check out of bounds index access for immutable vectors, where the compiler could know how long this vector actually is. This feels kind of inconsistent after the chapter on the `Option` enum and the billion dollar mistake of the `null` type. In [this stackoverflow answer on C++](https://stackoverflow.com/a/1239977) the reasoning for having a way to access an array element without bounds checking, is that it's faster than the one that does check bounds.
2. calling `Vec.get(index)`, which returns an `Option` enum, where you can use the `match` operator to handle `Some` or `None` element.

As long as another variable holds a reference to an element a mutable vector, you cannot change the vector until that reference is destroyed, as mutating it might require allocating new memory and copying the old elements to the new space.

## Strings

- [Link to the API docs](https://doc.rust-lang.org/std/string/struct.String.html)
- [Link to my code examples](./chapter8/strings/src/main.rs)

When Rustaceans refer to “strings” in Rust, they usually mean the `String` and the string slice `&str` types, not just one of those types. The `String` type is a growable, mutable, owned and UTF-8 encoded. `&str` is also UTF-8 encoded.

There is no way to access a character in a string by index.

This is because some strings of foreign languages might look like they contain four characters, like the hindi `नमस्ते`, but they actually contain six char values (`['न', 'म', 'स', '्', 'त', 'े']`), where the fourth and the sixth are diacritics that don’t make sense on their own.

Rust forces you to acknowledge that characters from different languages can be stored very differently so that accessing by index is not natural.

What you can do is call `String.chars()` to iterate over each char within a string.

## Hash map

- [Link to the API docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Link to my code examples](./chapter8/hash-maps/src/main.rs)

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.

# Chapter 10

## Generics

- [Link to my code examples](./chapter10/generics/src/main.rs)
- [Link to official Rust code examples](https://doc.rust-lang.org/stable/rust-by-example/generics.html)

We can use generics to create definitions for items like function signatures, structs or enums, which we can then use with many different concrete data types. 
You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.

### Enums and generics

When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.

Examples:
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

### Performance of Code Using Generics

Rust code using generic types doesn’t run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. When the code runs, it performs just as it would if we had duplicated each definition by hand. 

```rust
// these two usages of the generic Option<T> enum...
let integer = Some(5);
let float = Some(5.0);

// are compiled into...
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

## Traits

- [Link to my code examples](./chapter10/traits/src/main.rs)

Traits are similar to a feature often called interfaces in other languages, although with some differences, such as default implementations for methods on a Trait.

### Traits as parameters
Instead of a concrete type for a function parameter, we can specify the impl keyword and the trait name. Such a parameter accepts any type that implements the specified trait. In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize

```rust
// this method accepts any instance of a struct that implements the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// the syntax above is just syntactic sugar for the bound syntax:
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

You can also type a method parameter in a way that it requires the provided struct to implement many traits:
```rust
pub fn notify(item: &(impl Summary + Display)) {
  println!("Display: {}, Summary: {}", item.display(), item.summarize())
}

// with the bound syntax:
pub fn notify<T: Summary + Display>(item: &T) {
  println!("Display: {}, Summary: {}", item.display(), item.summarize())
}
```

For easier readability Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature:
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { /* ...*/ }

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ /* ... */ }
```

### Lifetimes

Another kind of generic that we’ve already been using is called lifetimes. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.

Rust uses lifetimes to save you from having dangling references in your program, which it does at compile time.

```rust
{
  let r;
  {
    let x = 5;
    r = &x;
  } // at this point the variable x has gone out of scope and its memory is deallocated.
  println!("r: {}", r); // wont't compile because r is set to a reference of the "now dead" x.
}
```

The syntax for lifetimes goes like this:
```rust
// vor variable definitions:
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
// for functions:
fn foo<'a>(x: &'a str, y: &'a str) -> &'a str { /* ... */ }
// for structs:
struct Play<'a> {
    part: &'a str,
}
```

#### Lifetime Elision
Why do we not need to provide the variable lifetimes for a function like: `fn first_word(s: &str) -> &str { /* ... */ }`?

This has a historic reason because the Rust team found themselves implementing the same lifetime generic `'a` in many places of the standard library, where they could also infer it based on **three rules**.
The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

**The first rule** is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

**The second rule** is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

**The third rule** is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

These three rules don't work for functions that accept more than one parameter today.
But in the Rust book they say that eventually the compiler will be able to do more lifetime inferring for you.

#### The Static Lifetime
There is one special `'static` lifetime, which indicates that a variable never goes out of scope while your program executes.

# Chapter 11: Writing automated tests

- [Link to my code examples](./chapter11/tests/src/lib.rs)

Most unit tests go into a tests mod with the `#[cfg(test)]` attribute. Test functions are marked with the `#[test]` attribute. The official rust convention recommends to write unit tests for a given code in the same file as the implementation. However there's nothing keeping you from writing tests in a separate file with a `.test.rs` suffix.

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

## Assertion macros

The failure messages are optional parameters of these macros.

```rust
// the assert macro panics when the provided expression (in this case x) returns false
assert!(x, "x wasn't true, but: {}!", x);

// the assert_eq macro panics when the first two parameters are not equal
// and also outputs the difference in this case
assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

// the assert_ne macro panics when the firs two parameters are equal
assert_ne!(a, b, "we are testing that the values are not equal");
```

## Testing panics

Use the `#[should_panic]` attribute to assert that some test should panic.

```rust
#[test]
#[should_panic]
fn test_any_panic() {
    divide_non_zero_result(1, 0);
}
```

Such a test will be green for any panic that occurs in the code.
To make sure that you're testing for a specific panic, use the `expected` parameter: `#[should_panic(expected = "Divide-by-zero error")]`.

## Using Result<T, E> in Tests

None of the previous unit test examples had a return type. But in Rust 2018, your unit tests can return `Result<()>`, which lets you use the `?` operator in them! This can make them much more concise.

In contrast to the unit tests above, these tests are red when the Result value is `Err` and are green when the result value is `Ok`. No panicking required for failing tests.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail.

```rust
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test] // non happy path test with no Result return type
    fn test_sqrt_fails_for_negative_float() {
        let x = -2.0;
        assert!(sqrt(x).is_err())
    }

    #[test] // non happy path test with Result return type
    fn test_sqrt_fails_for_negative_float_with_result() -> Result<(), String> {
        let x = -2.0;
        match sqrt(x) {
            Ok(_) => Err("sqrt didn't fail for negative float".to_owned()),
            Err(_) => Ok(())
        }
    }
}
```

### The ? operator

Chapter 11 is the first time I came across this `?` parameter and it's quite handy so I'll explain it right now. Here's a [link](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html) to the official documentation in the 2018 edition guide.

```rust
// taken from the offical docs
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// with ? operator:
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
```
The `?` is shorthand for the entire match statements in the first implementation. In other words, `?` applies to a `Result` value, and if it was an `Ok`, it unwraps it and gives the inner value. If it was an `Err`, it returns from the function you're currently in. Visually, it is much more straightforward. Instead of an entire match statement, now we are just using the single `?` character to indicate that here we are handling errors in the standard way, by passing them up the call stack.

## Running cargo test with GitHub Actions

As part of this chapter I've also added a GitHub Actions workflow that executes the tests I've added on each push.
I learned that there's several unoffical but helpful rust actions available in the [actions-rs](https://github.com/actions-rs) organization.

[Link to my unit test workflow](https://github.com/xkons/learning-rust/blob/master/.github/workflows/tests.yml)