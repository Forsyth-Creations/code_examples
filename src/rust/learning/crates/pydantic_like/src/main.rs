use anyhow::Result;
use serde::Deserialize;
use validator::Validate;
use console::style;

/// Pydantic-like model
#[derive(Debug, Deserialize, Validate)]
struct User {
    name: String,

    #[validate(range(min = 1))]
    age: i32,

    #[validate(email)]
    email: String,
}

impl User {
    /// Example constructor (acts like a test fixture or demo input)
    fn example() -> Self {
        Self {
            name: "Alice".to_string(),
            age: 30,
            email: "alice@example.com".to_string(),
        }
    }

    /// Custom validation method wrapper
    fn validate_user(&self) -> Result<()> {
        self.validate().map_err(|e| anyhow::anyhow!(e))
    }

    fn print_info(&self) {
        println!("User Info:");
        println!("  Name: {}", self.name);
        println!("  Age: {}", self.age);
        println!("  Email: {}", self.email);
    }
}

fn main() -> Result<()> {
    // Equivalent to: user = User(...)
    let user = User::example();

    // Equivalent to: user.validate()
    match user.validate_user() {
        Ok(_) => println!("{}", style("✅ User validated successfully").green()),
        Err(e) => {
            println!("{} {}", style("❌ Validation errors:").red(), e);
            return Ok(());
        }
    }

    println!("Validated example model:");
    println!("{:#?}", user);
    user.print_info();

    // An unvalidated user
    let invalid_user = User {
        name: "Bob".to_string(),
        age: 0, // Invalid age
        email: "invalid-email".to_string(), // Invalid email
    };

    // Attempt validation
    match invalid_user.validate_user() {
        Ok(_) => println!("{}", style("❌ Invalid user passed validation (unexpected)").red()),
        Err(e) => println!("{} {}", style("❌ Validation errors for invalid user:").red(), e),
    }

    Ok(())
}