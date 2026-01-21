"""
Modules and Imports in Python
Demonstrates how to organize code with modules and import statements
"""

# Standard library imports
import os
import sys
import math
from datetime import datetime, timedelta
from collections import defaultdict, Counter
from typing import List, Dict, Optional, Union

# Import with alias
import json as js
from pathlib import Path as PathLib

def main():
    print("=== Python Modules and Imports ===\n")

    # Using os module
    print("--- os module ---")
    print(f"Current directory: {os.getcwd()}")
    print(f"Platform: {os.name}")
    print(f"Environment PATH: {os.environ.get('PATH', 'Not found')[:50]}...")

    # Using sys module
    print("\n--- sys module ---")
    print(f"Python version: {sys.version}")
    print(f"Platform: {sys.platform}")

    # Using math module
    print("\n--- math module ---")
    print(f"PI: {math.pi}")
    print(f"E: {math.e}")
    print(f"Square root of 16: {math.sqrt(16)}")
    print(f"Sin(90°): {math.sin(math.radians(90))}")

    # Using datetime
    print("\n--- datetime module ---")
    now = datetime.now()
    print(f"Current time: {now}")
    future = now + timedelta(days=7)
    print(f"One week from now: {future}")

    # Using collections
    print("\n--- collections module ---")
    
    # defaultdict
    word_count = defaultdict(int)
    for word in ["apple", "banana", "apple", "cherry", "banana", "apple"]:
        word_count[word] += 1
    print(f"Word count: {dict(word_count)}")
    
    # Counter
    letters = Counter("hello world")
    print(f"Letter frequency: {dict(letters)}")
    print(f"Most common: {letters.most_common(3)}")

    # Using typing for type hints
    print("\n--- typing module (Type Hints) ---")
    
    def greet(name: str) -> str:
        return f"Hello, {name}!"
    
    def process_items(items: List[int]) -> Dict[str, int]:
        return {
            "count": len(items),
            "sum": sum(items),
            "max": max(items) if items else 0
        }
    
    def find_user(user_id: int) -> Optional[str]:
        users = {1: "Alice", 2: "Bob"}
        return users.get(user_id)
    
    def format_value(value: Union[int, float, str]) -> str:
        return f"Value: {value}"
    
    print(greet("World"))
    print(f"Process items: {process_items([1, 2, 3, 4, 5])}")
    print(f"Find user 1: {find_user(1)}")
    print(f"Find user 99: {find_user(99)}")
    print(format_value(42))
    print(format_value(3.14))
    print(format_value("hello"))

    # Using json with alias
    print("\n--- json module (imported as js) ---")
    data = {"name": "Alice", "age": 30, "city": "New York"}
    json_string = js.dumps(data, indent=2)
    print(f"JSON:\n{json_string}")
    
    parsed = js.loads(json_string)
    print(f"Parsed back: {parsed}")

    # Using pathlib
    print("\n--- pathlib module (imported as PathLib) ---")
    current_path = PathLib.cwd()
    print(f"Current path: {current_path}")
    print(f"Is directory: {current_path.is_dir()}")
    print(f"Parent: {current_path.parent}")

    # Demonstrating module organization
    print("\n--- Module Organization ---")
    print("Standard import patterns:")
    print("1. import module")
    print("2. from module import function")
    print("3. from module import Class")
    print("4. import module as alias")
    print("5. from module import *  (avoid in production)")
    
    # __name__ usage
    print("\n--- __name__ variable ---")
    print(f"Current module name: {__name__}")
    print("When running directly: __name__ == '__main__'")
    print("When imported: __name__ == 'module_name'")

    # Demonstrating dir() to explore modules
    print("\n--- Exploring modules with dir() ---")
    print("Math module functions (first 10):")
    print(dir(math)[:10])

if __name__ == "__main__":
    main()
    
    # Additional demonstration
    print("\n--- Running as main module ---")
    print("This block only runs when the script is executed directly")
    print("Not when imported as a module")
