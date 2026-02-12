use common_primatives::add;
use printing::common_prints;

fn main() {
    println!("Hello, world!");

    let (a, b) = (5, 4);
    let c = (5, 4);
    let emoji = "😀";
    
    println!("{}", add(a, b));

    // Rust doesn't have this concept of spreading 
    // C into a and b, but we can destructure the tuple to achieve the same result
    // println!("{}", add(**c));

    let (d, e) = c;
    println!("{}", add(d, e));

    println!("Emoji: {}", emoji);

    // -------------------------------------------------------------------------
    common_logic_gates::show_some_gates();
    common_prints::say_hello_blue();
    properly_erroring::messing_with_errors();
    traits_and_attributes::playing_with_traits();
    borrowing::testing_borrowing();
}
