# RUST Learning

## rusttest.rs
hello world [equivalent](https://www.techrepublic.com/article/how-to-install-rust-on-linux/) for rust

## macro vs function
`println!` is a macro and `println` is a function
for now, we know that they behave differently 

## Cargo
- `cargo new <package-name>` to create a new cargo project
- will contain a `cargo.toml` and `src/main.rs`
- creates a new git repository along with .gitignore file (overridden if inside existing git repo)
- cargo expects the root directory to contain the Cargo.toml file, license info, config files, readme etc. Functional code file should be in src
- if there are no code changes, it will not recompile the code (build the project)
- when the project is rebuilt, we will see output for that
- use `rustup` to update rust

## Cargo Commands
- `cargo build` creates an executable in `target/debug/<package_name>`
- use `./target/debug/<executable_name>` to run it
- use 'cargo run` to compile and run in one command
- use `cargo check` to see if the package compiles, but does not produce an executable (usually this will be much faster than creating an executable when you are still developing your code)
- also at this point, once you have installed rust, you are OS agnostic since the CLI commands are the same for all OS
- when you are ready, you can compile with optimizations for a faster release build `cargo build --release`
- use `cargo doc --open` to generate documentation for the deps you are using
	- this will create docs locally, so you should gitignore the `/target` folder
## TOML
"Tom's Obvious Minimal Language"
- `[]` are section headings
- package section: the following statements are configuring a package
- dependencies section: list other dependencies referred to as crates 
- crate's version follows Semantic Versioning
- semantic verionsioning makes sure you get the latest patches for your version, but gaurantee compatability so your code will compile
- when we inlcude a crate it will be a library crate not a binary create which we create when runningg cargo build/run etc
- cargo fetches the latest versions of hte deps you have specified from the *registry*
- the registry is a copy of the data on Crates.io which is where people post OS Rust projects
- after downloading the crates, they are compiled for the first time
- if the dependencies are not changed, then they are no recompiled
- when you build a project for the first time, cargo saves the working verions of deps in `.lock` file
- if there is a breaking update, then it will not be added to the project, thanks to `.lock` file


## Crates
- `rand` is an exmple crate we could use, rng is not part of rust std lib
- adding a new crate:
	- add crate and version to the `.toml` file
	- add the `use` statment to the `.rs` file


## Variables and Mutability
- use `let`
	- `let` and other *statements* do not have a return value (as in C)
	- therefore `let x = y = 6;` is invalid in rust
- in rust variables are immutable by default
- `mut` indicates that are variable is going to be immutable
- `const` is used for variables that must be immutable
	- rust might do some kind of optimization with consts, not sure tho
- we use the term "bind" when dealing with variables and `=`
- shadowing: repeat the usage of `let` on an exiting variable's name
- with shadowing we can perform some transformations, but keep the variable immutable
- shadowing respects the rules of scope as usual
- shadowing essentially will create a new variable, which *could* have a different type too
- shadowing could help us avoid having to come up with creative helper var names
- we cannot change the type of `mut` variables, however. This is a key difference between them

## Data Types
- every value in rust has a datatype (statically typed language, types need to be known at compile time)
- there are two subsets of datatypes: scalar and compound
- the compiler can infer the type, but when we do conversions we should specify
- four primary scalar types: integers, floating-pt, Bool, char
- signed integers are stored using two's complement representation
- we can use an underscore as a visual separator for any kind of number
- in debug mode overflow is checked during compile time, which would cause a runtime panick
- if compiled in release mode, overflow checking is foregone and 2's complement wrapping is used
	- relying on this behavior is considered an error
	- there are some libraries that offer wrapping and saturation (research more)
- f32 is ieee-754 floating point standard and f64 is double precision
	- have roughly the same runtime from the CPU
- `bool` type can be inferred or explicitly typed by the programmer
- division is floored unless there is a floating point operand
- char is specified with singular quotes
	- is four bytes, represents a unicode scalar value (ie more than just ascii values)
	- chars are little more complex in rust than human intuition tells us (see ch 8)
### Compound types
- could be Tuple or Array
- *Tuple* has fixed size after init, we can assign tuple's values to indiv values called `desctructing`
- we can also access tuples by 0'd index using `.` operator
- `unit type` tuple has no elements which can be used when there is no meaningful value to be assigned. The value returned is the `unit value`
- we can define *arrays* with their [type;size] or [value; ct]
- during runtime accesses of arrays, it will check array bounds (other low-level langs don't check this, and will proceed to blindly segfault). If detected, code execution will stop and tell you there was a panik

## Fucntions
- use snake_case
- declare with `fn`
- rust does not care where you define functions
- must declare the types of the parameters so rust does not need to infer their types
- function bodies contain statements and expressions
	- bodies are made up of `statements`, *optionally* ending in an `expression`
	- `expressions`: instructions that perform some action and do not return a value
	- `expressions` evaluate to a resulting value
- we can use `{}` alone to create their own scopes
- use arrow notation to return a value from a function
- you can return early from a function using `return` but most functions return the last expression implicitly

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

```

## Expressions vs Statements
- rust is an expression-based language
- function definitions are statements, therefore they cannot simply return a value
	- instead we need to use `->` notation
- let is a statement
- in order to convert an expression into a statement, we need to remove the semi-colon from the last line of a block (enclosed by `{}`)
- if we accidently put a semi-colon at the end of a returning function, we will get a compiler error as the expected return type would be the empty type, and not what we had specified


## Control Flow
- `if` statment with `else if` and `else`
- `if` can be an exprression, so a variable can be bound it is return value
	- kinda like a ternarary
	- same type must be returned (compile time check)

- looping constructs include `loop`, `while` and `for`
- we can label loops (good for nested)
- we can use `break` to return a value if the value is placed right after it
	- these will have a semi-colon after them
- while loop has a condition
- enhanced for loop adds more safety to the code and iterates through arrays faster since the compiler does not need to check for bounds



## Other
Terminator, move between windows alt + arrow
[Parameter vs Argument](https://stackoverflow.com/a/23992345)
