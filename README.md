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

## Ownership and closures
This is doing my head in. I want to use lambdas and be functional in my code, e.g. to get a grouping of cells by those with the same potential values

```
        cell_group.cells
        .iter()
        .filter_map(|weak| weak.upgrade())
        .map(|rc| rc.clone())
        .group_by(|rc| rc.clone().borrow().potentially_valid_values)
        .into_iter()
        .collect();
```

and I don't get the error
`creates a temporary value which is freed while still in use`

It feels like I should be expressing what I want here - I'm going to have to take an alternative and less idiomatic approach. It's not the first time, however, that I've run into issues using lambdas. They're doing my head in. Just please, read the values and then let it be done. 

## My eyes
I admit it, I don't really like short hand. I find it adds a mental load to reading the code, and I much prefer fully typed out words. I guess that's to be expected, I am coming from c#, after all. We all have autocomplete in our IDEs, so I don't think we need to pretend that it's really saving us any keystrokes. To that end, my two biggest bugbears are `Vec` and `iter()`, and I can thankfully report that I've been able to make these read, to my eyes at least, better, as `Vector` and `iterate()`. `&str` is another one that bothers me, but less egrigeous I guess becuase I understand it is a slice, and `String` is taken. `StringSlice`... I kind of prefer it to `&str` ~~but not enough to actually try do anything about that one.... yet.~~

## And back to Rc<>
So after playing around, something dawned on me. I had initially thought that the cell grid is the only thing to alter a cell, and for that reason it stood to reason that the validatable units were purely a view on the data. However in the solver, I'm applying logic across all cells in the group, and updating the cells - from the group - with inferences from other cells. E.g. a validatable unit is looking at all the used values in all of its cells, and then causing all other cells to eliminate that as an option.

And then it dawned on me. The dreaded feeling.

This _is_ shared ownership wiht interior mutability, and the `Rc<RefCell<Cell>>` is precisely the tool for the job. And to think, I fought _so_ hard against it. Oops! Well at least I learned about the `Weak` type, and the pattern of 
```
weak_references
.iter()
.filter_map(|weak| weak.upgrade())
.continue_from_here(...)
```

## Conjugate groups
The next step I looked at was to consider the "naked pairs" idea. I figured that I should like an implementation that can handle any group size, and so settled upon the idea of looking at each cell, identifying if its candidates are an inclusive subset of anothers, and building groupings in this way. To that end, the hashmap felt appropriate. One annoyance I experienced was:

```
           all_keys
           .iterate()
           .filter(|key| key.is_superset_of(&cell_reference.borrow().potentially_valid_values))
           .for_each(|matched_key| {
                let count = dictionary.get(matched_key).expect("We are iterating over the keys that seeded the HashMap");
                dictionary.insert(&matched_key, count + 1);
           });
```

`all_keys` originally wasn't de-duped. This is because I had thought to use `dictionary.keys()` - but this leads to an attempt to borrow the dictionary mutably to insert, while borrowing immutably in order to iterate. Oops. 

Next up, I'd like to look at further techniques, to get as far as I can before "resorting" to a brute force approach. Currently I have a puzzle with 50 completed cells, and I need to look at it by hand and work out what is missing to solve that one. Exciting!

The initial state of the puzzle: 
```
...97564...13..572.7....8....27....3..7..32..8..6.2..74.9....6.7..8..1.4286.34...
| x | x | x || 9 | 7 | 5 || 6 | 4 | x |
| x | x | 1 || 3 | x | x || 5 | 7 | 2 |
| x | 7 | x || x | x | x || 8 | x | x |
| x | x | 2 || 7 | x | x || x | x | 3 |
| x | x | 7 || x | x | 3 || 2 | x | x |
| 8 | x | x || 6 | x | 2 || x | x | 7 |
| 4 | x | 9 || x | x | x || x | 6 | x |
| 7 | x | x || 8 | x | x || 1 | x | 4 |
| 2 | 8 | 6 || x | 3 | 4 || x | x | x |
```

is now at 
```
328975641..13..572.7....839..27....3..7..32.68..6.2..7419..73687..8..124286134795
| 3 | 2 | 8 || 9 | 7 | 5 || 6 | 4 | 1 |
| x | x | 1 || 3 | x | x || 5 | 7 | 2 |
| x | 7 | x || x | x | x || 8 | 3 | 9 |
| x | x | 2 || 7 | x | x || x | x | 3 |
| x | x | 7 || x | x | 3 || 2 | x | 6 |
| 8 | x | x || 6 | x | 2 || x | x | 7 |
| 4 | 1 | 9 || x | x | 7 || 3 | 6 | 8 |
| 7 | x | x || 8 | x | x || 1 | 2 | 4 |
| 2 | 8 | 6 || 1 | 3 | 4 || 7 | 9 | 5 |
```