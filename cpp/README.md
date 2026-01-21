# C++ Code Examples

This directory contains comprehensive C++ examples demonstrating fundamental concepts and language features.

## Examples

### 1. Primitives (`primitives.cpp`)
Demonstrates all basic primitive types in C++:
- Integer types (char, short, int, long, long long, signed/unsigned variants)
- Floating point types (float, double, long double)
- Boolean type
- String types (C-strings and std::string)
- Pointers and references
- Arrays (C-style and std::array)
- Vectors (dynamic arrays)
- Auto keyword (type inference)
- Type sizes and limits

**Compile and run:**
```bash
g++ -std=c++17 primitives.cpp -o primitives && ./primitives
```

### 2. Control Flow (`control_flow.cpp`)
Demonstrates various control flow mechanisms:
- If/else statements and ternary operator
- Traditional for loops
- Range-based for loops (C++11)
- While and do-while loops
- Break and continue
- Switch statements
- Switch with enums
- Nested loops
- If/switch with initialization (C++17)
- Exception handling
- Goto statement

**Compile and run:**
```bash
g++ -std=c++17 control_flow.cpp -o control_flow && ./control_flow
```

### 3. Classes (`classes.cpp`)
Demonstrates object-oriented programming:
- Structs vs classes
- Constructors and destructors
- Member functions and access specifiers
- Constructor overloading
- Operator overloading
- Inheritance and polymorphism
- Virtual functions
- Abstract classes (pure virtual functions)
- Static members
- Template classes
- Smart pointers (unique_ptr, shared_ptr)

**Compile and run:**
```bash
g++ -std=c++17 classes.cpp -o classes && ./classes
```

### 4. Enums (`enums.cpp`)
Demonstrates enumeration types:
- Traditional enums (unscoped)
- Enums with explicit values
- Scoped enums (enum class) - C++11
- Enums with underlying types
- Switch statements with enums
- Type safety benefits of enum class
- Using enums in classes

**Compile and run:**
```bash
g++ -std=c++17 enums.cpp -o enums && ./enums
```

### 5. Modules (`modules.cpp`)
Demonstrates the include system and standard library:
- Standard library includes
- Namespaces and namespace aliases
- Using declarations and directives
- Template functions
- STL containers (vector, map)
- STL algorithms
- Smart pointers
- File I/O
- String streams
- Lambdas with STL

**Compile and run:**
```bash
g++ -std=c++17 modules.cpp -o modules && ./modules
```

## Building and Running

### Prerequisites
- C++ compiler with C++17 support (g++, clang++)
- Standard C++ library

### Compile Individual Files
```bash
g++ -std=c++17 <filename>.cpp -o <output_name>
./<output_name>
```

### Using Docker
A Dockerfile is provided for running these examples in a containerized environment.

## Key C++ Concepts Demonstrated

### Object-Oriented Programming
C++ is a multi-paradigm language with strong OOP support:
- Classes and structs
- Inheritance and polymorphism
- Encapsulation with access specifiers
- Virtual functions for runtime polymorphism

### Memory Management
Manual and automatic memory management:
- Stack vs heap allocation
- Pointers and references
- RAII (Resource Acquisition Is Initialization)
- Smart pointers for automatic cleanup

### Templates
Compile-time polymorphism:
- Function templates
- Class templates
- Template specialization
- STL is built on templates

### Standard Template Library (STL)
Powerful collection of template classes and functions:
- Containers (vector, map, set, etc.)
- Algorithms (sort, find, transform, etc.)
- Iterators
- Function objects and lambdas

### Type Safety
Strong static typing with modern features:
- Type inference with auto
- Scoped enums (enum class)
- Explicit type conversions
- Templates for generic programming

## Learning Path

1. Start with **primitives.cpp** to understand basic types
2. Move to **control_flow.cpp** for program structure
3. Learn **classes.cpp** for object-oriented programming
4. Explore **enums.cpp** for type-safe constants
5. Study **modules.cpp** for code organization and STL

## Additional Resources

- [C++ Reference](https://en.cppreference.com/)
- [C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/)
- [Learn C++](https://www.learncpp.com/)
- [ISO C++ FAQ](https://isocpp.org/faq)
