"""
Primitive Types in Python
Demonstrates all basic primitive types and their usage
"""

def main():
    print("=== Python Primitive Types ===\n")

    # Integer types (Python 3 has arbitrary precision integers)
    print("--- Integer Types ---")
    small_int = 42
    large_int = 123456789012345678901234567890
    negative_int = -999
    hex_int = 0xFF  # 255 in hexadecimal
    octal_int = 0o77  # 63 in octal
    binary_int = 0b1010  # 10 in binary

    print(f"Small integer: {small_int}")
    print(f"Large integer: {large_int}")
    print(f"Negative: {negative_int}")
    print(f"Hexadecimal (0xFF): {hex_int}")
    print(f"Octal (0o77): {octal_int}")
    print(f"Binary (0b1010): {binary_int}")

    # Floating point types
    print("\n--- Floating Point Types ---")
    float_num = 3.14159
    scientific = 1.5e10  # 15000000000.0
    negative_float = -2.718

    print(f"Float: {float_num}")
    print(f"Scientific notation: {scientific}")
    print(f"Negative float: {negative_float}")

    # Complex numbers
    print("\n--- Complex Numbers ---")
    complex_num = 3 + 4j
    another_complex = complex(2, -5)

    print(f"Complex number: {complex_num}")
    print(f"Another complex: {another_complex}")
    print(f"Real part: {complex_num.real}")
    print(f"Imaginary part: {complex_num.imag}")

    # Boolean type
    print("\n--- Boolean Type ---")
    is_true = True
    is_false = False

    print(f"True: {is_true}")
    print(f"False: {is_false}")
    print(f"Boolean from comparison: {5 > 3}")

    # String types
    print("\n--- String Types ---")
    single_quote = 'Hello'
    double_quote = "World"
    multi_line = """This is a
    multi-line
    string"""
    raw_string = r"C:\path\to\file"
    f_string = f"Interpolated: {small_int}"

    print(f"Single quote: {single_quote}")
    print(f"Double quote: {double_quote}")
    print(f"Multi-line: {multi_line}")
    print(f"Raw string: {raw_string}")
    print(f"F-string: {f_string}")

    # Bytes type
    print("\n--- Bytes Type ---")
    byte_literal = b"Hello"
    byte_array = bytearray(b"World")

    print(f"Bytes: {byte_literal}")
    print(f"Bytearray: {byte_array}")

    # None type
    print("\n--- None Type ---")
    none_value = None
    print(f"None: {none_value}")
    print(f"Type: {type(none_value)}")

    # List (mutable sequence)
    print("\n--- List Type ---")
    numbers = [1, 2, 3, 4, 5]
    mixed = [1, "two", 3.0, True]

    print(f"Numbers list: {numbers}")
    print(f"Mixed list: {mixed}")
    print(f"First element: {numbers[0]}")

    # Tuple (immutable sequence)
    print("\n--- Tuple Type ---")
    coordinates = (10, 20)
    single_item = (42,)  # Note the comma
    mixed_tuple = (1, "hello", 3.14)

    print(f"Tuple: {coordinates}")
    print(f"Single item tuple: {single_item}")
    print(f"Mixed tuple: {mixed_tuple}")

    # Set (unordered unique elements)
    print("\n--- Set Type ---")
    unique_numbers = {1, 2, 3, 4, 5}
    unique_numbers.add(3)  # Won't add duplicate

    print(f"Set: {unique_numbers}")
    print(f"Length: {len(unique_numbers)}")

    # Dictionary (key-value pairs)
    print("\n--- Dictionary Type ---")
    person = {
        "name": "Alice",
        "age": 30,
        "city": "New York"
    }

    print(f"Dictionary: {person}")
    print(f"Name: {person['name']}")

    # Type checking
    print("\n--- Type Checking ---")
    print(f"Type of integer: {type(small_int)}")
    print(f"Type of float: {type(float_num)}")
    print(f"Type of string: {type(single_quote)}")
    print(f"Type of list: {type(numbers)}")
    print(f"Type of dict: {type(person)}")

if __name__ == "__main__":
    main()
