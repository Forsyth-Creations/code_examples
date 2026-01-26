fn main() {
    println!("=== Cargo Features in Rust ===\n");

    println!("Basic functionality is always available.");
    basic_function();

    #[cfg(feature = "advanced")]
    {
        println!("\nAdvanced feature is enabled!");
        advanced_function();
    }

    #[cfg(not(feature = "advanced"))]
    {
        println!("\nAdvanced feature is NOT enabled.");
        println!("  Run with: cargo run -p features --features advanced");
    }

    #[cfg(feature = "experimental")]
    {
        println!("\nExperimental feature is enabled!");
        experimental_function();
    }

    #[cfg(not(feature = "experimental"))]
    {
        println!("\nExperimental feature is NOT enabled.");
        println!("  Run with: cargo run -p features --features experimental");
    }

    println!("\nYou can enable multiple features:");
    println!("  cargo run -p features --features advanced,experimental");
    println!("  cargo run -p features --all-features");
}

fn basic_function() {
    println!("  This is the basic function.");
}

#[cfg(feature = "advanced")]
fn advanced_function() {
    println!("  This is an advanced function.");
    println!("  It provides extra capabilities.");
}

#[cfg(feature = "experimental")]
fn experimental_function() {
    println!("  This is an experimental function.");
    println!("  Use at your own risk!");
}
