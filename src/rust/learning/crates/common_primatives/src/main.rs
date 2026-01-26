fn main() {
    println!("=== Common Primitives in Rust ===\n");

    // Boolean
    let is_active: bool = true;
    println!("Boolean: {}", is_active);

    // Integer types
    let signed_8bit: i8 = -127;
    let unsigned_8bit: u8 = 255;
    let signed_32bit: i32 = -2_147_483_648;
    let unsigned_32bit: u32 = 4_294_967_295;
    let signed_64bit: i64 = -9_223_372_036_854_775_808;
    println!("\nIntegers:");
    println!("  i8: {}", signed_8bit);
    println!("  u8: {}", unsigned_8bit);
    println!("  i32: {}", signed_32bit);
    println!("  u32: {}", unsigned_32bit);
    println!("  i64: {}", signed_64bit);

    // Floating point
    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045;
    println!("\nFloating Point:");
    println!("  f32: {}", float_32);
    println!("  f64: {}", float_64);

    // Character (Unicode scalar value)
    let letter: char = 'A';
    let emoji: char = '🦀';
    println!("\nCharacters:");
    println!("  char: {}", letter);
    println!("  emoji: {}", emoji);

    // Tuples
    let tuple: (i32, f64, char) = (42, 3.14, 'x');
    println!("\nTuple: {:?}", tuple);
    println!("  First element: {}", tuple.0);

    // Arrays (fixed size)
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("\nArray: {:?}", array);
    println!("  Length: {}", array.len());

    // String slice
    let string_slice: &str = "Hello, Rust!";
    println!("\nString slice: {}", string_slice);

    // String (heap-allocated)
    let mut string: String = String::from("Rust");
    string.push_str(" is awesome!");
    println!("String: {}", string);
}
