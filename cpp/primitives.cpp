// Primitive Types in C++
// Demonstrates all basic primitive types and their usage

#include <iostream>
#include <string>
#include <vector>
#include <array>
#include <limits>

int main() {
    std::cout << "=== C++ Primitive Types ===\n\n";

    // Integer types
    std::cout << "--- Integer Types ---\n";
    char c = 'A';
    signed char sc = -128;
    unsigned char uc = 255;
    short s = -32768;
    unsigned short us = 65535;
    int i = -2147483648;
    unsigned int ui = 4294967295;
    long l = -2147483648L;
    unsigned long ul = 4294967295UL;
    long long ll = -9223372036854775807LL;
    unsigned long long ull = 18446744073709551615ULL;

    std::cout << "char: " << c << " (ASCII: " << static_cast<int>(c) << ")\n";
    std::cout << "signed char: " << static_cast<int>(sc) << "\n";
    std::cout << "unsigned char: " << static_cast<int>(uc) << "\n";
    std::cout << "short: " << s << "\n";
    std::cout << "int: " << i << "\n";
    std::cout << "long long: " << ll << "\n";

    // Floating point types
    std::cout << "\n--- Floating Point Types ---\n";
    float f = 3.14159f;
    double d = 2.718281828459045;
    long double ld = 1.234567890123456789L;

    std::cout << "float: " << f << "\n";
    std::cout << "double: " << d << "\n";
    std::cout << "long double: " << ld << "\n";

    // Boolean type
    std::cout << "\n--- Boolean Type ---\n";
    bool is_true = true;
    bool is_false = false;

    std::cout << "true: " << std::boolalpha << is_true << "\n";
    std::cout << "false: " << is_false << "\n";

    // String types
    std::cout << "\n--- String Types ---\n";
    const char* c_string = "C-style string";
    std::string cpp_string = "C++ string";
    std::string concatenated = cpp_string + " with concatenation";

    std::cout << "C-string: " << c_string << "\n";
    std::cout << "C++ string: " << cpp_string << "\n";
    std::cout << "Concatenated: " << concatenated << "\n";

    // Pointers
    std::cout << "\n--- Pointer Types ---\n";
    int value = 42;
    int* ptr = &value;
    int** double_ptr = &ptr;

    std::cout << "Value: " << value << "\n";
    std::cout << "Pointer address: " << ptr << "\n";
    std::cout << "Pointer value: " << *ptr << "\n";
    std::cout << "Double pointer value: " << **double_ptr << "\n";

    // References
    std::cout << "\n--- Reference Types ---\n";
    int original = 100;
    int& ref = original;
    ref = 200;

    std::cout << "Original after modification via reference: " << original << "\n";

    // Arrays
    std::cout << "\n--- Array Types ---\n";
    int c_array[5] = {1, 2, 3, 4, 5};
    std::array<int, 5> std_array = {10, 20, 30, 40, 50};

    std::cout << "C-style array: ";
    for (int i = 0; i < 5; i++) {
        std::cout << c_array[i] << " ";
    }
    std::cout << "\n";

    std::cout << "std::array: ";
    for (const auto& elem : std_array) {
        std::cout << elem << " ";
    }
    std::cout << "\n";

    // Vectors (dynamic arrays)
    std::cout << "\n--- Vector Type ---\n";
    std::vector<int> vec = {1, 2, 3, 4, 5};
    vec.push_back(6);

    std::cout << "Vector: ";
    for (const auto& elem : vec) {
        std::cout << elem << " ";
    }
    std::cout << "\n";
    std::cout << "Vector size: " << vec.size() << "\n";

    // Auto keyword (type inference)
    std::cout << "\n--- Auto Keyword (Type Inference) ---\n";
    auto auto_int = 42;
    auto auto_double = 3.14;
    auto auto_string = std::string("Hello");

    std::cout << "auto int: " << auto_int << "\n";
    std::cout << "auto double: " << auto_double << "\n";
    std::cout << "auto string: " << auto_string << "\n";

    // Size information
    std::cout << "\n--- Type Sizes (in bytes) ---\n";
    std::cout << "char: " << sizeof(char) << "\n";
    std::cout << "short: " << sizeof(short) << "\n";
    std::cout << "int: " << sizeof(int) << "\n";
    std::cout << "long: " << sizeof(long) << "\n";
    std::cout << "long long: " << sizeof(long long) << "\n";
    std::cout << "float: " << sizeof(float) << "\n";
    std::cout << "double: " << sizeof(double) << "\n";
    std::cout << "pointer: " << sizeof(void*) << "\n";

    // Limits
    std::cout << "\n--- Type Limits ---\n";
    std::cout << "int max: " << std::numeric_limits<int>::max() << "\n";
    std::cout << "int min: " << std::numeric_limits<int>::min() << "\n";
    std::cout << "double max: " << std::numeric_limits<double>::max() << "\n";
    std::cout << "double min: " << std::numeric_limits<double>::min() << "\n";

    return 0;
}
