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
