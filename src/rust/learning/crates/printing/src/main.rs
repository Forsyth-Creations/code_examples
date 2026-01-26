use console::style;

fn main() {
    println!("=== Printing and Formatting in Rust ===\n");

    // Basic printing
    println!("Basic println!");
    print!("print! does not add newline. ");
    println!("But println! does.\n");

    // Positional arguments
    println!("Positional: {0}, {1}, {0}", "first", "second");

    // Named arguments
    println!("Named: {name} is {age} years old", name = "Alice", age = 30);

    // Debug printing
    let vec = vec![1, 2, 3, 4, 5];
    println!("\nDebug: {:?}", vec);
    println!("Pretty Debug: {:#?}", vec);

    // Display formatting
    let number = 42;
    println!("\nFormatting numbers:");
    println!("  Decimal: {}", number);
    println!("  Binary: {:b}", number);
    println!("  Octal: {:o}", number);
    println!("  Hex (lowercase): {:x}", number);
    println!("  Hex (uppercase): {:X}", number);

    // Padding and alignment
    println!("\nAlignment and padding:");
    println!("  Left aligned: '{:<10}'", "text");
    println!("  Right aligned: '{:>10}'", "text");
    println!("  Center aligned: '{:^10}'", "text");
    println!("  Zero-padded: '{:05}'", 42);

    // Precision for floats
    let pi = std::f64::consts::PI;
    println!("\nFloat precision:");
    println!("  Default: {}", pi);
    println!("  2 decimals: {:.2}", pi);
    println!("  5 decimals: {:.5}", pi);

    // Error printing
    eprintln!("\nThis goes to stderr!");


    // ---------- You can also use the 'console' crate for styled output ----------
    println!("\nStyled output using 'console' crate:");
    println!("{} This is red text", style("Red").red());
    println!("{} This is green text", style("Green").green());
    println!("{} This is bold text", style("Bold").bold());
    println!("{} This is underlined text", style("Underlined").underlined());
}
