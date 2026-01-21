FROM python:3.11-slim

WORKDIR /app

# Copy all Python source files
COPY *.py .

# Run all examples
CMD echo "=== Running Primitives ===" && python3 primitives.py && \
    echo "\n=== Running Control Flow ===" && python3 control_flow.py && \
    echo "\n=== Running Classes ===" && python3 classes.py && \
    echo "\n=== Running Modules ===" && python3 modules.py
