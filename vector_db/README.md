# Vector Database Setup Guide

This guide explains how to set up and use the Qdrant vector database with Gemini embeddings for the UI components system.

All vector database related files are organized in the `vector_db/` folder.

## Files in this folder

- `upload.sh` - Main upload script with command-line options
- `upload_to_vector_db.rs` - Rust binary for uploading components
- `query_components.rs` - Rust binary for querying components
- `README.md` - This documentation

## Overview

The system uses **incremental embedding and upload** to handle large datasets efficiently:
- Processes components in configurable batches (default: 5)
- Generates embeddings and uploads each batch immediately
- Provides fault tolerance - if interrupted, only lose current batch progress
- Includes automatic retry with exponential backoff for API rate limits

## Prerequisites

1. **Qdrant Database**: Either local installation or Qdrant Cloud account
2. **Gemini API Key**: From Google AI Studio
3. **Rust**: For running the upload and query tools

## Environment Setup

Create a `.env` file in the **project root** (not in this folder):

```bash
# Required
GEMINI_API_KEY=your_gemini_api_key_here

# For Qdrant Cloud (recommended)
QDRANT_URL=https://your-cluster-url.qdrant.io:6333
QDRANT_API_KEY=your_qdrant_api_key_here

# For local Qdrant (alternative)
# QDRANT_URL=http://localhost:6333
# No API key needed for local
```

### Getting API Keys

**Gemini API Key:**
1. Visit [Google AI Studio](https://aistudio.google.com/)
2. Create a new API key
3. Copy the key to your `.env` file

**Qdrant Cloud Setup:**
1. Create account at [Qdrant Cloud](https://qdrant.tech/cloud/)
2. Create a new cluster
3. Copy the cluster URL and API key to your `.env` file

## Usage

### Upload Components

From the project root or vector_db folder:

```bash
# Upload all components with default settings (batch size: 5)
./vector_db/upload.sh

# Or from within this folder:
cd vector_db
./upload.sh

# Custom batch size
./upload.sh --batch-size 3

# Custom delay between API calls (to avoid rate limits)
./upload.sh --delay 200

# Different collection name
./upload.sh --collection my_components

# See all options
./upload.sh --help
```

### Query Components

From the project root:

```bash
# Basic query
cargo run --bin query_components -- "button component for forms"

# With custom options
cargo run --bin query_components -- "navigation menu" --collection components --limit 5

# See all query options
cargo run --bin query_components -- --help
```

### Command Line Options

**Upload Script (`./upload.sh`):**
- `-c, --components-file FILE`: Path to components JSON file (auto-detected)
- `--collection NAME`: Qdrant collection name (default: `components`)
- `-d, --delay MS`: Delay between API calls in milliseconds (default: `100`)
- `-m, --max-retries N`: Maximum retries for rate-limited requests (default: `3`)
- `-b, --batch-size N`: Batch size for incremental uploads (default: `5`)

**Query Binary:**
- `--query, -q`: Search query (required)
- `--collection NAME`: Qdrant collection name (default: `components`)
- `--limit, -l`: Number of results to return (default: `5`)

## How It Works

### Incremental Upload Process

1. **Load Components**: Reads all components from `components.json`
2. **Create Collection**: Sets up Qdrant collection with proper vector dimensions (3072 for gemini-embedding-exp-03-07)
3. **Batch Processing**:
   - Process components in batches (default: 5)
   - Generate embeddings for each component in the batch
   - Upload the entire batch to Qdrant immediately
   - Continue to next batch
4. **Progress Tracking**: Shows detailed progress with batch-by-batch updates
5. **Fault Tolerance**: If interrupted, can resume from last completed batch

### Rate Limiting & Retries

- **Automatic Retry**: Exponential backoff for 429 (rate limit) errors
- **Configurable Delays**: Adjust delay between API calls
- **Max Retries**: Set maximum retry attempts (default: 3)

### Example Output

```
🚀 Starting UI components vector upload...
📁 Using components file: components.json
🎯 Collection: components
⏱️  Delay: 100ms
🔄 Max retries: 3
📦 Batch size: 5

Loading components from components.json...
Loaded 55 components
Connecting to Qdrant at https://your-cluster.qdrant.io:6333...
✅ Created collection 'components'

📋 Processing batch 1 (5/55 components)
Generating embedding 1/55
Generating embedding 2/55
...
Generating embedding 5/55
✅ Batch 1 uploaded (5/55 components completed)

📋 Processing batch 2 (10/55 components)
...
```

## Data Structure

Each component is stored in Qdrant with:
- **Vector**: 768-dimensional Gemini embedding
- **Metadata**:
  - `component_id`: Original component identifier
  - `name`: Component name
  - `parent`: Parent component (if any)
  - `description`: Component description
  - `category`: Component category
  - `usage`: Usage information
  - `examples`: Array of example use cases
  - `embedding_text`: The text used to generate the embedding

## Troubleshooting

### Common Issues

**Rate Limiting (429 errors):**
- Increase delay: `./upload.sh --delay 200`
- Reduce batch size: `./upload.sh --batch-size 3`
- The system will automatically retry with exponential backoff

**Connection Issues:**
- Verify Qdrant URL and API key in `.env`
- Check network connectivity to Qdrant Cloud
- For local Qdrant, ensure it's running on port 6333

**Memory Issues:**
- Reduce batch size: `./upload.sh --batch-size 2`
- The incremental approach minimizes memory usage

### Recovery from Interruption

If the upload is interrupted:
1. Simply run the upload script again
2. The system will recreate the collection if needed
3. It will start processing from the beginning
4. Only the current batch in progress will be lost

### Monitoring Progress

The system provides detailed progress updates:
- Batch-by-batch progress tracking
- Individual embedding generation progress
- Upload confirmation for each batch
- Total completion status

## Performance Tips

1. **Batch Size**: Balance between progress saving and efficiency
   - Smaller batches (2-3): Better fault tolerance, more frequent saves
   - Larger batches (5-10): Better throughput, less overhead

2. **Delay Settings**: Adjust based on your API quota
   - Higher delay (200-500ms): For limited quota or rate limits
   - Lower delay (50-100ms): For higher quota allowances

3. **Retry Settings**: Configure based on reliability needs
   - Higher retries: For unreliable networks
   - Lower retries: For fast failure detection

## Integration

The uploaded vectors can be queried using:
- The included `query_components` binary
- Direct Qdrant API calls
- Any Qdrant SDK (Python, JavaScript, etc.)

The vector database enables semantic search over UI components, allowing natural language queries to find relevant components based on functionality, appearance, and use cases.
