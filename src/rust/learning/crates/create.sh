#!/bin/bash

# Check if crate name is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <crate_name>"
    exit 1
fi

CRATE_NAME=$1

# if the crate directory already exists, delete it but warn the user first
if [ -d "$CRATE_NAME" ]; then
    echo "Warning: Directory '$CRATE_NAME' already exists. Deleting it..."
    rm -rf "$CRATE_NAME"
fi

# Create new crate using cargo
cargo new "$CRATE_NAME"

echo "Crate '$CRATE_NAME' created successfully!"