"""
Classes in Python
Demonstrates class structures, inheritance, and special methods
"""

# Simple class
class Person:
    """A simple Person class"""
    
    def __init__(self, name, age):
        """Constructor"""
        self.name = name
        self.age = age
    
    def introduce(self):
        """Instance method"""
        return f"Hi, I'm {self.name} and I'm {self.age} years old"
    
    def __str__(self):
        """String representation"""
        return f"Person(name={self.name}, age={self.age})"
    
    def __repr__(self):
        """Developer-friendly representation"""
        return f"Person('{self.name}', {self.age})"


# Class with properties
class Rectangle:
    """Rectangle with properties"""
    
    def __init__(self, width, height):
        self._width = width
        self._height = height
    
    @property
    def width(self):
        """Width property getter"""
        return self._width
    
    @width.setter
    def width(self, value):
        """Width property setter"""
        if value <= 0:
            raise ValueError("Width must be positive")
        self._width = value
    
    @property
    def area(self):
        """Computed property"""
        return self._width * self._height
    
    def __str__(self):
        return f"Rectangle({self._width}x{self._height})"


# Class with class variables and methods
class Counter:
    """Class with class-level attributes"""
    
    count = 0  # Class variable
    
    def __init__(self, name):
        self.name = name
        Counter.count += 1
    
    @classmethod
    def get_count(cls):
        """Class method"""
        return cls.count
    
    @staticmethod
    def is_valid_name(name):
        """Static method"""
        return len(name) > 0 and name.isalnum()


# Inheritance
class Animal:
    """Base class"""
    
    def __init__(self, name):
        self.name = name
    
    def speak(self):
        """Method to be overridden"""
        pass
    
    def __str__(self):
        return f"{self.__class__.__name__}({self.name})"


class Dog(Animal):
    """Derived class"""
    
    def __init__(self, name, breed):
        super().__init__(name)
        self.breed = breed
    
    def speak(self):
        """Override parent method"""
        return "Woof!"
    
    def __str__(self):
        return f"Dog(name={self.name}, breed={self.breed})"


class Cat(Animal):
    """Another derived class"""
    
    def speak(self):
        return "Meow!"


# Multiple inheritance
class Flyable:
    """Mixin for flying"""
    
    def fly(self):
        return "Flying!"


class Swimmable:
    """Mixin for swimming"""
    
    def swim(self):
        return "Swimming!"


class Duck(Animal, Flyable, Swimmable):
    """Class with multiple inheritance"""
    
    def speak(self):
        return "Quack!"


# Dataclass (Python 3.7+)
from dataclasses import dataclass, field
from typing import List

@dataclass
class Product:
    """Dataclass example"""
    name: str
    price: float
    quantity: int = 0
    tags: List[str] = field(default_factory=list)
    
    def total_value(self):
        return self.price * self.quantity


# Abstract base class
from abc import ABC, abstractmethod

class Shape(ABC):
    """Abstract base class"""
    
    @abstractmethod
    def area(self):
        """Must be implemented by subclasses"""
        pass
    
    @abstractmethod
    def perimeter(self):
        """Must be implemented by subclasses"""
        pass


class Circle(Shape):
    """Concrete implementation"""
    
    def __init__(self, radius):
        self.radius = radius
    
    def area(self):
        import math
        return math.pi * self.radius ** 2
    
    def perimeter(self):
        import math
        return 2 * math.pi * self.radius


def main():
    print("=== Python Classes ===\n")

    # Simple class
    print("--- Simple Class ---")
    person = Person("Alice", 30)
    print(person.introduce())
    print(str(person))
    print(repr(person))

    # Class with properties
    print("\n--- Properties ---")
    rect = Rectangle(10, 5)
    print(rect)
    print(f"Area: {rect.area}")
    rect.width = 20
    print(f"New area: {rect.area}")

    # Class variables and methods
    print("\n--- Class Variables and Methods ---")
    c1 = Counter("first")
    c2 = Counter("second")
    print(f"Total counters: {Counter.get_count()}")
    print(f"Valid name 'abc123': {Counter.is_valid_name('abc123')}")
    print(f"Valid name 'abc-123': {Counter.is_valid_name('abc-123')}")

    # Inheritance
    print("\n--- Inheritance ---")
    dog = Dog("Buddy", "Golden Retriever")
    cat = Cat("Whiskers")
    print(f"{dog}: {dog.speak()}")
    print(f"{cat}: {cat.speak()}")

    # Polymorphism
    print("\n--- Polymorphism ---")
    animals = [dog, cat, Duck("Donald")]
    for animal in animals:
        print(f"{animal.name} says: {animal.speak()}")

    # Multiple inheritance
    print("\n--- Multiple Inheritance ---")
    duck = Duck("Daffy")
    print(f"{duck.name} says: {duck.speak()}")
    print(f"{duck.name} can: {duck.fly()}")
    print(f"{duck.name} can: {duck.swim()}")

    # Dataclass
    print("\n--- Dataclass ---")
    product = Product("Widget", 19.99, 5, ["electronics", "gadget"])
    print(product)
    print(f"Total value: ${product.total_value():.2f}")

    # Abstract base class
    print("\n--- Abstract Base Class ---")
    circle = Circle(5)
    print(f"Circle area: {circle.area():.2f}")
    print(f"Circle perimeter: {circle.perimeter():.2f}")

    # Special methods
    print("\n--- Special Methods ---")
    class Point:
        def __init__(self, x, y):
            self.x = x
            self.y = y
        
        def __add__(self, other):
            return Point(self.x + other.x, self.y + other.y)
        
        def __eq__(self, other):
            return self.x == other.x and self.y == other.y
        
        def __str__(self):
            return f"Point({self.x}, {self.y})"
    
    p1 = Point(1, 2)
    p2 = Point(3, 4)
    p3 = p1 + p2
    print(f"{p1} + {p2} = {p3}")
    print(f"{p1} == {p2}: {p1 == p2}")


if __name__ == "__main__":
    main()
