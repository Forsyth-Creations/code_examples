// Control Flow in C++
// Demonstrates if statements, loops, and switch statements

#include <iostream>
#include <vector>
#include <string>

int main() {
    std::cout << "=== C++ Control Flow ===\n\n";

    // If statements
    std::cout << "--- If Statements ---\n";
    int number = 42;
    
    if (number > 0) {
        std::cout << number << " is positive\n";
    } else if (number < 0) {
        std::cout << number << " is negative\n";
    } else {
        std::cout << number << " is zero\n";
    }

    // Ternary operator
    std::string result = (number % 2 == 0) ? "even" : "odd";
    std::cout << number << " is " << result << "\n";

    // For loop (traditional)
    std::cout << "\n--- Traditional For Loop ---\n";
    for (int i = 0; i < 5; i++) {
        std::cout << "Index: " << i << "\n";
    }

    // Range-based for loop (C++11)
    std::cout << "\n--- Range-Based For Loop ---\n";
    std::vector<std::string> fruits = {"apple", "banana", "cherry"};
    for (const auto& fruit : fruits) {
        std::cout << "Fruit: " << fruit << "\n";
    }

    // For loop with iterator
    std::cout << "\n--- For Loop with Iterator ---\n";
    std::vector<int> numbers = {1, 2, 3, 4, 5};
    for (auto it = numbers.begin(); it != numbers.end(); ++it) {
        std::cout << "Value: " << *it << "\n";
    }

    // While loop
    std::cout << "\n--- While Loop ---\n";
    int countdown = 3;
    while (countdown > 0) {
        std::cout << "Countdown: " << countdown << "\n";
        countdown--;
    }
    std::cout << "Liftoff!\n";

    // Do-while loop
    std::cout << "\n--- Do-While Loop ---\n";
    int count = 0;
    do {
        std::cout << "Count: " << count << "\n";
        count++;
    } while (count < 3);

    // Break and continue
    std::cout << "\n--- Break and Continue ---\n";
    for (int i = 0; i < 10; i++) {
        if (i == 3) {
            continue;  // Skip 3
        }
        if (i == 7) {
            break;  // Stop at 7
        }
        std::cout << "Number: " << i << "\n";
    }

    // Switch statement
    std::cout << "\n--- Switch Statement ---\n";
    int day = 3;
    switch (day) {
        case 1:
            std::cout << "Monday\n";
            break;
        case 2:
            std::cout << "Tuesday\n";
            break;
        case 3:
            std::cout << "Wednesday\n";
            break;
        case 4:
            std::cout << "Thursday\n";
            break;
        case 5:
            std::cout << "Friday\n";
            break;
        case 6:
        case 7:
            std::cout << "Weekend\n";
            break;
        default:
            std::cout << "Invalid day\n";
    }

    // Switch with enum
    std::cout << "\n--- Switch with Enum ---\n";
    enum class Color { Red, Green, Blue, Yellow };
    Color color = Color::Blue;
    
    switch (color) {
        case Color::Red:
            std::cout << "Color is Red\n";
            break;
        case Color::Green:
            std::cout << "Color is Green\n";
            break;
        case Color::Blue:
            std::cout << "Color is Blue\n";
            break;
        case Color::Yellow:
            std::cout << "Color is Yellow\n";
            break;
    }

    // Nested loops
    std::cout << "\n--- Nested Loops ---\n";
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            std::cout << "(" << i << ", " << j << ") ";
        }
        std::cout << "\n";
    }

    // If with initialization (C++17)
    std::cout << "\n--- If with Initialization (C++17) ---\n";
    if (auto val = 42; val > 0) {
        std::cout << "Value " << val << " is positive\n";
    }

    // Switch with initialization (C++17)
    std::cout << "\n--- Switch with Initialization (C++17) ---\n";
    switch (auto value = 2; value) {
        case 1:
            std::cout << "One\n";
            break;
        case 2:
            std::cout << "Two\n";
            break;
        default:
            std::cout << "Other\n";
    }

    // Exception handling
    std::cout << "\n--- Exception Handling ---\n";
    try {
        throw std::runtime_error("An error occurred");
    } catch (const std::runtime_error& e) {
        std::cout << "Caught exception: " << e.what() << "\n";
    } catch (...) {
        std::cout << "Caught unknown exception\n";
    }

    // Goto (generally discouraged)
    std::cout << "\n--- Goto Statement (use sparingly) ---\n";
    int x = 0;
    start:
    std::cout << "x = " << x << "\n";
    x++;
    if (x < 3) {
        goto start;
    }

    return 0;
}
