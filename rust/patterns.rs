// Design Patterns in Rust
// Demonstrates common access patterns: Singleton, Registry (Multition), Mediator, and more

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Once};
use std::cell::RefCell;

// ============================================================================
// 1. SINGLETON PATTERN
// ============================================================================
// Thread-safe singleton using Once and static
// NOTE: For production code, consider using std::sync::OnceLock (Rust 1.70+)
// or the lazy_static crate for safer alternatives to mutable statics

static mut SINGLETON: Option<Database> = None;
static INIT: Once = Once::new();

#[derive(Clone)]
struct Database {
    connection_string: String,
}

impl Database {
    fn instance() -> &'static Database {
        unsafe {
            INIT.call_once(|| {
                SINGLETON = Some(Database {
                    connection_string: String::from("localhost:5432"),
                });
            });
            SINGLETON.as_ref().unwrap()
        }
    }

    fn query(&self, sql: &str) -> String {
        format!("Executing '{}' on {}", sql, self.connection_string)
    }
}

// Alternative: Lazy static singleton (using lazy_static would be more idiomatic)
struct Logger {
    prefix: String,
}

impl Logger {
    fn global() -> &'static Mutex<Logger> {
        static LOGGER: Once = Once::new();
        static mut INSTANCE: Option<Mutex<Logger>> = None;
        
        unsafe {
            LOGGER.call_once(|| {
                INSTANCE = Some(Mutex::new(Logger {
                    prefix: String::from("[LOG]"),
                }));
            });
            INSTANCE.as_ref().unwrap()
        }
    }

    fn log(&self, message: &str) {
        println!("{} {}", self.prefix, message);
    }
}

// ============================================================================
// 2. MULTITION PATTERN (Registry)
// ============================================================================
// Multiple named instances stored in a registry

struct ConnectionPool {
    pools: Mutex<HashMap<String, Arc<Connection>>>,
}

#[derive(Debug)]
struct Connection {
    name: String,
    host: String,
}

impl ConnectionPool {
    fn new() -> Self {
        ConnectionPool {
            pools: Mutex::new(HashMap::new()),
        }
    }

    fn get_connection(&self, name: &str, host: &str) -> Arc<Connection> {
        let mut pools = self.pools.lock().unwrap();
        
        pools.entry(name.to_string())
            .or_insert_with(|| {
                println!("Creating new connection: {}", name);
                Arc::new(Connection {
                    name: name.to_string(),
                    host: host.to_string(),
                })
            })
            .clone()
    }

    fn list_connections(&self) {
        let pools = self.pools.lock().unwrap();
        println!("Active connections: {}", pools.len());
        for (name, conn) in pools.iter() {
            println!("  {} -> {:?}", name, conn);
        }
    }
}

// ============================================================================
// 3. MEDIATOR PATTERN
// ============================================================================
// Centralizes complex communications between objects

trait Component {
    fn send(&self, mediator: &ChatMediator, message: &str);
    fn receive(&self, message: &str);
    fn name(&self) -> &str;
}

struct User {
    name: String,
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}

impl Component for User {
    fn send(&self, mediator: &ChatMediator, message: &str) {
        println!("[{}] Sending: {}", self.name, message);
        mediator.broadcast(self.name(), message);
    }

    fn receive(&self, message: &str) {
        println!("[{}] Received: {}", self.name, message);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct ChatMediator {
    users: RefCell<HashMap<String, Arc<User>>>,
}

impl ChatMediator {
    fn new() -> Self {
        ChatMediator {
            users: RefCell::new(HashMap::new()),
        }
    }

    fn add_user(&self, user: Arc<User>) {
        self.users.borrow_mut().insert(user.name().to_string(), user);
    }

    fn broadcast(&self, sender: &str, message: &str) {
        let users = self.users.borrow();
        for (name, user) in users.iter() {
            if name != sender {
                user.receive(message);
            }
        }
    }
}

// ============================================================================
// 4. FACTORY PATTERN
// ============================================================================
// Creates objects without specifying exact class

trait Shape {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle {}x{}", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct ShapeFactory;

impl ShapeFactory {
    fn create_shape(shape_type: &str, params: Vec<f64>) -> Option<Box<dyn Shape>> {
        match shape_type {
            "circle" if params.len() >= 1 => {
                Some(Box::new(Circle { radius: params[0] }))
            }
            "rectangle" if params.len() >= 2 => {
                Some(Box::new(Rectangle {
                    width: params[0],
                    height: params[1],
                }))
            }
            _ => None,
        }
    }
}

// ============================================================================
// 5. BUILDER PATTERN
// ============================================================================
// Constructs complex objects step by step

#[derive(Debug)]
struct HttpRequest {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

struct HttpRequestBuilder {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl HttpRequestBuilder {
    fn new(url: &str) -> Self {
        HttpRequestBuilder {
            method: String::from("GET"),
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
        }
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
        }
    }
}

// ============================================================================
// MAIN - DEMONSTRATE ALL PATTERNS
// ============================================================================

fn main() {
    println!("=== Rust Design Patterns ===\n");

    // Singleton Pattern
    println!("--- Singleton Pattern ---");
    let db1 = Database::instance();
    let db2 = Database::instance();
    println!("{}", db1.query("SELECT * FROM users"));
    println!("Same instance: {}", std::ptr::eq(db1, db2));

    let logger = Logger::global();
    logger.lock().unwrap().log("Application started");

    // Multition Pattern (Registry)
    println!("\n--- Multition Pattern (Registry) ---");
    let pool = ConnectionPool::new();
    let conn1 = pool.get_connection("primary", "db1.example.com");
    let conn2 = pool.get_connection("secondary", "db2.example.com");
    let conn3 = pool.get_connection("primary", "db1.example.com"); // Reuses existing
    pool.list_connections();
    println!("Same primary connection: {}", Arc::ptr_eq(&conn1, &conn3));

    // Mediator Pattern
    println!("\n--- Mediator Pattern ---");
    let mediator = ChatMediator::new();
    
    let alice = Arc::new(User::new("Alice"));
    let bob = Arc::new(User::new("Bob"));
    let charlie = Arc::new(User::new("Charlie"));
    
    mediator.add_user(alice.clone());
    mediator.add_user(bob.clone());
    mediator.add_user(charlie.clone());
    
    alice.send(&mediator, "Hello everyone!");
    bob.send(&mediator, "Hi Alice!");

    // Factory Pattern
    println!("\n--- Factory Pattern ---");
    let circle = ShapeFactory::create_shape("circle", vec![5.0]).unwrap();
    let rectangle = ShapeFactory::create_shape("rectangle", vec![4.0, 6.0]).unwrap();
    
    circle.draw();
    println!("Area: {:.2}", circle.area());
    
    rectangle.draw();
    println!("Area: {:.2}", rectangle.area());

    // Builder Pattern
    println!("\n--- Builder Pattern ---");
    let request = HttpRequestBuilder::new("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "John Doe"}"#)
        .build();
    
    println!("{:?}", request);

    println!("\n--- Pattern Summary ---");
    println!("✓ Singleton: Single instance, global access");
    println!("✓ Multition: Multiple named instances in registry");
    println!("✓ Mediator: Centralized communication");
    println!("✓ Factory: Object creation without specifying class");
    println!("✓ Builder: Step-by-step complex object construction");
}
