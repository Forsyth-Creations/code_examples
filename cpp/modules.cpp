// Modules and Includes in C++
// Demonstrates how to organize code and use the preprocessor

#include <iostream>      // Standard I/O
#include <string>        // String class
#include <vector>        // Dynamic array
#include <map>           // Associative container
#include <algorithm>     // Algorithms library
#include <memory>        // Smart pointers
#include <fstream>       // File I/O
#include <sstream>       // String streams
#include <cmath>         // Math functions
#include <ctime>         // Time functions
#include <iomanip>       // I/O manipulators

// Namespace demonstration
namespace MyNamespace {
    int value = 42;
    
    void greet(const std::string& name) {
        std::cout << "Hello from MyNamespace, " << name << "!\n";
    }
    
    class MyClass {
    public:
        void display() {
            std::cout << "MyClass from MyNamespace\n";
        }
    };
}

// Nested namespace
namespace Outer {
    namespace Inner {
        int secretValue = 99;
        
        void reveal() {
            std::cout << "Secret value: " << secretValue << "\n";
        }
    }
}

// Namespace alias
namespace MI = MyNamespace;
namespace OI = Outer::Inner;

// Using directive for specific items
using std::cout;
using std::string;
using std::vector;

// Function templates
template<typename T>
T maximum(T a, T b) {
    return (a > b) ? a : b;
}

int main() {
    cout << "=== C++ Modules and Includes ===\n\n";

    // Standard library - iostream
    cout << "--- iostream ---\n";
    cout << "Standard output\n";
    std::cerr << "Standard error\n";

    // Standard library - string
    cout << "\n--- string ---\n";
    string text = "Hello, C++!";
    cout << "String: " << text << "\n";
    cout << "Length: " << text.length() << "\n";
    cout << "Substring: " << text.substr(0, 5) << "\n";

    // Standard library - vector
    cout << "\n--- vector ---\n";
    vector<int> numbers = {1, 2, 3, 4, 5};
    numbers.push_back(6);
    
    cout << "Vector: ";
    for (const auto& num : numbers) {
        cout << num << " ";
    }
    cout << "\n";

    // Standard library - map
    cout << "\n--- map ---\n";
    std::map<string, int> ages;
    ages["Alice"] = 30;
    ages["Bob"] = 25;
    ages["Charlie"] = 35;
    
    cout << "Ages:\n";
    for (const auto& pair : ages) {
        cout << "  " << pair.first << ": " << pair.second << "\n";
    }

    // Standard library - algorithm
    cout << "\n--- algorithm ---\n";
    vector<int> nums = {5, 2, 8, 1, 9};
    
    std::sort(nums.begin(), nums.end());
    cout << "Sorted: ";
    for (const auto& n : nums) {
        cout << n << " ";
    }
    cout << "\n";
    
    auto it = std::find(nums.begin(), nums.end(), 8);
    if (it != nums.end()) {
        cout << "Found 8 at position " << (it - nums.begin()) << "\n";
    }

    // Standard library - memory (smart pointers)
    cout << "\n--- memory (Smart Pointers) ---\n";
    auto uniquePtr = std::make_unique<int>(42);
    auto sharedPtr = std::make_shared<string>("Shared");
    
    cout << "Unique pointer: " << *uniquePtr << "\n";
    cout << "Shared pointer: " << *sharedPtr << "\n";
    cout << "Shared count: " << sharedPtr.use_count() << "\n";

    // Standard library - cmath
    cout << "\n--- cmath ---\n";
    cout << "sqrt(16): " << std::sqrt(16) << "\n";
    cout << "pow(2, 10): " << std::pow(2, 10) << "\n";
    cout << "sin(PI/2): " << std::sin(3.14159 / 2) << "\n";

    // Standard library - sstream
    cout << "\n--- sstream ---\n";
    std::stringstream ss;
    ss << "Number: " << 42 << ", Float: " << 3.14;
    cout << "Stringstream: " << ss.str() << "\n";

    // Custom namespace
    cout << "\n--- Custom Namespace ---\n";
    cout << "Value from MyNamespace: " << MyNamespace::value << "\n";
    MyNamespace::greet("World");
    
    MyNamespace::MyClass obj;
    obj.display();

    // Nested namespace
    cout << "\n--- Nested Namespace ---\n";
    Outer::Inner::reveal();

    // Namespace alias
    cout << "\n--- Namespace Alias ---\n";
    cout << "Using alias MI: " << MI::value << "\n";
    OI::reveal();

    // Using declaration
    cout << "\n--- Using Declaration ---\n";
    using Outer::Inner::secretValue;
    cout << "Direct access: " << secretValue << "\n";

    // Template function
    cout << "\n--- Template Function ---\n";
    cout << "Max(10, 20): " << maximum(10, 20) << "\n";
    cout << "Max(3.14, 2.71): " << maximum(3.14, 2.71) << "\n";
    cout << "Max('a', 'z'): " << maximum('a', 'z') << "\n";

    // STL algorithms with lambdas
    cout << "\n--- Lambdas with STL ---\n";
    vector<int> values = {1, 2, 3, 4, 5};
    
    // for_each with lambda
    cout << "Squared values: ";
    std::for_each(values.begin(), values.end(), [](int n) {
        cout << n * n << " ";
    });
    cout << "\n";
    
    // count_if with lambda
    int evenCount = std::count_if(values.begin(), values.end(), [](int n) {
        return n % 2 == 0;
    });
    cout << "Even numbers count: " << evenCount << "\n";

    // I/O Manipulators
    cout << "\n--- I/O Manipulators ---\n";
    double pi = 3.14159265359;
    cout << "Default: " << pi << "\n";
    cout << "Fixed(2): " << std::fixed << std::setprecision(2) << pi << "\n";
    cout << "Scientific: " << std::scientific << pi << "\n";
    cout << std::defaultfloat;  // Reset to default

    // File operations
    cout << "\n--- File Operations ---\n";
    std::ofstream outFile("/tmp/test.txt");
    if (outFile.is_open()) {
        outFile << "Hello, File!\n";
        outFile << "Second line\n";
        outFile.close();
        cout << "File written successfully\n";
    }
    
    std::ifstream inFile("/tmp/test.txt");
    if (inFile.is_open()) {
        string line;
        cout << "File contents:\n";
        while (std::getline(inFile, line)) {
            cout << "  " << line << "\n";
        }
        inFile.close();
    }

    return 0;
}
