"""
Design Patterns in Python
Demonstrates common access patterns: Singleton, Multition (Registry), Mediator, and more
"""

from threading import Lock
from typing import Dict, Optional, Any, List
from abc import ABC, abstractmethod


# ============================================================================
# 1. SINGLETON PATTERN
# ============================================================================
# Classic singleton with thread safety

class SingletonMeta(type):
    """
    Thread-safe Singleton metaclass
    """
    _instances: Dict[type, Any] = {}
    _lock: Lock = Lock()

    def __call__(cls, *args, **kwargs):
        with cls._lock:
            if cls not in cls._instances:
                instance = super().__call__(*args, **kwargs)
                cls._instances[cls] = instance
        return cls._instances[cls]


class Database(metaclass=SingletonMeta):
    """Singleton database connection"""
    
    def __init__(self):
        self.connection_string = "localhost:5432"
        print(f"Initializing Database connection to {self.connection_string}")
    
    def query(self, sql: str) -> str:
        return f"Executing '{sql}' on {self.connection_string}"


class Logger(metaclass=SingletonMeta):
    """Singleton logger"""
    
    def __init__(self):
        self.prefix = "[LOG]"
    
    def log(self, message: str):
        print(f"{self.prefix} {message}")


# Alternative: Simple singleton using decorator
def singleton(cls):
    """Decorator to make a class a singleton"""
    instances = {}
    lock = Lock()
    
    def get_instance(*args, **kwargs):
        if cls not in instances:
            with lock:
                if cls not in instances:
                    instances[cls] = cls(*args, **kwargs)
        return instances[cls]
    
    return get_instance


@singleton
class ConfigManager:
    """Configuration manager as singleton"""
    
    def __init__(self):
        self.config = {"app_name": "MyApp", "version": "1.0"}
    
    def get(self, key: str) -> Any:
        return self.config.get(key)
    
    def set(self, key: str, value: Any):
        self.config[key] = value


# ============================================================================
# 2. MULTITION PATTERN (Registry)
# ============================================================================
# Multiple named instances stored in a registry

class ConnectionPool:
    """Registry pattern for managing multiple named connections"""
    
    _pools: Dict[str, 'Connection'] = {}
    _lock: Lock = Lock()
    
    @classmethod
    def get_connection(cls, name: str, host: str) -> 'Connection':
        """Get or create a named connection"""
        with cls._lock:
            if name not in cls._pools:
                print(f"Creating new connection: {name}")
                cls._pools[name] = Connection(name, host)
            return cls._pools[name]
    
    @classmethod
    def list_connections(cls):
        """List all active connections"""
        print(f"Active connections: {len(cls._pools)}")
        for name, conn in cls._pools.items():
            print(f"  {name} -> {conn}")
    
    @classmethod
    def close_connection(cls, name: str):
        """Close a specific connection"""
        if name in cls._pools:
            del cls._pools[name]
            print(f"Closed connection: {name}")


class Connection:
    """Represents a database connection"""
    
    def __init__(self, name: str, host: str):
        self.name = name
        self.host = host
    
    def __repr__(self):
        return f"Connection(name='{self.name}', host='{self.host}')"


# ============================================================================
# 3. MEDIATOR PATTERN
# ============================================================================
# Centralizes complex communications between objects

class Mediator(ABC):
    """Abstract mediator interface"""
    
    @abstractmethod
    def notify(self, sender: 'Component', event: str, data: Any = None):
        pass


class Component(ABC):
    """Base component that communicates through mediator"""
    
    def __init__(self, mediator: Optional[Mediator] = None):
        self._mediator = mediator
    
    @property
    def mediator(self) -> Mediator:
        return self._mediator
    
    @mediator.setter
    def mediator(self, mediator: Mediator):
        self._mediator = mediator


class ChatMediator(Mediator):
    """Mediator for chat room"""
    
    def __init__(self):
        self._users: Dict[str, 'User'] = {}
    
    def add_user(self, user: 'User'):
        """Register a user with the mediator"""
        self._users[user.name] = user
        user.mediator = self
    
    def notify(self, sender: Component, event: str, data: Any = None):
        """Handle events from components"""
        if event == "message":
            self.broadcast(sender.name, data)
    
    def broadcast(self, sender_name: str, message: str):
        """Broadcast message to all users except sender"""
        for name, user in self._users.items():
            if name != sender_name:
                user.receive(message)


class User(Component):
    """User in chat system"""
    
    def __init__(self, name: str):
        super().__init__()
        self.name = name
    
    def send(self, message: str):
        """Send message through mediator"""
        print(f"[{self.name}] Sending: {message}")
        if self.mediator:
            self.mediator.notify(self, "message", message)
    
    def receive(self, message: str):
        """Receive message from mediator"""
        print(f"[{self.name}] Received: {message}")


# ============================================================================
# 4. FACTORY PATTERN
# ============================================================================
# Creates objects without specifying exact class

class Shape(ABC):
    """Abstract shape interface"""
    
    @abstractmethod
    def draw(self):
        pass
    
    @abstractmethod
    def area(self) -> float:
        pass


class Circle(Shape):
    """Circle shape"""
    
    def __init__(self, radius: float):
        self.radius = radius
    
    def draw(self):
        print(f"Drawing circle with radius {self.radius}")
    
    def area(self) -> float:
        import math
        return math.pi * self.radius ** 2


class Rectangle(Shape):
    """Rectangle shape"""
    
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height
    
    def draw(self):
        print(f"Drawing rectangle {self.width}x{self.height}")
    
    def area(self) -> float:
        return self.width * self.height


class Triangle(Shape):
    """Triangle shape"""
    
    def __init__(self, base: float, height: float):
        self.base = base
        self.height = height
    
    def draw(self):
        print(f"Drawing triangle with base {self.base} and height {self.height}")
    
    def area(self) -> float:
        return 0.5 * self.base * self.height


class ShapeFactory:
    """Factory for creating shapes"""
    
    @staticmethod
    def create_shape(shape_type: str, *args) -> Optional[Shape]:
        """Create a shape based on type"""
        shapes = {
            'circle': Circle,
            'rectangle': Rectangle,
            'triangle': Triangle,
        }
        
        shape_class = shapes.get(shape_type.lower())
        if shape_class:
            return shape_class(*args)
        return None


# ============================================================================
# 5. BUILDER PATTERN
# ============================================================================
# Constructs complex objects step by step

class HttpRequest:
    """HTTP request object"""
    
    def __init__(self):
        self.method = "GET"
        self.url = ""
        self.headers: Dict[str, str] = {}
        self.body: Optional[str] = None
    
    def __repr__(self):
        return (f"HttpRequest(method='{self.method}', url='{self.url}', "
                f"headers={self.headers}, body={self.body})")


class HttpRequestBuilder:
    """Builder for constructing HTTP requests"""
    
    def __init__(self, url: str):
        self._request = HttpRequest()
        self._request.url = url
    
    def method(self, method: str) -> 'HttpRequestBuilder':
        """Set HTTP method"""
        self._request.method = method
        return self
    
    def header(self, key: str, value: str) -> 'HttpRequestBuilder':
        """Add a header"""
        self._request.headers[key] = value
        return self
    
    def body(self, body: str) -> 'HttpRequestBuilder':
        """Set request body"""
        self._request.body = body
        return self
    
    def build(self) -> HttpRequest:
        """Build and return the request"""
        return self._request


# ============================================================================
# 6. OBSERVER PATTERN
# ============================================================================
# Define subscription mechanism to notify multiple objects

class Observable:
    """Subject that can be observed"""
    
    def __init__(self):
        self._observers: List['Observer'] = []
    
    def attach(self, observer: 'Observer'):
        """Attach an observer"""
        self._observers.append(observer)
    
    def detach(self, observer: 'Observer'):
        """Detach an observer"""
        self._observers.remove(observer)
    
    def notify(self, data: Any):
        """Notify all observers"""
        for observer in self._observers:
            observer.update(data)


class Observer(ABC):
    """Observer interface"""
    
    @abstractmethod
    def update(self, data: Any):
        pass


class NewsPublisher(Observable):
    """News publisher that notifies subscribers"""
    
    def publish_news(self, headline: str):
        print(f"\n[Publisher] Publishing: {headline}")
        self.notify(headline)


class Subscriber(Observer):
    """News subscriber"""
    
    def __init__(self, name: str):
        self.name = name
    
    def update(self, data: Any):
        print(f"[{self.name}] Received news: {data}")


# ============================================================================
# MAIN - DEMONSTRATE ALL PATTERNS
# ============================================================================

def main():
    print("=== Python Design Patterns ===\n")

    # Singleton Pattern
    print("--- Singleton Pattern ---")
    db1 = Database()
    db2 = Database()
    print(db1.query("SELECT * FROM users"))
    print(f"Same instance: {db1 is db2}")
    
    logger = Logger()
    logger.log("Application started")
    
    config1 = ConfigManager()
    config1.set("theme", "dark")
    config2 = ConfigManager()
    print(f"Config theme: {config2.get('theme')}")
    print(f"Same config instance: {config1 is config2}")

    # Multition Pattern (Registry)
    print("\n--- Multition Pattern (Registry) ---")
    conn1 = ConnectionPool.get_connection("primary", "db1.example.com")
    conn2 = ConnectionPool.get_connection("secondary", "db2.example.com")
    conn3 = ConnectionPool.get_connection("primary", "db1.example.com")  # Reuses
    ConnectionPool.list_connections()
    print(f"Same primary connection: {conn1 is conn3}")

    # Mediator Pattern
    print("\n--- Mediator Pattern ---")
    mediator = ChatMediator()
    
    alice = User("Alice")
    bob = User("Bob")
    charlie = User("Charlie")
    
    mediator.add_user(alice)
    mediator.add_user(bob)
    mediator.add_user(charlie)
    
    alice.send("Hello everyone!")
    bob.send("Hi Alice!")

    # Factory Pattern
    print("\n--- Factory Pattern ---")
    circle = ShapeFactory.create_shape("circle", 5.0)
    rectangle = ShapeFactory.create_shape("rectangle", 4.0, 6.0)
    triangle = ShapeFactory.create_shape("triangle", 3.0, 4.0)
    
    if circle:
        circle.draw()
        print(f"Area: {circle.area():.2f}")
    
    if rectangle:
        rectangle.draw()
        print(f"Area: {rectangle.area():.2f}")
    
    if triangle:
        triangle.draw()
        print(f"Area: {triangle.area():.2f}")

    # Builder Pattern
    print("\n--- Builder Pattern ---")
    request = (HttpRequestBuilder("https://api.example.com/users")
               .method("POST")
               .header("Content-Type", "application/json")
               .header("Authorization", "Bearer token123")
               .body('{"name": "John Doe"}')
               .build())
    
    print(request)

    # Observer Pattern
    print("\n--- Observer Pattern ---")
    publisher = NewsPublisher()
    
    sub1 = Subscriber("Alice")
    sub2 = Subscriber("Bob")
    sub3 = Subscriber("Charlie")
    
    publisher.attach(sub1)
    publisher.attach(sub2)
    publisher.attach(sub3)
    
    publisher.publish_news("Python 4.0 Released!")
    
    publisher.detach(sub2)
    publisher.publish_news("New Framework Announced")

    print("\n--- Pattern Summary ---")
    print("✓ Singleton: Single instance, global access")
    print("✓ Multition: Multiple named instances in registry")
    print("✓ Mediator: Centralized communication")
    print("✓ Factory: Object creation without specifying class")
    print("✓ Builder: Step-by-step complex object construction")
    print("✓ Observer: Subscription mechanism for notifications")


if __name__ == "__main__":
    main()
