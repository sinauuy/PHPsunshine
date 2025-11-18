#!/bin/bash
# Quick start script for PHP Sunshine IDE

echo "☀️  Starting PHP Sunshine IDE..."
echo ""

# Source cargo environment
source "$HOME/.cargo/env"

# Run the IDE
cargo run --release

echo ""
echo "Thanks for using PHP Sunshine! 🌞"
