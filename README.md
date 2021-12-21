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
- in rust variables are immutable by default
- `mut` indicates that are variable is going to be immutable
- we use the term "bind" when dealing with variables and `=`

 
## Other
Terminator, move between windows alt + arrow
[Parameter vs Argument](https://stackoverflow.com/a/23992345)
