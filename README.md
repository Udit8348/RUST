# RUST Learning
## Cargo
- `cargo new <package-name>` to create a new cargo project
	- will contain a `cargo.toml` and `src/main.rs`
- creates a new git repository along with .gitignore file (overridden if inside existing git repo)
- cargo expects the root directory to contain the Cargo.toml file, license info, config files, readme etc. Functional code file should be in src
	- lists the multiple rust projects you have in this root directory (created with `cargo new`)
	- file names should be in snake_case
- if there are no code changes, it will not recompile the code (build the project)
- when the project is rebuilt, we will see output for that
- use `rustup` to update rust

## Cargo Commands
- `cargo build` creates an executable in `target/debug/<package_name>`
- use `./target/debug/<executable_name>` to run it
- use `cargo run` to compile and run in one command
	- if our cargo TOML has multiple bins specified, then you can tell it which bin to choose using `--bin <name>` ie: `cargo run --bin calculator`
- use `cargo check` to see if the package compiles, but does not produce an executable (usually this will be much faster than creating an executable when you are still developing your code)
- also at this point, once you have installed rust, you are OS agnostic since the CLI commands are the same for all OS
- when you are ready, you can compile with optimizations for a faster release build `cargo build --release`
- use `cargo doc --open` to generate documentation for the deps you are using
	- this will create docs locally, so you should gitignore the `/target` folder

## Manually Compiling
- `rustc` compile rust code into machine code


## TOML
"Tom's Obvious Minimal Language"
- `[]` are section headings
- package section: the following statements are configuring a package
- dependencies section: list other dependencies referred to as crates 
- crate's version follows Semantic Versioning
- semantic versioning makes sure you get the latest patches for your version, but guarantee compatibility so your code will compile
- when we include a crate it will be a library crate not a binary create which we create when running cargo build/run etc
- cargo fetches the latest versions of the deps you have specified from the *registry*
- the registry is a copy of the data on Crates.io which is where people post OS Rust projects
- after downloading the crates, they are compiled for the first time
- if the dependencies are not changed, then they are no recompiled
- when you build a project for the first time, cargo saves the working versions of deps in `.lock` file
- if there is a breaking update, then it will not be added to the project, thanks to `.lock` file

## Crates
- `rand` is an example crate we could use, rng is not part of rust std lib
- adding a new crate:
	- add crate and version to the `.toml` file
	- add the `use` statement to the `.rs` file

---

## Variables and Mutability
- use snake_case
- use SCREAMING_SNAKE_CASE for constants
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
- in debug mode overflow is checked during compile time, which would cause a runtime panic
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
- `&` indicates a reference type in rust

### str vs String
- `str` is an immutable fixed length string *somewhere* in memory
- `String` is a growable, heap allocated data structure. *Note*: it is analogous to `StringBuffer` from Java, not Java `String` which is immutable.
`String` maintains if length and capacity, while `str` only has len()
- probably has to be a reference type since it will always get heap allocated
- heap allocation lets them be unknown sizes at compile time
- it is not automatically created when we have `""`, you need to convert it into a String using the `from` trait of `String` which looks like (`String::from("Udit")`)
- strings do no implement the Copy trait, so we cannot use that to manage ownership issues

### Characters
These are "Unicode Scalar Values", USV
Use single quotes, and are represented with values like `U+221E`
We can do character analysis on a string using `.len()` to get the number of bytes and `.chars().count()`
- important distinction here is that ascii characters will have one byte of data dedicated to them, while unicode characters may have more bytes dedicated to them for example `∞` has 3 bytes, so using `chars count` will tell us that despite having 3 bytes, it is still one character. `chars` uses an iterator to determine individual characters.
- **dont** confuse this with sizeof `char`, since we have observed a character having different lengths. The truth is that `chars` have to be *4 bytes* in size, or smaller. This is so that they can hold an entire unicode character.

### Compound types
- could be Tuple or Array
- *Tuple* has fixed size after init, we can assign tuple's values to indiv values called `destructing`
- we can also access tuples by 0'd index using `.` operator
- `unit type` tuple has no elements which can be used when there is no meaningful value to be assigned. The value returned is the `unit value`
- we can define *arrays* with their [type;size] or [value; ct]
- during runtime accesses of arrays, it will check array bounds (other low-level langs don't check this, and will proceed to blindly segfault). If detected, code execution will stop and tell you there was a panik
---

## Functions
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

### macro vs function
`println!` is a macro and `println` is a function
- think of macro as a piece of code that writes other code
- macros can take in a variable number of arguments 

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
- if we accidentally put a semi-colon at the end of a returning function, we will get a compiler error as the expected return type would be the empty type, and not what we had specified

## Control Flow
- `if` statement with `else if` and `else`
- `if` can be an expression, so a variable can be bound it is return value
	- kinda like a ternary
	- same type must be returned (compile time check)

- looping constructs include `loop`, `while` and `for`
- we can label loops (good for nested)
- we can use `break` to return a value if the value is placed right after it
	- these will have a semi-colon after them
- while loop has a condition
- enhanced for loop adds more safety to the code and iterates through arrays faster since the compiler does not need to check for bounds

## Ownership
In rust a big memory management concept is the notion that a reference can only have one owner. If we reassign a reference, then the reference belongs to the newest owner. One example of this could be creating a string, then assigning the reference to a new variable. If we try to use the original, we will get an error. (Look it up using `rustc --explain E0382`)
- simply states: a variable was used after its contents were moved elsewhere
- resolutions: use a Copy, Clone, or pass by reference (or just use & to get a reference)
- read more in rust book chapter 4

### References
If we want to modify a reference, we need to have ownership over it

---

## Testing in Rust



## Other
Terminator, move between windows alt + arrow

[Parameter vs Argument](https://stackoverflow.com/a/23992345)

### rusttest.rs
located in manual, it is a hello world [equivalent](https://www.techrepublic.com/article/how-to-install-rust-on-linux/) for rust that demonstrates how to manually compile for rust