# Rust

## Starting a Project

To create a new Rust project:

```bash
cargo new project_name
cd project_name
```

## Building and Running

```bash
# Build the project
cargo build

#-+-+-+-+-+
# Rust

## Starting a Project

To create a new Rust project, use Cargo:

```bash
cargo new project_name
cd project_name
```

## Building and Running

```bash
# Build the project
cargo build

# Run the project
cargo run

# Build for release (optimized)
cargo build --release
```

## Project Structure

```
project_name/
├── Cargo.toml      # Project manifest
├── Cargo.lock      # Dependency lock file
└── src/
    └── main.rs     # Entry point
```

## Common Commands

```bash
# Check code without building
cargo check

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Adding Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
