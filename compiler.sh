#!/bin/bash

# Path to the directory containing your Rust files
RUST_FILES_DIR="Path/to/your/rust_src"

# Change to the directory containing the Rust files
cd "$RUST_FILES_DIR"

# Compile each Rust file
rustc init_lifetime.rs
rustc inter_arc.rs
rustc inter_lifetime.rs
rustc main_concurrence.rs

echo "Compilation complete."
