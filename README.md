# Code Examples Repository

A comprehensive collection of programming language examples demonstrating fundamental concepts and best practices. This repository contains detailed code samples for **Rust**, **Python**, and **C++**, covering essential programming concepts that are common across these languages.

## Overview

Each language directory contains well-documented examples demonstrating:
- **Primitive types and data structures**
- **Control flow** (if statements, loops, pattern matching)
- **Object-oriented programming** (classes, structs, inheritance)
- **Enumerations** and type definitions
- **Module systems** and code organization
- **Language-specific features** and idioms

## Languages

### 🦀 Rust
Modern systems programming language focused on safety, speed, and concurrency.

**Location:** [`rust/`](rust/)

**Examples:**
- `primitives.rs` - All basic types (integers, floats, strings, arrays, tuples)
- `control_flow.rs` - If/else, loops, match expressions, pattern matching
- `structs.rs` - Regular structs, tuple structs, methods, generics
- `enums.rs` - Simple enums, enums with data, Option, Result
- `modules.rs` - Module organization, imports, visibility

**Key Features:**
- Ownership and borrowing system
- Zero-cost abstractions
- Powerful pattern matching
- Type safety without garbage collection

[Read Rust examples documentation →](rust/README.md)

### 🐍 Python
High-level, interpreted language known for readability and versatility.

**Location:** [`python/`](python/)

**Examples:**
- `primitives.py` - All basic types (int, float, string, list, dict, set)
- `control_flow.py` - If/else, loops, comprehensions, exception handling
- `classes.py` - Classes, inheritance, dataclasses, abstract classes
- `modules.py` - Imports, standard library, type hints

**Key Features:**
- Dynamic typing with optional type hints
- Extensive standard library
- Multiple programming paradigms
- Comprehensions and generators

[Read Python examples documentation →](python/README.md)

### ⚡ C++
Powerful, compiled language for performance-critical applications.

**Location:** [`cpp/`](cpp/)

**Examples:**
- `primitives.cpp` - All basic types (int, float, pointers, arrays, vectors)
- `control_flow.cpp` - If/else, loops, switch, exception handling
- `classes.cpp` - Classes, inheritance, templates, smart pointers
- `enums.cpp` - Enums, scoped enums (enum class)
- `modules.cpp` - Includes, namespaces, STL

**Key Features:**
- Low-level memory control
- Object-oriented and generic programming
- Standard Template Library (STL)
- Zero-overhead principle

[Read C++ examples documentation →](cpp/README.md)

## Getting Started

### Running Examples Locally

Each language directory contains standalone examples that can be run independently.

#### Rust
```bash
cd rust
rustc primitives.rs && ./primitives
rustc control_flow.rs && ./control_flow
# ... etc
```

#### Python
```bash
cd python
python3 primitives.py
python3 control_flow.py
# ... etc
```

#### C++
```bash
cd cpp
g++ -std=c++17 primitives.cpp -o primitives && ./primitives
g++ -std=c++17 control_flow.cpp -o control_flow && ./control_flow
# ... etc
```

### Using Docker

Each language directory includes a Dockerfile for running examples in a containerized environment:

```bash
# Rust
cd rust
docker build -t rust-examples .
docker run rust-examples

# Python
cd python
docker build -t python-examples .
docker run python-examples

# C++
cd cpp
docker build -t cpp-examples .
docker run cpp-examples
```

## Concepts Covered

### Common Concepts Across All Languages

1. **Primitive Types**
   - Numbers (integers, floating point)
   - Strings and characters
   - Booleans
   - Collections (arrays, vectors, lists)

2. **Control Flow**
   - Conditional statements (if/else)
   - Loops (for, while)
   - Pattern matching / switch statements
   - Error handling

3. **Code Organization**
   - Functions and methods
   - Modules and namespaces
   - Imports and includes
   - Code reusability

4. **Data Structures**
   - Structs/classes
   - Enumerations
   - Inheritance (where applicable)
   - Generics/templates

### Language-Specific Features

- **Rust:** Ownership, borrowing, lifetimes, trait system
- **Python:** Duck typing, decorators, comprehensions, context managers
- **C++:** Manual memory management, RAII, STL, operator overloading

## Learning Path

For beginners learning programming concepts:

1. **Start with primitives** - Understand basic data types in each language
2. **Move to control flow** - Learn how to direct program execution
3. **Explore data structures** - Learn to organize data with structs/classes
4. **Study enumerations** - Understand type-safe constants and variants
5. **Master modules** - Learn to organize larger programs

For those learning a specific language:

1. Read the language-specific README in each directory
2. Run examples and experiment with modifications
3. Compare with other languages to understand design choices
4. Study language-specific features and idioms

## Prerequisites

### For Rust
- Rust toolchain: https://rustup.rs/
- `rustc` compiler and `cargo` build tool

### For Python
- Python 3.7+: https://www.python.org/downloads/
- Standard library (included with Python)

### For C++
- C++ compiler with C++17 support (g++, clang++)
- Standard C++ library

### For Docker (Optional)
- Docker: https://www.docker.com/get-started

## Contributing

This is an educational repository demonstrating fundamental programming concepts. Each example is thoroughly commented and designed to be:

- **Self-contained** - Runnable independently
- **Well-documented** - Clear comments and README files
- **Beginner-friendly** - Suitable for those learning programming
- **Comprehensive** - Covers the full range of basic concepts

## License

This repository is provided as educational material for learning programming concepts across different languages.

## Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Documentation](https://doc.rust-lang.org/)

### Python
- [Python Official Tutorial](https://docs.python.org/3/tutorial/)
- [Python Documentation](https://docs.python.org/3/)
- [Real Python](https://realpython.com/)

### C++
- [C++ Reference](https://en.cppreference.com/)
- [Learn C++](https://www.learncpp.com/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)
