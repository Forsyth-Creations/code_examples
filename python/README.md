# Python Code Examples

This directory contains comprehensive Python examples demonstrating fundamental concepts and language features.

## Examples

### 1. Primitives (`primitives.py`)
Demonstrates all basic primitive types in Python:
- Integer types (including arbitrary precision)
- Floating point types
- Complex numbers
- Boolean type
- String types (various formats)
- Bytes and bytearray
- None type
- Collections (list, tuple, set, dict)
- Type checking

**Run:**
```bash
python3 primitives.py
```

### 2. Control Flow (`control_flow.py`)
Demonstrates various control flow mechanisms:
- If/elif/else statements
- Ternary operator
- For loops (with list, range, enumerate)
- While loops
- Break and continue
- For-else and while-else
- Match statements (Python 3.10+)
- Comprehensions (list, dict, set)
- Exception handling (try/except/else/finally)
- With statement (context managers)

**Run:**
```bash
python3 control_flow.py
```

### 3. Classes (`classes.py`)
Demonstrates object-oriented programming:
- Simple classes with methods
- Properties (getters and setters)
- Class variables and methods
- Static methods
- Inheritance and polymorphism
- Multiple inheritance (mixins)
- Dataclasses
- Abstract base classes
- Special/magic methods (__str__, __add__, etc.)

**Run:**
```bash
python3 classes.py
```

### 4. Modules (`modules.py`)
Demonstrates the module system and imports:
- Standard library imports (os, sys, math, datetime, collections)
- Import with aliases
- Type hints (typing module)
- Module organization patterns
- __name__ variable usage
- Exploring modules with dir()

**Run:**
```bash
python3 modules.py
```

### 5. Design Patterns (`patterns.py`)
Demonstrates common design patterns and access patterns:
- **Singleton Pattern** - Single instance with global access (thread-safe metaclass and decorator)
- **Multition Pattern (Registry)** - Multiple named instances managed in a registry
- **Mediator Pattern** - Centralized communication between components
- **Factory Pattern** - Object creation without specifying exact classes
- **Builder Pattern** - Fluent API for step-by-step object construction
- **Observer Pattern** - Subscription mechanism for event notifications

**Run:**
```bash
python3 patterns.py
```

## Building and Running

### Prerequisites
- Python 3.7 or higher
- Install from: https://www.python.org/downloads/

### Run Individual Files
```bash
python3 <filename>.py
```

### Using Docker
A Dockerfile is provided for running these examples in a containerized environment.

## Key Python Concepts Demonstrated

### Dynamic Typing
Python uses dynamic typing with optional type hints:
- Variables don't need type declarations
- Type hints improve code documentation
- Runtime type checking available

### Object-Oriented Programming
Everything in Python is an object:
- Classes and inheritance
- Multiple inheritance support
- Magic methods for operator overloading
- Properties for controlled attribute access

### Comprehensions
Pythonic way to create collections:
- List comprehensions
- Dictionary comprehensions
- Set comprehensions
- Generator expressions

### Exception Handling
Robust error handling mechanism:
- Try/except blocks
- Multiple exception types
- Finally clause
- Custom exceptions

### Context Managers
Resource management with 'with' statement:
- Automatic cleanup
- File handling
- Lock management

## Learning Path

1. Start with **primitives.py** to understand basic types
2. Move to **control_flow.py** for program structure
3. Learn **classes.py** for object-oriented programming
4. Study **modules.py** for code organization
5. Master **patterns.py** for design patterns and best practices

## Additional Resources

- [Python Official Tutorial](https://docs.python.org/3/tutorial/)
- [Python Documentation](https://docs.python.org/3/)
- [Real Python Tutorials](https://realpython.com/)
- [PEP 8 Style Guide](https://pep8.org/)
