//! Match is a great way to handle errors in Rust.

pub fn messing_with_errors() {
    println!("This is a function in the properly_erroring crate.");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // If we actually divide by 0, we can use unwrap_or to provide a default value
    let result = -10.0 / 0.0;
    println!("10 / 0 with unwrap_or = {}", result);

}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
