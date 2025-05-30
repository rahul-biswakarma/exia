#!/bin/bash

# Exit on any error
set -e

echo "🚀 Starting comprehensive vector database upload..."

# Default values
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
COMPONENTS_FILE="$SCRIPT_DIR/components.json"
ACTIONS_FILE="$SCRIPT_DIR/actions_definitions.json"
DELAY=100
MAX_RETRIES=3
BATCH_SIZE=5

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -c|--components-file)
            COMPONENTS_FILE="$2"
            shift 2
            ;;
        -a|--actions-file)
            ACTIONS_FILE="$2"
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
        --components-only)
            ACTIONS_FILE=""
            shift
            ;;
        --actions-only)
            COMPONENTS_FILE=""
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -c, --components-file FILE  Path to components JSON file (default: vector_db/components.json)"
            echo "  -a, --actions-file FILE     Path to actions definitions JSON file (default: vector_db/actions_definitions.json)"
            echo "  -d, --delay MS             Delay between API calls in milliseconds (default: 100)"
            echo "  -m, --max-retries N        Maximum retries for rate-limited requests (default: 3)"
            echo "  -b, --batch-size N         Batch size for incremental uploads (default: 5)"
            echo "  --components-only          Upload only components"
            echo "  --actions-only             Upload only actions definitions"
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

# Check if required environment variables are set
if [ -z "$GEMINI_API_KEY" ]; then
    echo "❌ Error: GEMINI_API_KEY environment variable is required"
    exit 1
fi

# Change to project root directory to ensure cargo works correctly
cd "$PROJECT_ROOT"

echo "📁 Project root: $PROJECT_ROOT"
echo "⏱️  Delay: ${DELAY}ms"
echo "🔄 Max retries: $MAX_RETRIES"
echo "📦 Batch size: $BATCH_SIZE"
echo ""

# Upload components if specified
if [ -n "$COMPONENTS_FILE" ]; then
    if [ -f "$COMPONENTS_FILE" ]; then
        echo "🧩 Uploading UI Components..."
        echo "📁 Components file: $COMPONENTS_FILE"

        cargo run --bin upload_to_vector_db -- \
            --components-file "$COMPONENTS_FILE" \
            --collection "components" \
            --delay "$DELAY" \
            --max-retries "$MAX_RETRIES" \
            --batch-size "$BATCH_SIZE"

        echo "✅ Components upload completed!"
        echo ""
    else
        echo "❌ Error: Components file '$COMPONENTS_FILE' not found"
        exit 1
    fi
fi

# Upload actions definitions if specified and if we have a script for it
if [ -n "$ACTIONS_FILE" ]; then
    if [ -f "$ACTIONS_FILE" ]; then
        echo "⚡ Actions definitions file found: $ACTIONS_FILE"
        echo "💡 Note: Actions definitions upload not yet implemented."
        echo "   You can extend the upload_to_vector_db.rs script to handle actions definitions."
        echo ""
    else
        echo "❌ Error: Actions definitions file '$ACTIONS_FILE' not found"
        exit 1
    fi
fi

echo "🎉 Vector database upload process completed!"
echo ""
echo "📊 Summary:"
if [ -n "$COMPONENTS_FILE" ] && [ -f "$COMPONENTS_FILE" ]; then
    COMPONENT_COUNT=$(jq length "$COMPONENTS_FILE" 2>/dev/null || echo "N/A")
    echo "   - UI Components: $COMPONENT_COUNT uploaded to 'components' collection"
fi
if [ -n "$ACTIONS_FILE" ] && [ -f "$ACTIONS_FILE" ]; then
    ACTION_COUNT=$(jq '.actions | length' "$ACTIONS_FILE" 2>/dev/null || echo "N/A")
    echo "   - Actions Definitions: $ACTION_COUNT found (upload pending implementation)"
fi
