# Rust Code Examples

This directory contains comprehensive Rust examples demonstrating fundamental concepts and language features.

## Examples

### 1. Primitives (`primitives.rs`)
Demonstrates all basic primitive types in Rust:
- Integer types (i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize)
- Floating point types (f32, f64)
- Boolean type
- Character type (Unicode scalar values)
- String types (string slices &str and owned String)
- Arrays (fixed size)
- Tuples (fixed size, mixed types)
- Unit type

**Run:**
```bash
rustc primitives.rs && ./primitives
```

### 2. Control Flow (`control_flow.rs`)
Demonstrates various control flow mechanisms:
- If/else statements and expressions
- Loop (infinite loop with break)
- While loops
- For loops (with iterators and ranges)
- Match expressions (pattern matching)
- If let and while let (convenient pattern matching)
- Guards in match expressions

**Run:**
```bash
rustc control_flow.rs && ./control_flow
```

### 3. Structs (`structs.rs`)
Demonstrates struct types and their usage:
- Regular structs with named fields
- Tuple structs
- Unit structs
- Structs with methods (impl blocks)
- Associated functions (constructors)
- Generic structs
- Struct update syntax
- Destructuring

**Run:**
```bash
rustc structs.rs && ./structs
```

### 4. Enums (`enums.rs`)
Demonstrates enumeration types:
- Simple enums
- Enums with associated data
- Enums with methods
- Option enum (handling nullable values)
- Result enum (error handling)
- Generic enums
- Pattern matching with enums

**Run:**
```bash
rustc enums.rs && ./enums
```

### 5. Modules (`modules.rs`)
Demonstrates the module system and code organization:
- Module definitions and nested modules
- Public vs private items
- Using 'use' to bring items into scope
- Path navigation (absolute and relative)
- Importing from the standard library
- Module aliasing
- Glob imports

**Run:**
```bash
rustc modules.rs && ./modules
```

## Building and Running

### Prerequisites
- Rust toolchain (rustc and cargo)
- Install from: https://rustup.rs/

### Compile Individual Files
```bash
rustc <filename>.rs
./<filename>
```

### Using Docker
A Dockerfile is provided for running these examples in a containerized environment.

## Key Rust Concepts Demonstrated

### Ownership and Borrowing
Rust's ownership system ensures memory safety without garbage collection. These examples show:
- Owned types (String)
- Borrowed references (&str, &self)
- Mutable references (&mut self)

### Type Safety
Rust is strongly typed with excellent type inference:
- Explicit type annotations when needed
- Type inference for most variables
- Generic types for reusable code

### Pattern Matching
Powerful pattern matching with match expressions:
- Exhaustive matching
- Pattern destructuring
- Guards for complex conditions

### Zero-Cost Abstractions
High-level features with no runtime overhead:
- Generics (compile-time monomorphization)
- Trait-based polymorphism
- Iterator patterns

## Learning Path

1. Start with **primitives.rs** to understand basic types
2. Move to **control_flow.rs** for program structure
3. Learn **structs.rs** for custom data types
4. Explore **enums.rs** for powerful type variants
5. Study **modules.rs** for code organization

## Additional Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
