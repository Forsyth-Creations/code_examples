fn main() {
    println!("=== Common Logic Gates in Rust ===\n");

    let a = true;
    let b = false;

    // Basic logic gates
    println!("Inputs: a = {}, b = {}\n", a, b);

    println!("1. AND Gate:");
    println!("   a AND b = {}\n", and_gate(a, b));

    println!("2. OR Gate:");
    println!("   a OR b = {}\n", or_gate(a, b));

    println!("3. NOT Gate:");
    println!("   NOT a = {}", not_gate(a));
    println!("   NOT b = {}\n", not_gate(b));

    println!("4. NAND Gate:");
    println!("   a NAND b = {}\n", nand_gate(a, b));

    println!("5. NOR Gate:");
    println!("   a NOR b = {}\n", nor_gate(a, b));

    println!("6. XOR Gate (Exclusive OR):");
    println!("   a XOR b = {}\n", xor_gate(a, b));

    println!("7. XNOR Gate:");
    println!("   a XNOR b = {}\n", xnor_gate(a, b));

    // Truth table demonstration
    println!("Truth Tables:");
    print_truth_table();
}

fn and_gate(a: bool, b: bool) -> bool {
    a && b
}

fn or_gate(a: bool, b: bool) -> bool {
    a || b
}

fn not_gate(a: bool) -> bool {
    !a
}

fn nand_gate(a: bool, b: bool) -> bool {
    !(a && b)
}

fn nor_gate(a: bool, b: bool) -> bool {
    !(a || b)
}

fn xor_gate(a: bool, b: bool) -> bool {
    a ^ b
}

fn xnor_gate(a: bool, b: bool) -> bool {
    !(a ^ b)
}

fn print_truth_table() {
    println!("\n   A     | B     | AND   | OR    | NAND  | NOR   | XOR   | XNOR");
    println!("   ------|-------|-------|-------|-------|-------|-------|------");
    
    for a in [false, true] {
        for b in [false, true] {
            println!(
                "   {:<5} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} | {:<5} | {}",
                a,
                b,
                and_gate(a, b),
                or_gate(a, b),
                nand_gate(a, b),
                nor_gate(a, b),
                xor_gate(a, b),
                xnor_gate(a, b)
            );
        }
    }
}
