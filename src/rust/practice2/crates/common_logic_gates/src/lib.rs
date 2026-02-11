pub fn show_some_gates() {
    let (a, b) = (true, false);

    println!("AND Gate: {} AND {} = {}\n", a, b, and_gate(a, b) );

}

fn and_gate(a : bool, b: bool) -> bool {
    return a && b;
}