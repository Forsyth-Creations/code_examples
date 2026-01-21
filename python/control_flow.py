"""
Control Flow in Python
Demonstrates if statements, loops, and pattern matching
"""

def main():
    print("=== Python Control Flow ===\n")

    # If statements
    print("--- If Statements ---")
    number = 42
    
    if number > 0:
        print(f"{number} is positive")
    elif number < 0:
        print(f"{number} is negative")
    else:
        print(f"{number} is zero")

    # Ternary operator
    result = "even" if number % 2 == 0 else "odd"
    print(f"{number} is {result}")

    # For loop with list
    print("\n--- For Loop with List ---")
    fruits = ["apple", "banana", "cherry"]
    for fruit in fruits:
        print(f"Fruit: {fruit}")

    # For loop with range
    print("\n--- For Loop with Range ---")
    for i in range(5):
        print(f"Index: {i}")

    # For loop with range (start, stop, step)
    print("\nFor with custom range:")
    for i in range(1, 10, 2):
        print(f"Odd number: {i}")

    # For loop with enumerate
    print("\n--- For Loop with Enumerate ---")
    colors = ["red", "green", "blue"]
    for index, color in enumerate(colors):
        print(f"{index}: {color}")

    # While loop
    print("\n--- While Loop ---")
    countdown = 3
    while countdown > 0:
        print(f"Countdown: {countdown}")
        countdown -= 1
    print("Liftoff!")

    # Break and continue
    print("\n--- Break and Continue ---")
    for i in range(10):
        if i == 3:
            continue  # Skip 3
        if i == 7:
            break  # Stop at 7
        print(f"Number: {i}")

    # For-else (executes if loop completes without break)
    print("\n--- For-Else ---")
    for i in range(3):
        print(f"Loop {i}")
    else:
        print("Loop completed normally")

    # While-else
    print("\n--- While-Else ---")
    count = 0
    while count < 3:
        print(f"Count: {count}")
        count += 1
    else:
        print("While completed normally")

    # Match statement (Python 3.10+)
    print("\n--- Match Statement (Pattern Matching) ---")
    try:
        point = (0, 0)
        match point:
            case (0, 0):
                print("Origin")
            case (0, y):
                print(f"On Y-axis at {y}")
            case (x, 0):
                print(f"On X-axis at {x}")
            case (x, y):
                print(f"Point at ({x}, {y})")
            case _:
                print("Not a point")

        # Match with conditions
        value = 15
        match value:
            case n if n < 0:
                print("Negative")
            case 0:
                print("Zero")
            case n if n % 2 == 0:
                print("Even positive")
            case _:
                print("Odd positive")

    except SyntaxError:
        print("Match statements require Python 3.10+")

    # List comprehension
    print("\n--- List Comprehension ---")
    squares = [x**2 for x in range(5)]
    print(f"Squares: {squares}")

    evens = [x for x in range(10) if x % 2 == 0]
    print(f"Even numbers: {evens}")

    # Dictionary comprehension
    print("\n--- Dictionary Comprehension ---")
    square_dict = {x: x**2 for x in range(5)}
    print(f"Square dictionary: {square_dict}")

    # Set comprehension
    print("\n--- Set Comprehension ---")
    unique_squares = {x**2 for x in [-2, -1, 0, 1, 2]}
    print(f"Unique squares: {unique_squares}")

    # Nested loops
    print("\n--- Nested Loops ---")
    for i in range(3):
        for j in range(3):
            print(f"({i}, {j})", end=" ")
        print()

    # Exception handling
    print("\n--- Exception Handling ---")
    try:
        result = 10 / 0
    except ZeroDivisionError as e:
        print(f"Error caught: {e}")
    except Exception as e:
        print(f"General error: {e}")
    else:
        print("No exception occurred")
    finally:
        print("Cleanup code runs always")

    # With statement (context manager)
    print("\n--- With Statement ---")
    with open("/tmp/test.txt", "w") as f:
        f.write("Hello, World!")
    print("File written and automatically closed")

if __name__ == "__main__":
    main()
