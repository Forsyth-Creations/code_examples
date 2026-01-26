use lazy_static::lazy_static;
use std::sync::Mutex;

// Singleton pattern using lazy_static
lazy_static! {
    static ref COUNTER: Mutex<Counter> = Mutex::new(Counter::new());
}

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

// Another singleton example - Configuration
lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

#[derive(Debug)]
struct Config {
    app_name: String,
    version: String,
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: String::from("MyApp"),
            version: String::from("1.0.0"),
            debug: false,
        }
    }
}

fn main() {
    println!("=== Singleton Pattern in Rust ===\n");

    // Using the Counter singleton
    println!("1. Counter Singleton:");
    println!("   Initial value: {}", COUNTER.lock().unwrap().get_value());
    
    COUNTER.lock().unwrap().increment();
    println!("   After increment: {}", COUNTER.lock().unwrap().get_value());
    
    COUNTER.lock().unwrap().increment();
    println!("   After another increment: {}", COUNTER.lock().unwrap().get_value());
    println!();

    // Using the Config singleton
    println!("2. Config Singleton:");
    {
        let config = CONFIG.lock().unwrap();
        println!("   App Name: {}", config.app_name);
        println!("   Version: {}", config.version);
        println!("   Debug Mode: {}", config.debug);
    }
    println!();

    // Modifying config
    println!("3. Modifying Singleton:");
    {
        let mut config = CONFIG.lock().unwrap();
        config.debug = true;
        config.app_name = String::from("UpdatedApp");
    }
    
    {
        let config = CONFIG.lock().unwrap();
        println!("   Updated App Name: {}", config.app_name);
        println!("   Updated Debug Mode: {}", config.debug);
    }
    
    println!("\nNote: Singletons are shared across the entire program!");
}
