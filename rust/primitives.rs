// Primitive Types in Rust
// Demonstrates all basic primitive types and their usage

fn main() {
    println!("=== Rust Primitive Types ===\n");

    // Integer types
    println!("--- Integer Types ---");
    let signed_8: i8 = -128;
    let unsigned_8: u8 = 255;
    let signed_16: i16 = -32768;
    let unsigned_16: u16 = 65535;
    let signed_32: i32 = -2147483648;
    let unsigned_32: u32 = 4294967295;
    let signed_64: i64 = -9223372036854775808;
    let unsigned_64: u64 = 18446744073709551615;
    let signed_128: i128 = -170141183460469231731687303715884105728;
    let unsigned_128: u128 = 340282366920938463463374607431768211455;
    let arch_signed: isize = -2147483648; // size depends on architecture
    let arch_unsigned: usize = 4294967295;

    println!("i8: {}", signed_8);
    println!("u8: {}", unsigned_8);
    println!("i32: {}", signed_32);
    println!("u64: {}", unsigned_64);
    println!("usize: {}", arch_unsigned);

    // Floating point types
    println!("\n--- Floating Point Types ---");
    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045;
    println!("f32: {}", float_32);
    println!("f64: {}", float_64);

    // Boolean type
    println!("\n--- Boolean Type ---");
    let is_true: bool = true;
    let is_false: bool = false;
    println!("true: {}", is_true);
    println!("false: {}", is_false);

    // Character type (Unicode scalar value)
    println!("\n--- Character Type ---");
    let letter: char = 'A';
    let emoji: char = '🦀';
    let chinese: char = '中';
    println!("ASCII char: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Unicode: {}", chinese);

    // String types
    println!("\n--- String Types ---");
    let string_slice: &str = "Hello, Rust!"; // string slice (immutable)
    let owned_string: String = String::from("Hello, World!"); // owned string
    println!("String slice: {}", string_slice);
    println!("Owned String: {}", owned_string);

    // Array (fixed size)
    println!("\n--- Array Type ---");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    println!("First element: {}", numbers[0]);

    // Tuple (fixed size, mixed types)
    println!("\n--- Tuple Type ---");
    let tuple: (i32, f64, char) = (42, 3.14, 'x');
    println!("Tuple: {:?}", tuple);
    println!("First: {}, Second: {}, Third: {}", tuple.0, tuple.1, tuple.2);

    // Unit type (empty tuple)
    println!("\n--- Unit Type ---");
    let unit: () = ();
    println!("Unit type (empty): {:?}", unit);
}
