// Structs in Rust
// Demonstrates different types of structs and their usage

// Regular struct with named fields
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

// Tuple struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// Unit struct (no fields)
#[derive(Debug)]
struct Unit;

// Struct with methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method that borrows self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that borrows self mutably
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method that takes ownership of self
    fn into_square(self) -> Rectangle {
        let size = std::cmp::max(self.width, self.height);
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Generic struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

fn main() {
    println!("=== Rust Structs ===\n");

    // Creating a regular struct
    println!("--- Regular Struct ---");
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    println!("{:?}", person);
    println!("Name: {}, Age: {}", person.name, person.age);

    // Struct update syntax
    let person2 = Person {
        name: String::from("Bob"),
        ..person
    };
    println!("\n{:?}", person2);

    // Tuple struct
    println!("\n--- Tuple Struct ---");
    let color = Color(255, 0, 128);
    println!("{:?}", color);
    println!("RGB: ({}, {}, {})", color.0, color.1, color.2);

    // Unit struct
    println!("\n--- Unit Struct ---");
    let unit = Unit;
    println!("{:?}", unit);

    // Struct with methods
    println!("\n--- Struct with Methods ---");
    let mut rect = Rectangle::new(10, 20);
    println!("{:?}", rect);
    println!("Area: {}", rect.area());
    
    rect.scale(2);
    println!("After scaling: {:?}", rect);
    println!("New area: {}", rect.area());

    let square = rect.into_square();
    println!("Square: {:?}", square);

    // Generic struct
    println!("\n--- Generic Struct ---");
    let int_point = Point::new(5, 10);
    let float_point = Point::new(3.5, 7.2);
    println!("Integer point: {:?}", int_point);
    println!("Float point: {:?}", float_point);

    // Destructuring
    println!("\n--- Destructuring ---");
    let Person { name, age, .. } = person2;
    println!("Destructured - Name: {}, Age: {}", name, age);
}
