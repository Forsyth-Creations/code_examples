// Control Flow in Rust
// Demonstrates if statements, loops, and pattern matching

fn main() {
    println!("=== Rust Control Flow ===\n");

    // If statements
    println!("--- If Statements ---");
    let number = 42;
    
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is zero", number);
    }

    // If as an expression
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", number, result);

    // Loop (infinite loop)
    println!("\n--- Loop (infinite with break) ---");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("Breaking at counter = {}", counter);
            break;
        }
        println!("Counter: {}", counter);
    }

    // Loop with return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // While loop
    println!("\n--- While Loop ---");
    let mut countdown = 3;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // For loop
    println!("\n--- For Loop ---");
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Value: {}", element);
    }

    // For loop with range
    println!("\nFor with range:");
    for i in 0..5 {
        println!("Index: {}", i);
    }

    // For loop with inclusive range
    println!("\nFor with inclusive range:");
    for i in 1..=3 {
        println!("Number: {}", i);
    }

    // Match expression (pattern matching)
    println!("\n--- Match Expression ---");
    let number = 7;
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 => println!("This is a prime"),
        4 | 6 | 8 | 9 => println!("This is not prime"),
        _ => println!("Something else"),
    }

    // Match with ranges
    let age = 25;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        _ => println!("Senior"),
    }

    // Match with guards
    let number = 4;
    match number {
        n if n < 0 => println!("Negative"),
        n if n == 0 => println!("Zero"),
        n if n % 2 == 0 => println!("Even positive"),
        _ => println!("Odd positive"),
    }

    // Match as an expression
    let result = match number {
        1 => "one",
        2 => "two",
        _ => "many",
    };
    println!("Result: {}", result);

    // If let (convenient pattern matching)
    println!("\n--- If Let ---");
    let maybe_value = Some(7);
    if let Some(x) = maybe_value {
        println!("Got a value: {}", x);
    }

    // While let
    println!("\n--- While Let ---");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
