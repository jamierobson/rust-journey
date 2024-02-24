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

## Trying to build the reference structure
So my idea for a sudoku solver starts with modelling the game itself. I realised the following ideas

- The Cell should be an item that represents a value, or none, and then some extra information for example possible or discounted values
- The cells should be owned by a grid. The grid would simply be the 81x81 (or whatever dimention you want) cells laid out, and this is the avenue you take to actually update a cell. The grid owns all of the cells.
- The ideas of rows, columns, blocks, are all just groups of cells that have the same validation logic on top of it. Their value is in being validatable units, and eventually for cells to be able to prompt the groups they are a member of to reassess the gamestate after a value has been updated, and if valid, to tell all other cells in those groups to discount the value placed here.
- The game owns all of this, and serves as the entry point of the sudoku.

The relationships between the grid, and the rows/columns/blocks has been a real sticking point. I tried:

- Have the grid with a property `[[Cell; size]; size]`, and having the cell groups have some kind of shared `Rc<Cell>`, but this ended up failing, as I seemed to have to clone the existing cell - which kind of lost the point of trying to have other things reference it.
- Include lifetimes all the way up to the Game type, even though this gets in the way of my ultimate goal of getting this compiled to wasm. I found, however, that I couldn't instantiate a cell grid, instantiate rows/columns/blocks with references to the grid `(error[E0515]: cannot return value referencing local data)`. I have a skill issue, somehow saying "but I don't want to own this any more, here you go, caller, it's yours". 

So I ended up the place I really wanted to avoid, the `Rc<RefCell<Cell>>`. 

At least now the rows/columns/blocks actually point at my cells now, and I can update the cell values as expected. Not best thrilled about it, but it's progress,