use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║          Rust Learning Examples - Interactive Menu            ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    loop {
        display_menu();
        
        print!("\nEnter your choice (or 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        if choice.eq_ignore_ascii_case("q") {
            println!("\nGoodbye!");
            break;
        }

        match choice {
            "1" => run_example("common_primatives", "Common Primitives"),
            "2" => run_example("printing", "Printing and Formatting"),
            "3" => run_example("ownership_and_borrowing", "Ownership and Borrowing"),
            "4" => run_example("traits_and_attributes", "Traits and Attributes"),
            "5" => run_example("properly_erroring", "Error Handling"),
            "6" => run_example("common_logic_gates", "Logic Gates"),
            "7" => run_example("testing", "Testing"),
            "8" => run_example_with_env("logging", "Logging", "RUST_LOG", "info"),
            "9" => run_example("parallelism", "Parallelism"),
            "10" => run_example("singleton_example", "Singleton Pattern"),
            "11" => run_example("performace_awareness", "Performance Awareness"),
            "12" => run_example("documentation", "Documentation"),
            "13" => run_cli_example(),
            "14" => run_features_example(),
            "15" => run_example("pydantic_like", "Pydantic-like Validation"),
            "0" => run_all_examples(),
            _ => println!("\n❌ Invalid choice. Please try again."),
        }

        println!("\n{}", "─".repeat(70));
    }
}

fn display_menu() {
    println!("\n📚 Available Examples:");
    println!("─────────────────────────────────────────────────────────────────");
    println!("  1.  Common Primitives           - Basic data types");
    println!("  2.  Printing & Formatting       - Console output");
    println!("  3.  Ownership & Borrowing       - Memory management");
    println!("  4.  Traits & Attributes         - Interfaces and metadata");
    println!("  5.  Error Handling              - Result and Option types");
    println!("  6.  Logic Gates                 - Boolean operations");
    println!("  7.  Testing                     - Unit tests");
    println!("  8.  Logging                     - Log levels and output");
    println!("  9.  Parallelism                 - Multi-threading");
    println!(" 10.  Singleton Pattern           - Global state");
    println!(" 11.  Performance Awareness       - Optimization tips");
    println!(" 12.  Documentation               - Doc comments");
    println!(" 13.  CLI Arguments               - Command-line parsing");
    println!(" 14.  Cargo Features              - Conditional compilation");
    println!(" 15.  Pydantic-like Validation    - Data validation example");
    println!("─────────────────────────────────────────────────────────────────");
    println!("  0.  Run ALL examples");
    println!("  q.  Quit");
}

fn run_example(package: &str, name: &str) {
    println!("\n🚀 Running: {}", name);
    println!("{}", "═".repeat(70));
    
    let status = Command::new("cargo")
        .args(["run", "-p", package, "--quiet"])
        .status();
    
    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("❌ Example failed with exit code: {:?}", exit_status.code());
            }
        }
        Err(e) => println!("❌ Failed to run example: {}", e),
    }
}

fn run_example_with_env(package: &str, name: &str, env_var: &str, env_value: &str) {
    println!("\n🚀 Running: {} (with {}={})", name, env_var, env_value);
    println!("{}", "═".repeat(70));
    
    let status = Command::new("cargo")
        .args(["run", "-p", package, "--quiet"])
        .env(env_var, env_value)
        .status();
    
    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("❌ Example failed with exit code: {:?}", exit_status.code());
            }
        }
        Err(e) => println!("❌ Failed to run example: {}", e),
    }
}

fn run_cli_example() {
    println!("\n🚀 Running: CLI Arguments");
    println!("{}", "═".repeat(70));
    println!("Running with default arguments...\n");
    
    let _ = Command::new("cargo")
        .args(["run", "-p", "build_config_cli", "--quiet"])
        .status();
    
    println!("\nRunning with custom arguments...\n");
    
    let _ = Command::new("cargo")
        .args(["run", "-p", "build_config_cli", "--quiet", "--", 
               "--name", "Rust", "--count", "2", "--verbose"])
        .status();
}

fn run_features_example() {
    println!("\n🚀 Running: Cargo Features");
    println!("{}", "═".repeat(70));
    println!("1. Running with default features...\n");
    
    let _ = Command::new("cargo")
        .args(["run", "-p", "features", "--quiet"])
        .status();
    
    println!("\n2. Running with 'advanced' feature...\n");
    
    let _ = Command::new("cargo")
        .args(["run", "-p", "features", "--quiet", "--features", "advanced"])
        .status();
    
    println!("\n3. Running with all features...\n");
    
    let _ = Command::new("cargo")
        .args(["run", "-p", "features", "--quiet", "--all-features"])
        .status();
}

fn run_all_examples() {
    println!("\n🎯 Running ALL Examples");
    println!("{}", "═".repeat(70));
    
    let examples = vec![
        ("common_primatives", "Common Primitives"),
        ("printing", "Printing and Formatting"),
        ("ownership_and_borrowing", "Ownership and Borrowing"),
        ("traits_and_attributes", "Traits and Attributes"),
        ("properly_erroring", "Error Handling"),
        ("common_logic_gates", "Logic Gates"),
        ("testing", "Testing"),
        ("parallelism", "Parallelism"),
        ("singleton_example", "Singleton Pattern"),
        ("performace_awareness", "Performance Awareness"),
        ("documentation", "Documentation"),
    ];
    
    for (package, name) in examples {
        run_example(package, name);
        println!();
    }
    
    // Special examples with custom execution
    run_example_with_env("logging", "Logging", "RUST_LOG", "info");
    println!();
    
    run_cli_example();
    println!();
    
    run_features_example();
    
    println!("\n✅ All examples completed!");
}
