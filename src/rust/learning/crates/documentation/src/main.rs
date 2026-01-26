/// This is the main function that demonstrates documentation in Rust.
///
/// # Examples
///
/// ```
/// // Run with: cargo run -p documentation
/// ```
fn main() {
    println!("=== Documentation in Rust ===\n");
    println!("Run 'cargo doc --open -p documentation' to see generated docs.\n");

    let calc = Calculator::new();
    println!("add(5, 3) = {}", calc.add(5, 3));
    println!("subtract(10, 4) = {}", calc.subtract(10, 4));
    
    let user = User::new("Alice", 30);
    println!("\n{}", user.greet());
}

/// A simple calculator that performs basic arithmetic operations.
///
/// # Examples
///
/// ```
/// let calc = Calculator::new();
/// assert_eq!(calc.add(2, 3), 5);
/// ```
pub struct Calculator;

impl Calculator {
    /// Creates a new Calculator instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Calculator
    }

    /// Adds two numbers together.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number
    /// * `b` - The second number
    ///
    /// # Returns
    ///
    /// The sum of `a` and `b`
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(2, 3), 5);
    /// ```
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts the second number from the first.
    ///
    /// # Arguments
    ///
    /// * `a` - The number to subtract from
    /// * `b` - The number to subtract
    ///
    /// # Returns
    ///
    /// The difference of `a` and `b`
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

/// Represents a user with a name and age.
///
/// # Fields
///
/// * `name` - The user's name
/// * `age` - The user's age
pub struct User {
    name: String,
    age: u32,
}

impl User {
    /// Creates a new User.
    ///
    /// # Arguments
    ///
    /// * `name` - The user's name
    /// * `age` - The user's age
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new("Alice", 30);
    /// ```
    pub fn new(name: &str, age: u32) -> Self {
        User {
            name: name.to_string(),
            age,
        }
    }

    /// Returns a greeting message for the user.
    ///
    /// # Returns
    ///
    /// A String containing the greeting
    pub fn greet(&self) -> String {
        format!("Hello, I'm {} and I'm {} years old.", self.name, self.age)
    }
}
