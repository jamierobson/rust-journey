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


## Trying to get rid of RefCell
I now have a construction whereby the grid has Cells, and want explore all the potential ways I can get rid of this RefCell. It feels like a crutch, and I hear that this is precisely true. Here's what I tried

- Adding lifetimes
  
  This fell over when either creating the game, or the grid, I would create something and then eventually call some kind of function with closure to create the blocks/rows/columns which ended up borrowing the grid, and not letting me assign the when constructing Self

- Adding Weak references
  
  I fell over with the `Weak<Cell>` idea, but `Weak<RefCell>` is still a step in the right direction, I feel, so I'll roll with it.

- Avoiding the references, hoping that slicing is performant

If I store on the cell it's row and column, then we can always use these coordinates to identify which cell, block, row it would be in, create 3 `CellGroup` and use these. It means constantly recomputing which cells belong in which group, so that something can, say, tell every other cell in all of the groups to delete a candidate when setting a number, but this is a step i may move to. 

Time to move on, for now

## Getting on with things
Now that this is out of the way, I decided to move on to getting valid games into the program, and explored the `Regex` type. I find it funny that it is searching in a `haystack`. I don't know if that's proper terminology, but I chuckled. Defining a puzzle as a `&str` whose every character is 1-9 or a "." to represent nothing, then mapping that to just an `Option<u8>` grid, which I can use to spawn cells with values seems to work. I am happy for the exitence of the `from_fn` array function. I didn't know if it was better to make my `Parser` a type or not, but recreating `Regex` to parse new games feels awkward. But then again, so does the empty constructor. Maybe it makes more sense when I want to support differernt sizes.

## Simple solves
Now that we're fighting less against the syntax and the borrow checker, I'm able to start to be a touch more expressive, in creating a simple solve when there's 1 obvious candidate for a cell. I have a test that can solve at least one non-trivial but very easy puzzle!