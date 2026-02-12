pub fn testing_borrowing() {
    println!("=== Ownership and Borrowing in Rust ===\n");

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    println!("{}", s3);

    // println!("{}", s1); // s1 is now owned by s3, so it can't be used independently

    println!("========== Cloning ==========");
    let s4 = s3.clone() + &s3;

    // With deep copies, both should be available
    println!("{}", s4);
    println!("{}", s3);

    let s3 = String::from("hello");

    println!("{}", s4);
    println!("{}", s3);

    println!("========== Shadowing ==========");

    // Notice how the type changes over the course of the shadowing

    let input = "42";
    let input = input.parse::<i32>().expect("Not a number");
    let input = input * 2;
    println!("{}", input);
}

// A shared access example using arc and mutex

pub mod async_clock {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::{Duration, sleep};

    #[derive(Clone)]
    pub struct AsyncClock {
        time: Arc<Mutex<u64>>,
    }

    impl AsyncClock {
        /// Create a new clock starting at 0
        pub fn new() -> Self {
            Self {
                time: Arc::new(Mutex::new(0)),
            }
        }

        /// Start the ticking task (spawns background updater)
        pub fn start(&self) {
            let time_clone = Arc::clone(&self.time);

            tokio::spawn(async move {
                loop {
                    sleep(Duration::from_secs(1)).await;

                    let mut t = time_clone.lock().await;
                    *t += 1;
                }
            });
        }

        /// Get current time
        pub async fn get(&self) -> u64 {
            *self.time.lock().await
        }

        /// Manually set time (optional)
        pub async fn set(&self, value: u64) {
            let mut t = self.time.lock().await;
            *t = value;
        }
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let clock = async_clock::AsyncClock::new();

    clock.start();

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    let current = clock.get().await;
    println!("Time: {}", current);
}
