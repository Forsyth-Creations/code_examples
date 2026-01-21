// Enums in Rust
// Demonstrates enums, pattern matching, and Option/Result types

// Simple enum
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with associated data
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

// Enum with methods
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

// Generic enum example
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    println!("=== Rust Enums ===\n");

    // Simple enum
    println!("--- Simple Enum ---");
    let direction = Direction::North;
    println!("Direction: {:?}", direction);

    match direction {
        Direction::North => println!("Heading north!"),
        Direction::South => println!("Heading south!"),
        Direction::East => println!("Heading east!"),
        Direction::West => println!("Heading west!"),
    }

    // Enum with associated data
    println!("\n--- Enum with Data ---");
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in messages {
        process_message(msg);
    }

    // Enum with methods
    println!("\n--- Enum with Methods ---");
    let light = TrafficLight::Red;
    println!("Current light: {:?}", light);
    println!("Time: {} seconds", light.time());
    
    let next_light = light.next();
    println!("Next light: {:?}", next_light);

    // Option enum (built-in)
    println!("\n--- Option Enum ---");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Pattern matching with Option
    match some_number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("Got nothing"),
    }

    // Using Option methods
    let value = some_number.unwrap_or(0);
    println!("Value with default: {}", value);

    if let Some(n) = some_number {
        println!("Using if let: {}", n);
    }

    // Result enum (built-in)
    println!("\n--- Result Enum ---");
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("Something went wrong"));

    println!("success: {:?}", success);
    println!("failure: {:?}", failure);

    match success {
        Ok(value) => println!("Success with value: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // Using Result methods
    let value = failure.unwrap_or(-1);
    println!("Value from failure with default: {}", value);

    // Generic enum
    println!("\n--- Generic Enum ---");
    let left: Either<i32, String> = Either::Left(42);
    let right: Either<i32, String> = Either::Right(String::from("hello"));

    println!("left: {:?}", left);
    println!("right: {:?}", right);

    match left {
        Either::Left(n) => println!("Left value: {}", n),
        Either::Right(s) => println!("Right value: {}", s),
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
