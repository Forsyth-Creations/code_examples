// Design Patterns in C++
// Demonstrates common access patterns: Singleton, Multition (Registry), Mediator, and more

#include <iostream>
#include <string>
#include <memory>
#include <map>
#include <vector>
#include <mutex>
#include <functional>

// ============================================================================
// 1. SINGLETON PATTERN
// ============================================================================
// Thread-safe singleton using Meyer's Singleton

class Database {
private:
    std::string connectionString;
    
    // Private constructor
    Database() : connectionString("localhost:5432") {
        std::cout << "Initializing Database connection to " << connectionString << "\n";
    }
    
    // Delete copy constructor and assignment
    Database(const Database&) = delete;
    Database& operator=(const Database&) = delete;

public:
    static Database& getInstance() {
        // Thread-safe in C++11 and later
        static Database instance;
        return instance;
    }
    
    std::string query(const std::string& sql) {
        return "Executing '" + sql + "' on " + connectionString;
    }
};

// Logger singleton
class Logger {
private:
    std::string prefix;
    std::mutex mutex_;
    
    Logger() : prefix("[LOG]") {}
    
    Logger(const Logger&) = delete;
    Logger& operator=(const Logger&) = delete;

public:
    static Logger& getInstance() {
        static Logger instance;
        return instance;
    }
    
    void log(const std::string& message) {
        std::lock_guard<std::mutex> lock(mutex_);
        std::cout << prefix << " " << message << "\n";
    }
};

// ============================================================================
// 2. MULTITION PATTERN (Registry)
// ============================================================================
// Multiple named instances stored in a registry

class Connection {
private:
    std::string name;
    std::string host;

public:
    Connection(const std::string& n, const std::string& h) 
        : name(n), host(h) {}
    
    std::string getName() const { return name; }
    std::string getHost() const { return host; }
    
    friend std::ostream& operator<<(std::ostream& os, const Connection& conn) {
        os << "Connection(name='" << conn.name << "', host='" << conn.host << "')";
        return os;
    }
};

class ConnectionPool {
private:
    static std::map<std::string, std::shared_ptr<Connection>> pools;
    static std::mutex mutex_;

public:
    static std::shared_ptr<Connection> getConnection(const std::string& name, const std::string& host) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        auto it = pools.find(name);
        if (it == pools.end()) {
            std::cout << "Creating new connection: " << name << "\n";
            auto conn = std::make_shared<Connection>(name, host);
            pools[name] = conn;
            return conn;
        }
        return it->second;
    }
    
    static void listConnections() {
        std::lock_guard<std::mutex> lock(mutex_);
        std::cout << "Active connections: " << pools.size() << "\n";
        for (const auto& pair : pools) {
            std::cout << "  " << pair.first << " -> " << *pair.second << "\n";
        }
    }
    
    static void closeConnection(const std::string& name) {
        std::lock_guard<std::mutex> lock(mutex_);
        pools.erase(name);
        std::cout << "Closed connection: " << name << "\n";
    }
};

// Initialize static members
std::map<std::string, std::shared_ptr<Connection>> ConnectionPool::pools;
std::mutex ConnectionPool::mutex_;

// ============================================================================
// 3. MEDIATOR PATTERN
// ============================================================================
// Centralizes complex communications between objects

class Component;

class Mediator {
public:
    virtual ~Mediator() = default;
    virtual void notify(Component* sender, const std::string& event, const std::string& data = "") = 0;
};

class Component {
protected:
    Mediator* mediator_;

public:
    Component(Mediator* mediator = nullptr) : mediator_(mediator) {}
    virtual ~Component() = default;
    
    void setMediator(Mediator* mediator) {
        mediator_ = mediator;
    }
};

class User : public Component {
private:
    std::string name;

public:
    User(const std::string& n) : Component(), name(n) {}
    
    std::string getName() const { return name; }
    
    void send(const std::string& message) {
        std::cout << "[" << name << "] Sending: " << message << "\n";
        if (mediator_) {
            mediator_->notify(this, "message", message);
        }
    }
    
    void receive(const std::string& message) {
        std::cout << "[" << name << "] Received: " << message << "\n";
    }
};

class ChatMediator : public Mediator {
private:
    std::map<std::string, User*> users;

public:
    void addUser(User* user) {
        users[user->getName()] = user;
        user->setMediator(this);
    }
    
    void notify(Component* sender, const std::string& event, const std::string& data) override {
        if (event == "message") {
            User* senderUser = dynamic_cast<User*>(sender);
            if (senderUser) {
                broadcast(senderUser->getName(), data);
            }
        }
    }
    
    void broadcast(const std::string& senderName, const std::string& message) {
        for (auto& pair : users) {
            if (pair.first != senderName) {
                pair.second->receive(message);
            }
        }
    }
};

// ============================================================================
// 4. FACTORY PATTERN
// ============================================================================
// Creates objects without specifying exact class

class Shape {
public:
    virtual ~Shape() = default;
    virtual void draw() const = 0;
    virtual double area() const = 0;
};

class Circle : public Shape {
private:
    double radius;

public:
    Circle(double r) : radius(r) {}
    
    void draw() const override {
        std::cout << "Drawing circle with radius " << radius << "\n";
    }
    
    double area() const override {
        constexpr double PI = 3.14159265358979323846;
        return PI * radius * radius;
    }
};

class Rectangle : public Shape {
private:
    double width, height;

public:
    Rectangle(double w, double h) : width(w), height(h) {}
    
    void draw() const override {
        std::cout << "Drawing rectangle " << width << "x" << height << "\n";
    }
    
    double area() const override {
        return width * height;
    }
};

class Triangle : public Shape {
private:
    double base, height;

public:
    Triangle(double b, double h) : base(b), height(h) {}
    
    void draw() const override {
        std::cout << "Drawing triangle with base " << base << " and height " << height << "\n";
    }
    
    double area() const override {
        return 0.5 * base * height;
    }
};

class ShapeFactory {
public:
    static std::unique_ptr<Shape> createShape(const std::string& type, const std::vector<double>& params) {
        if (type == "circle" && params.size() >= 1) {
            return std::make_unique<Circle>(params[0]);
        } else if (type == "rectangle" && params.size() >= 2) {
            return std::make_unique<Rectangle>(params[0], params[1]);
        } else if (type == "triangle" && params.size() >= 2) {
            return std::make_unique<Triangle>(params[0], params[1]);
        }
        return nullptr;
    }
};

// ============================================================================
// 5. BUILDER PATTERN
// ============================================================================
// Constructs complex objects step by step

class HttpRequest {
private:
    std::string method;
    std::string url;
    std::map<std::string, std::string> headers;
    std::string body;

public:
    HttpRequest(const std::string& m, const std::string& u,
                const std::map<std::string, std::string>& h,
                const std::string& b)
        : method(m), url(u), headers(h), body(b) {}
    
    friend std::ostream& operator<<(std::ostream& os, const HttpRequest& req) {
        os << "HttpRequest(method='" << req.method << "', url='" << req.url << "', ";
        os << "headers={";
        for (const auto& h : req.headers) {
            os << h.first << ": " << h.second << ", ";
        }
        os << "}, body='" << req.body << "')";
        return os;
    }
};

class HttpRequestBuilder {
private:
    std::string method = "GET";
    std::string url;
    std::map<std::string, std::string> headers;
    std::string body;

public:
    HttpRequestBuilder(const std::string& u) : url(u) {}
    
    HttpRequestBuilder& setMethod(const std::string& m) {
        method = m;
        return *this;
    }
    
    HttpRequestBuilder& addHeader(const std::string& key, const std::string& value) {
        headers[key] = value;
        return *this;
    }
    
    HttpRequestBuilder& setBody(const std::string& b) {
        body = b;
        return *this;
    }
    
    HttpRequest build() {
        return HttpRequest(method, url, headers, body);
    }
};

// ============================================================================
// 6. OBSERVER PATTERN
// ============================================================================
// Define subscription mechanism to notify multiple objects

class Observer {
public:
    virtual ~Observer() = default;
    virtual void update(const std::string& data) = 0;
};

class Observable {
private:
    std::vector<Observer*> observers;

public:
    void attach(Observer* observer) {
        observers.push_back(observer);
    }
    
    void detach(Observer* observer) {
        observers.erase(
            std::remove(observers.begin(), observers.end(), observer),
            observers.end()
        );
    }
    
    void notify(const std::string& data) {
        for (auto observer : observers) {
            observer->update(data);
        }
    }
};

class NewsPublisher : public Observable {
public:
    void publishNews(const std::string& headline) {
        std::cout << "\n[Publisher] Publishing: " << headline << "\n";
        notify(headline);
    }
};

class Subscriber : public Observer {
private:
    std::string name;

public:
    Subscriber(const std::string& n) : name(n) {}
    
    void update(const std::string& data) override {
        std::cout << "[" << name << "] Received news: " << data << "\n";
    }
};

// ============================================================================
// MAIN - DEMONSTRATE ALL PATTERNS
// ============================================================================

int main() {
    std::cout << "=== C++ Design Patterns ===\n\n";

    // Singleton Pattern
    std::cout << "--- Singleton Pattern ---\n";
    Database& db1 = Database::getInstance();
    Database& db2 = Database::getInstance();
    std::cout << db1.query("SELECT * FROM users") << "\n";
    std::cout << "Same instance: " << (&db1 == &db2) << "\n";
    
    Logger& logger = Logger::getInstance();
    logger.log("Application started");

    // Multition Pattern (Registry)
    std::cout << "\n--- Multition Pattern (Registry) ---\n";
    auto conn1 = ConnectionPool::getConnection("primary", "db1.example.com");
    auto conn2 = ConnectionPool::getConnection("secondary", "db2.example.com");
    auto conn3 = ConnectionPool::getConnection("primary", "db1.example.com");
    ConnectionPool::listConnections();
    std::cout << "Same primary connection: " << (conn1 == conn3) << "\n";

    // Mediator Pattern
    std::cout << "\n--- Mediator Pattern ---\n";
    ChatMediator mediator;
    
    User alice("Alice");
    User bob("Bob");
    User charlie("Charlie");
    
    mediator.addUser(&alice);
    mediator.addUser(&bob);
    mediator.addUser(&charlie);
    
    alice.send("Hello everyone!");
    bob.send("Hi Alice!");

    // Factory Pattern
    std::cout << "\n--- Factory Pattern ---\n";
    auto circle = ShapeFactory::createShape("circle", {5.0});
    auto rectangle = ShapeFactory::createShape("rectangle", {4.0, 6.0});
    auto triangle = ShapeFactory::createShape("triangle", {3.0, 4.0});
    
    if (circle) {
        circle->draw();
        std::cout << "Area: " << circle->area() << "\n";
    }
    
    if (rectangle) {
        rectangle->draw();
        std::cout << "Area: " << rectangle->area() << "\n";
    }
    
    if (triangle) {
        triangle->draw();
        std::cout << "Area: " << triangle->area() << "\n";
    }

    // Builder Pattern
    std::cout << "\n--- Builder Pattern ---\n";
    HttpRequest request = HttpRequestBuilder("https://api.example.com/users")
        .setMethod("POST")
        .addHeader("Content-Type", "application/json")
        .addHeader("Authorization", "Bearer token123")
        .setBody(R"({"name": "John Doe"})")
        .build();
    
    std::cout << request << "\n";

    // Observer Pattern
    std::cout << "\n--- Observer Pattern ---\n";
    NewsPublisher publisher;
    
    Subscriber sub1("Alice");
    Subscriber sub2("Bob");
    Subscriber sub3("Charlie");
    
    publisher.attach(&sub1);
    publisher.attach(&sub2);
    publisher.attach(&sub3);
    
    publisher.publishNews("C++23 Released!");
    
    publisher.detach(&sub2);
    publisher.publishNews("New Library Announced");

    std::cout << "\n--- Pattern Summary ---\n";
    std::cout << "✓ Singleton: Single instance, global access\n";
    std::cout << "✓ Multition: Multiple named instances in registry\n";
    std::cout << "✓ Mediator: Centralized communication\n";
    std::cout << "✓ Factory: Object creation without specifying class\n";
    std::cout << "✓ Builder: Step-by-step complex object construction\n";
    std::cout << "✓ Observer: Subscription mechanism for notifications\n";

    return 0;
}
