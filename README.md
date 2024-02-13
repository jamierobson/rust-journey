# rust-journey
My second attempt to pick up rust, after bouncing off the first time

## Step by step getting started:

## Tooling
Installing the rust toolchain
- `choco install rustup.install`
- `rustup update`
- `rustup default stable-x86_64-pc-windows-gnu`
- `rustup toolchain install stable-gnu`

Software
- [VS Code](https://code.visualstudio.com/download) with following extensions
  - Rust Syntax
  - rust-analyzer

## Hello World
Hello World
- `cargo init`
- `cargo run`

## Calling functions from other files
if in same directory, you can just use `mod::file_name`
if in some kind of child directory, then each folder going down needs `mod.rs`, which exposes the files as `mod`. see `hello_world\mod.rs` that exposes ``hello_world\greeter.rs`. Now from `main.rs` I can reference the `mod folder_name`, and from there, reference, for example, a method from `greeter.rs` by `hello_world::greeter::some_public_function`

## Adding types:

I see what's going on here.

```
pub struct MyType {
    members
}

impl MyType {
    public functions
}

fn my_private_function(argument: &mut ArgumentType){
    // lets you modify argument
}
```

It seems to me you should almost always take a `&T` reference as an argument? And the argument must be `&mut T` if you want to alter it - both on the function definition and arguments when invoking. Fair enough. It's hard to accidentally let the functiom change something if you demand in the function that a caller allows you to mutate.