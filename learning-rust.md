# Learning Rust

## Cargo

### commands

`cargo build` builds your application and also installs uninstalled dependencies. The package version that is installed during execution will be written to the `cargo.lock`. Any later `cargo build` call will install the exact version mentioned in the lock file.

`cargo update` updates a provided package name and writes the newly installed version to the lock file.

`cargo run` builds and executes your `main.rs` directly in the console.

`cargo doc --open` opens a local web server with doc pages for every dependency (in rust ecosystem called `crate`) of your application



## General stuff

Associated methods === static methods

match statement === switch statement. The match statement can be used on any function that returns an enum. You can then implement specific logic for each value in the Enum. You can also use this match expression to silently ignore thrown errors like this:

```
let number: u32 = match string.trim().parse() {
  Ok(number) => number,
  Err(_) => continue,
};
```



It is possible to reassin previously instantiated let variables by assigning another let with the same name. This is called shadowing and is mostly used for changing the type of the value of a variable.

Use `break;` to stop a `loop`.






