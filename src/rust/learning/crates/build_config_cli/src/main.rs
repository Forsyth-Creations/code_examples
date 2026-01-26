use clap::Parser;

/// A simple CLI application demonstrating command-line argument parsing
#[derive(Parser, Debug)]
#[command(name = "build_config_cli")]
#[command(about = "Demonstrates CLI configuration in Rust", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: Option<String>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Enable verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Configuration file path
    #[arg(short = 'f', long)]
    config: Option<String>,
}

fn main() {
    println!("=== Build Config & CLI in Rust ===\n");

    let args = Args::parse();

    if args.verbose {
        println!("Verbose mode enabled!");
        println!("Debug info: {:?}\n", args);
    }

    let name = args.name.unwrap_or_else(|| String::from("World"));

    for i in 1..=args.count {
        println!("{}. Hello, {}!", i, name);
    }

    if let Some(config_path) = args.config {
        println!("\nUsing config file: {}", config_path);
    }

    println!("\nTry running with arguments:");
    println!("  cargo run -p build_config_cli -- --name Alice --count 3");
    println!("  cargo run -p build_config_cli -- -n Bob -c 2 -v");
    println!("  cargo run -p build_config_cli -- --help");
}
