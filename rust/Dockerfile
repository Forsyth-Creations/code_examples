FROM rust:1.75

WORKDIR /app

# Copy all Rust source files
COPY *.rs .

# Compile all examples
RUN rustc primitives.rs && \
    rustc control_flow.rs && \
    rustc structs.rs && \
    rustc enums.rs && \
    rustc modules.rs

# Run all examples
CMD echo "=== Running Primitives ===" && ./primitives && \
    echo "\n=== Running Control Flow ===" && ./control_flow && \
    echo "\n=== Running Structs ===" && ./structs && \
    echo "\n=== Running Enums ===" && ./enums && \
    echo "\n=== Running Modules ===" && ./modules
