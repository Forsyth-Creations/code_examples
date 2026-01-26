# Rust Learning Examples

A comprehensive collection of Rust examples demonstrating various concepts and best practices.

## Quick Start

### Run the Interactive Menu

```bash
cargo run
```

This launches an interactive menu where you can choose which example to run.

### Run Individual Examples

```bash
# Run a specific example
cargo run -p common_primatives
cargo run -p printing
cargo run -p ownership_and_borrowing
# ... etc
```

### Build All Examples

```bash
cargo build --workspace
```

### Run All Tests

```bash
cargo test --workspace
```

## Learning Path

The examples are organized in a suggested learning order:

1. **common_primatives** - Learn basic Rust primitive types (integers, floats, booleans, chars, tuples, arrays, strings)
2. **common_logic_gates** - Understand boolean operations and logic gates
3. **printing** - Master console output and formatting options
4. **properly_erroring** - Learn proper error handling with Result and Option types
5. **traits_and_attributes** - Understand traits (interfaces) and attributes (metadata)
6. **ownership_and_borrowing** - Master Rust's unique memory management system
7. **logging** - Implement structured logging with different log levels
8. **parallelism** - Learn multi-threading and parallel iteration
9. **singleton_example** - Implement the singleton pattern with thread safety
10. **performace_awareness** - Understand performance optimization techniques
11. **documentation** - Write excellent documentation with doc comments
12. **testing** - Create comprehensive unit tests
13. **build_config_cli** - Parse command-line arguments and configuration
14. **features** - Use cargo features for conditional compilation

## Example Descriptions

### common_primatives
Demonstrates all basic Rust data types including integers, floats, booleans, characters, tuples, arrays, and strings.

### common_logic_gates
Implements boolean logic gates (AND, OR, NOT, NAND, NOR, XOR, XNOR) with truth tables.

### printing
Shows various ways to format and print output to the console, including positional arguments, named arguments, debug formatting, and number formatting.

### properly_erroring
Demonstrates proper error handling using Result and Option types, the ? operator, and custom error types.

### traits_and_attributes
Explains how to define and implement traits (similar to interfaces), and how to use common derive attributes.

### ownership_and_borrowing
Covers Rust's unique ownership system, borrowing rules, and how to prevent dangling references.

### logging
Shows how to use the `log` and `env_logger` crates for structured logging with different log levels.

**Run with:** `RUST_LOG=debug cargo run -p logging`

### parallelism
Demonstrates multi-threading with std::thread and parallel iteration with the rayon crate.

### singleton_example
Implements the singleton pattern using lazy_static and Mutex for thread-safe global state.

### performace_awareness
Covers performance optimization techniques like avoiding unnecessary allocations, using iterators, and understanding stack vs heap allocation.

### documentation
Shows how to write excellent documentation using doc comments with examples, arguments, and return value descriptions.

**Generate docs:** `cargo doc --open -p documentation`

### testing
Demonstrates unit testing with various assertion types, should_panic tests, and ignored tests.

**Run tests:** `cargo test -p testing`

### build_config_cli
Shows how to parse command-line arguments using the clap crate with derive macros.

**Try it:** `cargo run -p build_config_cli -- --name Alice --count 3 --verbose`

### features
Demonstrates cargo features for conditional compilation.

**Run with features:** `cargo run -p features --features advanced,experimental`

## Project Structure

```
learning/
├── Cargo.toml          # Workspace configuration
├── Cargo.lock          # Dependency lock file
├── src/
│   └── main.rs         # Interactive menu runner
└── crates/
    ├── common_primatives/
    ├── common_logic_gates/
    ├── printing/
    ├── properly_erroring/
    ├── traits_and_attributes/
    ├── ownership_and_borrowing/
    ├── logging/
    ├── parallelism/
    ├── singleton_example/
    ├── performace_awareness/
    ├── documentation/
    ├── testing/
    ├── build_config_cli/
    └── features/
```

## Tips

- Start with the examples in the suggested learning order
- Read the code comments in each example
- Modify the examples and see what happens
- Run `cargo clippy` to get helpful suggestions
- Run `cargo fmt` to format your code
- Use `cargo doc --open` to browse generated documentation

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)