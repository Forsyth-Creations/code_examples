// Classes and Structs in C++
// Demonstrates object-oriented programming concepts

#include <iostream>
#include <string>
#include <memory>
#include <cmath>

// Simple struct (public by default)
struct Point {
    double x;
    double y;
    
    // Constructor
    Point(double x = 0, double y = 0) : x(x), y(y) {}
    
    // Method
    double distance(const Point& other) const {
        double dx = x - other.x;
        double dy = y - other.y;
        return std::sqrt(dx * dx + dy * dy);
    }
};

// Simple class (private by default)
class Person {
private:
    std::string name;
    int age;

public:
    // Constructor
    Person(const std::string& name, int age) : name(name), age(age) {}
    
    // Getter methods
    std::string getName() const { return name; }
    int getAge() const { return age; }
    
    // Setter methods
    void setName(const std::string& newName) { name = newName; }
    void setAge(int newAge) {
        if (newAge >= 0) {
            age = newAge;
        }
    }
    
    // Method
    void introduce() const {
        std::cout << "Hi, I'm " << name << " and I'm " << age << " years old\n";
    }
};

// Class with constructor overloading
class Rectangle {
private:
    double width;
    double height;

public:
    // Default constructor
    Rectangle() : width(0), height(0) {}
    
    // Parameterized constructor
    Rectangle(double w, double h) : width(w), height(h) {}
    
    // Square constructor (single parameter)
    explicit Rectangle(double size) : width(size), height(size) {}
    
    // Methods
    double area() const {
        return width * height;
    }
    
    double perimeter() const {
        return 2 * (width + height);
    }
    
    // Operator overloading
    Rectangle operator+(const Rectangle& other) const {
        return Rectangle(width + other.width, height + other.height);
    }
    
    // Friend function for output
    friend std::ostream& operator<<(std::ostream& os, const Rectangle& rect) {
        os << "Rectangle(" << rect.width << "x" << rect.height << ")";
        return os;
    }
};

// Inheritance
class Animal {
protected:
    std::string name;

public:
    Animal(const std::string& name) : name(name) {}
    
    // Virtual function (can be overridden)
    virtual std::string speak() const {
        return "Some sound";
    }
    
    // Virtual destructor (important for polymorphism)
    virtual ~Animal() = default;
    
    std::string getName() const { return name; }
};

class Dog : public Animal {
private:
    std::string breed;

public:
    Dog(const std::string& name, const std::string& breed) 
        : Animal(name), breed(breed) {}
    
    // Override virtual function
    std::string speak() const override {
        return "Woof!";
    }
    
    std::string getBreed() const { return breed; }
};

class Cat : public Animal {
public:
    Cat(const std::string& name) : Animal(name) {}
    
    std::string speak() const override {
        return "Meow!";
    }
};

// Abstract class (interface)
class Shape {
public:
    // Pure virtual functions
    virtual double area() const = 0;
    virtual double perimeter() const = 0;
    virtual ~Shape() = default;
};

class Circle : public Shape {
private:
    double radius;

public:
    Circle(double r) : radius(r) {}
    
    double area() const override {
        return 3.14159 * radius * radius;
    }
    
    double perimeter() const override {
        return 2 * 3.14159 * radius;
    }
};

// Static members
class Counter {
private:
    static int count;
    int id;

public:
    Counter() : id(++count) {}
    
    static int getCount() {
        return count;
    }
    
    int getId() const {
        return id;
    }
};

// Initialize static member
int Counter::count = 0;

// Template class
template<typename T>
class Container {
private:
    T value;

public:
    Container(T val) : value(val) {}
    
    T getValue() const { return value; }
    void setValue(T val) { value = val; }
};

int main() {
    std::cout << "=== C++ Classes and Structs ===\n\n";

    // Struct usage
    std::cout << "--- Struct ---\n";
    Point p1(3, 4);
    Point p2(0, 0);
    std::cout << "Point 1: (" << p1.x << ", " << p1.y << ")\n";
    std::cout << "Distance to origin: " << p1.distance(p2) << "\n";

    // Class usage
    std::cout << "\n--- Class ---\n";
    Person person("Alice", 30);
    person.introduce();
    person.setAge(31);
    std::cout << "After birthday: " << person.getAge() << " years old\n";

    // Constructor overloading
    std::cout << "\n--- Constructor Overloading ---\n";
    Rectangle rect1;
    Rectangle rect2(10, 5);
    Rectangle square(7);
    std::cout << rect1 << " Area: " << rect1.area() << "\n";
    std::cout << rect2 << " Area: " << rect2.area() << "\n";
    std::cout << square << " Area: " << square.area() << "\n";

    // Operator overloading
    std::cout << "\n--- Operator Overloading ---\n";
    Rectangle combined = rect2 + square;
    std::cout << rect2 << " + " << square << " = " << combined << "\n";

    // Inheritance and polymorphism
    std::cout << "\n--- Inheritance and Polymorphism ---\n";
    Dog dog("Buddy", "Golden Retriever");
    Cat cat("Whiskers");
    
    std::cout << dog.getName() << " (" << dog.getBreed() << ") says: " << dog.speak() << "\n";
    std::cout << cat.getName() << " says: " << cat.speak() << "\n";

    // Polymorphism with pointers
    std::cout << "\n--- Polymorphism with Pointers ---\n";
    Animal* animals[] = {&dog, &cat};
    for (const auto& animal : animals) {
        std::cout << animal->getName() << " says: " << animal->speak() << "\n";
    }

    // Abstract class
    std::cout << "\n--- Abstract Class ---\n";
    Circle circle(5);
    std::cout << "Circle area: " << circle.area() << "\n";
    std::cout << "Circle perimeter: " << circle.perimeter() << "\n";

    // Static members
    std::cout << "\n--- Static Members ---\n";
    Counter c1, c2, c3;
    std::cout << "Total counters: " << Counter::getCount() << "\n";
    std::cout << "Counter IDs: " << c1.getId() << ", " << c2.getId() << ", " << c3.getId() << "\n";

    // Template class
    std::cout << "\n--- Template Class ---\n";
    Container<int> intContainer(42);
    Container<std::string> stringContainer("Hello");
    std::cout << "Int container: " << intContainer.getValue() << "\n";
    std::cout << "String container: " << stringContainer.getValue() << "\n";

    // Smart pointers (modern C++)
    std::cout << "\n--- Smart Pointers ---\n";
    auto uniquePtr = std::make_unique<Dog>("Max", "Labrador");
    auto sharedPtr = std::make_shared<Cat>("Felix");
    
    std::cout << "Unique ptr: " << uniquePtr->getName() << " says " << uniquePtr->speak() << "\n";
    std::cout << "Shared ptr: " << sharedPtr->getName() << " says " << sharedPtr->speak() << "\n";

    return 0;
}
