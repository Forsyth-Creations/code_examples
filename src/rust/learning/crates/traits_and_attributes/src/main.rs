fn main() {
    println!("=== Traits and Attributes in Rust ===\n");

    // Using traits
    println!("1. Traits:");
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };
    
    println!("   {}", dog.speak());
    println!("   {}", cat.speak());
    println!();

    // Trait bounds
    println!("2. Trait Bounds:");
    print_animal_sound(&dog);
    print_animal_sound(&cat);
    println!();

    // Derive attributes
    println!("3. Derive Attributes:");
    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 10, y: 20 };
    let point3 = point1.clone();
    
    println!("   point1: {:?}", point1);
    println!("   point2: {:?}", point2);
    println!("   point1 == point2: {}", point1 == point2);
    println!("   cloned point3: {:?}", point3);
    println!();

    // Custom attributes
    println!("4. Other Common Attributes:");
    deprecated_function();
    let _unused_value = 42; // #[allow(dead_code)] on variables
}

// Define a trait
trait Animal {
    fn speak(&self) -> String;
}

// Implement trait for Dog
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

// Implement trait for Cat
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

// Function with trait bound
fn print_animal_sound<T: Animal>(animal: &T) {
    println!("   {}", animal.speak());
}

// Struct with derive attributes
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Deprecated attribute
#[deprecated(since = "0.1.0", note = "use new_function instead")]
fn deprecated_function() {
    println!("   This function is deprecated!");
}
