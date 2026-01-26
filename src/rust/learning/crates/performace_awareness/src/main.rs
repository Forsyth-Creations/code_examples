use std::time::Instant;

fn main() {
    println!("=== Performance Awareness in Rust ===\n");

    // 1. Avoid unnecessary allocations
    println!("1. String Allocation:");
    avoid_unnecessary_allocations();
    println!();

    // 2. Use iterators instead of collecting
    println!("2. Iterator vs. Collection:");
    iterator_vs_collection();
    println!();

    // 3. Measure performance
    println!("3. Measuring Performance:");
    measure_performance();
    println!();

    // 4. Stack vs Heap allocation
    println!("4. Stack vs Heap:");
    stack_vs_heap();
    println!();

    // 5. Inline functions
    println!("5. Inline Functions:");
    let result = expensive_calculation(10);
    println!("   Result: {}", result);
}

fn avoid_unnecessary_allocations() {
    // Bad: Creates new String
    fn get_greeting_bad() -> String {
        "Hello".to_string()
    }
    
    // Good: Uses string slice
    fn get_greeting_good() -> &'static str {
        "Hello"
    }
    
    let _greeting1 = get_greeting_bad();
    let _greeting2 = get_greeting_good();
    println!("   Using &str is more efficient than String for constants");
}

fn iterator_vs_collection() {
    let numbers: Vec<i32> = (1..=1000).collect();
    
    // Less efficient: collects into intermediate Vec
    let _sum_bad: i32 = numbers
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    
    // More efficient: uses iterators throughout
    let sum_good: i32 = numbers
        .iter()
        .map(|x| x * 2)
        .sum();
    
    println!("   Using iterators without intermediate collections: {}", sum_good);
}

fn measure_performance() {
    let start = Instant::now();
    
    // Some work
    let mut sum = 0;
    for i in 1..=1_000_000 {
        sum += i;
    }
    
    let duration = start.elapsed();
    println!("   Sum: {}", sum);
    println!("   Time taken: {:?}", duration);
}

fn stack_vs_heap() {
    // Stack allocation (fast)
    let _array: [i32; 100] = [0; 100];
    println!("   Fixed-size arrays are allocated on the stack (fast)");
    
    // Heap allocation (slower but flexible)
    let _vec: Vec<i32> = vec![0; 100];
    println!("   Vectors are allocated on the heap (flexible size)");
}

// #[inline] suggests compiler to inline this function
#[inline]
fn expensive_calculation(x: i32) -> i32 {
    x * x + 2 * x + 1
}
