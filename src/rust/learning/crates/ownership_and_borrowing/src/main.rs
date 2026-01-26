fn main() {
    println!("=== Ownership and Borrowing in Rust ===\n");

    // Ownership - Move semantics
    println!("1. Ownership (Move Semantics):");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause a compile error
    println!("   s2 owns the string: {}\n", s2);

    // Clone to create a deep copy
    println!("2. Cloning:");
    let s3 = String::from("world");
    let s4 = s3.clone(); // Creates a deep copy
    println!("   s3: {}, s4: {}\n", s3, s4);

    // Borrowing - Immutable references
    println!("3. Immutable Borrowing:");
    let s5 = String::from("borrowed");
    let len = calculate_length(&s5); // Borrow s5
    println!("   The length of '{}' is {}.\n", s5, len);

    // Mutable borrowing
    println!("4. Mutable Borrowing:");
    let mut s6 = String::from("hello");
    change_string(&mut s6); // Mutable borrow
    println!("   Modified string: {}\n", s6);

    // Multiple immutable borrows are allowed
    println!("5. Multiple Immutable Borrows:");
    let s7 = String::from("multiple");
    let r1 = &s7;
    let r2 = &s7;
    println!("   r1: {}, r2: {}\n", r1, r2);

    // Borrowing rules demonstration
    println!("6. Borrowing Rules:");
    let mut s8 = String::from("rules");
    {
        let r3 = &mut s8;
        r3.push_str(" example");
    } // r3 goes out of scope here
    println!("   After mutable borrow: {}\n", s8);

    // Dangling references are prevented at compile time
    println!("7. Preventing Dangling References:");
    let reference = no_dangle();
    println!("   Valid reference: {}", reference);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s // Return ownership instead of reference
}
