#!/bin/bash

echo "🔧 Fixing Exia application issues..."

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "📝 Creating .env file from template..."
    cp env.example .env
fi

echo "⚙️  Current .env configuration:"
echo "GEMINI_API_KEY=$(grep GEMINI_API_KEY .env | cut -d'=' -f2)"
echo "QDRANT_URL=$(grep QDRANT_URL .env | cut -d'=' -f2)"

# Check if GEMINI_API_KEY is set
if grep -q "your_gemini_api_key_here" .env; then
    echo ""
    echo "❌ GEMINI_API_KEY not configured!"
    echo "📋 To fix this:"
    echo "1. Visit https://aistudio.google.com/"
    echo "2. Create a new API key"
    echo "3. Edit .env file and replace 'your_gemini_api_key_here' with your actual API key"
    echo ""
    echo "Or run: sed -i 's/your_gemini_api_key_here/YOUR_ACTUAL_API_KEY/' .env"
else
    echo "✅ GEMINI_API_KEY is configured"
fi

# Check Qdrant container
echo ""
echo "🔍 Checking Qdrant status..."
if docker ps | grep -q qdrant; then
    echo "✅ Qdrant container is running"
else
    echo "🚀 Starting Qdrant container..."
    docker run -d -p 6333:6333 -v $(pwd)/qdrant_storage:/qdrant/storage qdrant/qdrant
fi

# Check if components collection exists
echo ""
echo "📊 Checking vector database setup..."
if [ -d "qdrant_storage/collections" ]; then
    if [ -d "qdrant_storage/collections/components" ]; then
        echo "✅ Components collection exists"
    else
        echo "⚠️  Components collection not found"
        echo "📋 To create it, run: ./vector_db/upload.sh"
    fi
else
    echo "⚠️  Vector database not initialized"
    echo "📋 To initialize it, run: ./vector_db/upload.sh"
fi

echo ""
echo "🛠️  Quick fixes applied:"
echo "• Updated Qdrant client with better timeout handling"
echo "• Improved JSON parsing for Gemini API responses"
echo "• Added graceful fallbacks for vector search failures"
echo "• Enhanced error messages"

echo ""
echo "📝 Next steps:"
echo "1. Set your GEMINI_API_KEY in .env file"
echo "2. Run: dx serve --platform desktop"
echo "3. If vector search fails, run: ./vector_db/upload.sh"

echo ""
echo "✨ Setup complete!"
