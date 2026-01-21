FROM gcc:13

WORKDIR /app

# Copy all C++ source files
COPY *.cpp .

# Compile all examples
RUN g++ -std=c++17 primitives.cpp -o primitives && \
    g++ -std=c++17 control_flow.cpp -o control_flow && \
    g++ -std=c++17 classes.cpp -o classes && \
    g++ -std=c++17 enums.cpp -o enums && \
    g++ -std=c++17 modules.cpp -o modules

# Run all examples
CMD echo "=== Running Primitives ===" && ./primitives && \
    echo "\n=== Running Control Flow ===" && ./control_flow && \
    echo "\n=== Running Classes ===" && ./classes && \
    echo "\n=== Running Enums ===" && ./enums && \
    echo "\n=== Running Modules ===" && ./modules
