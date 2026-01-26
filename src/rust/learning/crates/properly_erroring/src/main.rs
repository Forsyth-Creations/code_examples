use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Proper Error Handling in Rust ===\n");

    // Using Result with match
    println!("1. Basic Result handling with match:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}\n", e),
    }

    // Using unwrap_or
    println!("2. Using unwrap_or for defaults:");
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("   Division with default: {}\n", result);

    // Using the ? operator
    println!("3. Using ? operator:");
    match read_username_from_file() {
        Ok(name) => println!("   Username: {}", name),
        Err(e) => println!("   Error reading file: {}\n", e),
    }

    // Option type
    println!("4. Option type:");
    let numbers = vec![1, 2, 3, 4, 5];
    match find_value(&numbers, 3) {
        Some(index) => println!("   Found at index: {}", index),
        None => println!("   Not found"),
    }
    match find_value(&numbers, 10) {
        Some(index) => println!("   Found at index: {}", index),
        None => println!("   Not found\n"),
    }

    // Custom error types
    println!("5. Custom error types:");
    match validate_age(25) {
        Ok(_) => println!("   Age is valid"),
        Err(e) => println!("   Error: {}", e),
    }
    match validate_age(-5) {
        Ok(_) => println!("   Age is valid"),
        Err(e) => println!("   Error: {}", e),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // This will fail since the file doesn't exist, demonstrating error handling
    let mut file = match File::open("username.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn find_value(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

#[derive(Debug)]
enum AgeError {
    Negative,
    TooOld,
}

impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AgeError::Negative => write!(f, "Age cannot be negative"),
            AgeError::TooOld => write!(f, "Age is too old (max 150)"),
        }
    }
}

fn validate_age(age: i32) -> Result<(), AgeError> {
    if age < 0 {
        Err(AgeError::Negative)
    } else if age > 150 {
        Err(AgeError::TooOld)
    } else {
        Ok(())
    }
}
