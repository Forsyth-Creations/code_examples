fn main() {
    println!("=== Testing in Rust ===\n");
    println!("This crate demonstrates testing in Rust.");
    println!("Run 'cargo test -p testing' to see the tests in action.\n");
    
    // Some functions to test
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 5), 0);
        assert_eq!(multiply(-2, 3), -6);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This test is supposed to panic");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        assert_eq!(1 + 1, 2);
    }
}
