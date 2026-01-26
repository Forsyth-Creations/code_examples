use log::{debug, error, info, trace, warn};

fn main() {
    // Initialize the logger
    env_logger::init();

    println!("=== Logging in Rust ===\n");
    println!("Set RUST_LOG environment variable to control log levels:");
    println!("  RUST_LOG=trace   - Shows all logs");
    println!("  RUST_LOG=debug   - Shows debug and above");
    println!("  RUST_LOG=info    - Shows info and above (default)");
    println!("  RUST_LOG=warn    - Shows warnings and errors");
    println!("  RUST_LOG=error   - Shows only errors\n");

    // Different log levels
    trace!("This is a TRACE message - very detailed");
    debug!("This is a DEBUG message - for debugging");
    info!("This is an INFO message - general information");
    warn!("This is a WARN message - warning!");
    error!("This is an ERROR message - something went wrong!");

    // Structured logging with format arguments
    let user = "Alice";
    let action = "login";
    info!("User '{}' performed action: {}", user, action);

    // Logging in functions
    perform_calculation(10, 5);
    perform_calculation(10, 0);

    println!("\nNote: Log messages go to stderr by default.");
    println!("Try running: RUST_LOG=debug cargo run -p logging");
}

fn perform_calculation(a: i32, b: i32) {
    debug!("Starting calculation with a={}, b={}", a, b);
    
    if b == 0 {
        error!("Cannot divide by zero!");
        return;
    }
    
    let result = a / b;
    info!("Calculation result: {} / {} = {}", a, b, result);
}
