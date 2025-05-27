#!/bin/bash

# DSA Learning Assistant Runner Script

echo "🎯 DSA Learning Assistant"
echo "=========================="
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the dsa_learning_assistant directory"
    exit 1
fi

# Check for API key
if [ -z "$GEMINI_API_KEY" ]; then
    echo "⚠️  Warning: GEMINI_API_KEY not set"
    echo "   Set your API key for full functionality:"
    echo "   export GEMINI_API_KEY='your_api_key_here'"
    echo
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Build and run
echo "🔨 Building application..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🚀 Starting DSA Learning Assistant..."
    echo
    cargo run --release
else
    echo "❌ Build failed!"
    exit 1
fi
