use std::thread;
use std::time::Duration;
use rayon::prelude::*;

fn main() {
    println!("=== Parallelism in Rust ===\n");

    // 1. Basic threading
    println!("1. Basic Threading:");
    basic_threading();
    println!();

    // 2. Thread with return value
    println!("2. Thread with Return Value:");
    thread_with_return();
    println!();

    // 3. Parallel iteration with Rayon
    println!("3. Parallel Iteration (Rayon):");
    parallel_iteration();
    println!();

    // 4. Parallel map
    println!("4. Parallel Map:");
    parallel_map();
    println!();

    // 5. Parallel sum
    println!("5. Parallel Sum:");
    parallel_sum();
}

fn basic_threading() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                println!("   Thread {} starting", i);
                thread::sleep(Duration::from_millis(100));
                println!("   Thread {} finished", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn thread_with_return() {
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=10 {
            sum += i;
        }
        sum
    });

    let result = handle.join().unwrap();
    println!("   Sum from 1 to 10: {}", result);
}

fn parallel_iteration() {
    let numbers: Vec<i32> = (1..=10).collect();
    
    println!("   Processing numbers in parallel:");
    numbers.par_iter().for_each(|&n| {
        let square = n * n;
        println!("   {} squared = {}", n, square);
    });
}

fn parallel_map() {
    let numbers: Vec<i32> = (1..=10).collect();
    
    let squares: Vec<i32> = numbers
        .par_iter()
        .map(|&n| n * n)
        .collect();
    
    println!("   Original: {:?}", numbers);
    println!("   Squared: {:?}", squares);
}

fn parallel_sum() {
    let numbers: Vec<i32> = (1..=100).collect();
    
    let sum: i32 = numbers.par_iter().sum();
    
    println!("   Sum of 1 to 100: {}", sum);
}
