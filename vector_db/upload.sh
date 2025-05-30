#!/bin/bash

# Exit on any error
set -e

echo "🚀 Starting UI components vector upload..."

# Default values - will be adjusted based on where script is run from
COMPONENTS_FILE=""
COLLECTION="components"
DELAY=100
MAX_RETRIES=3
BATCH_SIZE=5

# Determine correct path to components.json based on where script is run
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if we're in the vector_db folder or project root
if [ -f "./components.json" ]; then
    # Running from project root
    COMPONENTS_FILE="./components.json"
elif [ -f "../components.json" ]; then
    # Running from vector_db folder
    COMPONENTS_FILE="../components.json"
else
    # Default fallback
    COMPONENTS_FILE="components.json"
fi

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--components-file)
            COMPONENTS_FILE="$2"
            shift 2
            ;;
        --collection)
            COLLECTION="$2"
            shift 2
            ;;
        -d|--delay)
            DELAY="$2"
            shift 2
            ;;
        -m|--max-retries)
            MAX_RETRIES="$2"
            shift 2
            ;;
        -b|--batch-size)
            BATCH_SIZE="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -c, --components-file FILE  Path to components JSON file (auto-detected)"
            echo "  --collection NAME           Qdrant collection name (default: components)"
            echo "  -d, --delay MS             Delay between API calls in milliseconds (default: 100)"
            echo "  -m, --max-retries N        Maximum retries for rate-limited requests (default: 3)"
            echo "  -b, --batch-size N         Batch size for incremental uploads (default: 5)"
            echo "  -h, --help                 Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

# Check if components file exists
if [ ! -f "$COMPONENTS_FILE" ]; then
    echo "❌ Error: Components file '$COMPONENTS_FILE' not found"
    echo "💡 Tried looking in:"
    echo "   - ./components.json (from project root)"
    echo "   - ../components.json (from vector_db folder)"
    echo ""
    echo "Please run this script from either:"
    echo "   - Project root: ./vector_db/upload.sh"
    echo "   - Vector_db folder: cd vector_db && ./upload.sh"
    exit 1
fi

# Check if required environment variables are set
if [ -z "$GEMINI_API_KEY" ]; then
    echo "❌ Error: GEMINI_API_KEY environment variable is required"
    exit 1
fi

echo "📁 Using components file: $COMPONENTS_FILE"
echo "🎯 Collection: $COLLECTION"
echo "⏱️  Delay: ${DELAY}ms"
echo "🔄 Max retries: $MAX_RETRIES"
echo "📦 Batch size: $BATCH_SIZE"
echo ""

# Change to project root directory to ensure cargo works correctly
cd "$PROJECT_ROOT"

# Run the Rust binary
cargo run --bin upload_to_vector_db -- \
    --components-file "$COMPONENTS_FILE" \
    --collection "$COLLECTION" \
    --delay "$DELAY" \
    --max-retries "$MAX_RETRIES" \
    --batch-size "$BATCH_SIZE"

echo ""
echo "🎉 Upload process completed!"
