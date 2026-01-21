// Module System in Rust
// Demonstrates how to organize code with modules

// Module with nested structure
mod shapes {
    // Public module
    pub mod circle {
        pub fn area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }

        pub fn circumference(radius: f64) -> f64 {
            2.0 * std::f64::consts::PI * radius
        }
    }

    // Public module with private function
    pub mod rectangle {
        pub fn area(width: f64, height: f64) -> f64 {
            width * height
        }

        pub fn perimeter(width: f64, height: f64) -> f64 {
            2.0 * (width + height)
        }

        // Private function
        fn diagonal(width: f64, height: f64) -> f64 {
            (width * width + height * height).sqrt()
        }

        // Public function that uses private function
        pub fn get_diagonal(width: f64, height: f64) -> f64 {
            diagonal(width, height)
        }
    }

    // Private module
    mod triangle {
        pub fn area(base: f64, height: f64) -> f64 {
            0.5 * base * height
        }
    }
}

// Module with struct
mod geometry {
    #[derive(Debug)]
    pub struct Point {
        pub x: f64,
        pub y: f64,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }

        pub fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }
}

// Using 'use' to bring items into scope
use shapes::circle;
use shapes::rectangle::{area as rect_area, perimeter};
use geometry::Point;

fn main() {
    println!("=== Rust Module System ===\n");

    // Using modules with full path
    println!("--- Module Access ---");
    let radius = 5.0;
    println!("Circle area (full path): {:.2}", shapes::circle::area(radius));

    // Using modules with 'use'
    println!("Circle circumference (use): {:.2}", circle::circumference(radius));

    // Using renamed import
    let width = 10.0;
    let height = 5.0;
    println!("\nRectangle area (renamed): {:.2}", rect_area(width, height));
    println!("Rectangle perimeter: {:.2}", perimeter(width, height));
    println!("Rectangle diagonal: {:.2}", shapes::rectangle::get_diagonal(width, height));

    // Using struct from module
    println!("\n--- Module Structs ---");
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("Point 1: {:?}", p1);
    println!("Point 2: {:?}", p2);
    println!("Distance: {:.2}", p1.distance(&p2));

    // Demonstrating standard library imports
    println!("\n--- Standard Library Imports ---");
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    println!("HashMap: {:?}", map);

    // Multiple imports
    use std::fs;
    use std::io;
    use std::path::Path;
    println!("\nStandard library modules imported:");
    println!("- fs (file system)");
    println!("- io (input/output)");
    println!("- path::Path (path handling)");

    // Glob imports (use sparingly)
    use std::f64::consts::*;
    println!("\n--- Math Constants (glob import) ---");
    println!("PI: {}", PI);
    println!("E: {}", E);
    println!("TAU: {}", TAU);
}
