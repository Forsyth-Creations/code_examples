# How to Start a Rust Project from scratch

1. `cargo new <name>` - Creates a new binary project
2. `cd <name>` - Navigate into the project directory
3. `cargo build` - Build the project
4. `cargo run` - Run the project

# Creating a library in that workspace

1. `cargo new <lib-name> --lib` - Creates a new library crate. Example: `argo new crates/common_logic_gates --lib`
2. Add the library as a dependency in `Cargo.toml` if needed
3. `cargo build` - Build the library
4. `cargo test` - Run library tests

Additional:

`cargo add printing --path crates/printing`

To avoid the .git folder, you can do `cargo init --vsc none` (the `--vsc none` being the most important) on whatever command you run