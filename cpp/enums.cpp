// Enums in C++
// Demonstrates enum and enum class (scoped enums)

#include <iostream>
#include <string>

// Traditional enum (unscoped)
enum Color {
    RED,
    GREEN,
    BLUE,
    YELLOW
};

// Enum with explicit values
enum HttpStatus {
    OK = 200,
    CREATED = 201,
    BAD_REQUEST = 400,
    NOT_FOUND = 404,
    SERVER_ERROR = 500
};

// Scoped enum (enum class) - C++11
enum class Direction {
    North,
    South,
    East,
    West
};

// Enum class with underlying type
enum class Priority : unsigned char {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
};

// Enum class for better type safety
enum class TrafficLight {
    Red,
    Yellow,
    Green
};

// Helper function for traditional enum
std::string colorToString(Color color) {
    switch (color) {
        case RED: return "Red";
        case GREEN: return "Green";
        case BLUE: return "Blue";
        case YELLOW: return "Yellow";
        default: return "Unknown";
    }
}

// Helper function for scoped enum
std::string directionToString(Direction dir) {
    switch (dir) {
        case Direction::North: return "North";
        case Direction::South: return "South";
        case Direction::East: return "East";
        case Direction::West: return "West";
        default: return "Unknown";
    }
}

// Class using enum
class Car {
private:
    Direction heading;
    int speed;

public:
    Car(Direction dir = Direction::North) : heading(dir), speed(0) {}
    
    void setDirection(Direction dir) {
        heading = dir;
    }
    
    Direction getDirection() const {
        return heading;
    }
    
    std::string getHeadingString() const {
        return directionToString(heading);
    }
};

int main() {
    std::cout << "=== C++ Enums ===\n\n";

    // Traditional enum
    std::cout << "--- Traditional Enum ---\n";
    Color favoriteColor = RED;
    Color skyColor = BLUE;
    
    std::cout << "Favorite color: " << colorToString(favoriteColor) << "\n";
    std::cout << "Sky color: " << colorToString(skyColor) << "\n";
    std::cout << "Color value: " << favoriteColor << "\n";  // Implicitly converts to int

    // Enum with explicit values
    std::cout << "\n--- Enum with Explicit Values ---\n";
    HttpStatus status = OK;
    std::cout << "HTTP Status OK: " << status << "\n";
    
    status = NOT_FOUND;
    if (status == NOT_FOUND) {
        std::cout << "HTTP Status " << status << ": Not Found\n";
    }

    // Scoped enum (enum class)
    std::cout << "\n--- Scoped Enum (enum class) ---\n";
    Direction currentDir = Direction::North;
    std::cout << "Current direction: " << directionToString(currentDir) << "\n";
    
    // Enum class requires explicit scope
    currentDir = Direction::East;
    std::cout << "New direction: " << directionToString(currentDir) << "\n";
    
    // Cannot implicitly convert to int (type safe)
    // int dirValue = currentDir; // ERROR: won't compile
    int dirValue = static_cast<int>(currentDir);  // Explicit cast needed
    std::cout << "Direction as int: " << dirValue << "\n";

    // Enum class with underlying type
    std::cout << "\n--- Enum with Underlying Type ---\n";
    Priority taskPriority = Priority::High;
    std::cout << "Task priority: " << static_cast<int>(taskPriority) << "\n";
    
    // Can explicitly specify the underlying value
    taskPriority = Priority::Critical;
    std::cout << "Critical priority value: " << static_cast<int>(taskPriority) << "\n";

    // Switch with enum class
    std::cout << "\n--- Switch with Enum Class ---\n";
    TrafficLight light = TrafficLight::Green;
    
    switch (light) {
        case TrafficLight::Red:
            std::cout << "Stop!\n";
            break;
        case TrafficLight::Yellow:
            std::cout << "Caution!\n";
            break;
        case TrafficLight::Green:
            std::cout << "Go!\n";
            break;
    }

    // Using enum in class
    std::cout << "\n--- Enum in Class ---\n";
    Car car(Direction::North);
    std::cout << "Car heading: " << car.getHeadingString() << "\n";
    
    car.setDirection(Direction::South);
    std::cout << "New heading: " << car.getHeadingString() << "\n";

    // Comparing enums
    std::cout << "\n--- Comparing Enums ---\n";
    Direction dir1 = Direction::North;
    Direction dir2 = Direction::North;
    Direction dir3 = Direction::South;
    
    if (dir1 == dir2) {
        std::cout << "dir1 and dir2 are the same\n";
    }
    
    if (dir1 != dir3) {
        std::cout << "dir1 and dir3 are different\n";
    }

    // Using multiple enum types (showing namespace advantage)
    std::cout << "\n--- Namespace Advantage ---\n";
    enum class Status { OK, Error };
    enum class Result { OK, Failed };
    
    Status s = Status::OK;
    Result r = Result::OK;
    
    // These are different types and won't be confused
    std::cout << "Status OK: " << static_cast<int>(s) << "\n";
    std::cout << "Result OK: " << static_cast<int>(r) << "\n";
    
    // Won't compile: if (s == r) // Different types!

    return 0;
}
