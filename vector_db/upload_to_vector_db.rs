use anyhow::{Context, Result};
use clap::Parser;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    CreateCollection, Distance, PointStruct, UpsertPoints, VectorParams, VectorsConfig,
};
use qdrant_client::Qdrant;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to components JSON file
    #[arg(short, long, default_value = "vector_db/components.json")]
    components_file: String,

    /// Qdrant collection name
    #[arg(long, default_value = "components")]
    collection: String,

    /// Delay between API calls in milliseconds
    #[arg(short, long, default_value = "100")]
    delay: u64,

    /// Maximum retries for rate-limited requests
    #[arg(short, long, default_value = "3")]
    max_retries: u32,

    /// Batch size for incremental uploads (saves every N embeddings)
    #[arg(short, long, default_value = "5")]
    batch_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct ComponentData {
    id: String,
    name: String,
    parent: String,
    description: String,
    category: String,
    usage: String,
    examples: Vec<String>,
    #[serde(rename = "embeddingText")]
    embedding_text: String,
}

#[derive(Debug, Clone)]
struct Component {
    data: ComponentData,
    embedding: Option<Vec<f32>>,
}

#[derive(Debug, Serialize)]
struct GeminiEmbedRequest {
    model: String,
    content: GeminiContent,
    #[serde(rename = "taskType")]
    task_type: String,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiEmbedResponse {
    embedding: GeminiEmbedding,
}

#[derive(Debug, Deserialize)]
struct GeminiEmbedding {
    values: Vec<f64>,
}

struct GeminiClient {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
    max_retries: u32,
}

impl GeminiClient {
    fn new(api_key: String, max_retries: u32) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models".to_string(),
            model: "gemini-embedding-exp-03-07".to_string(),
            max_retries,
        }
    }

    async fn generate_embedding_with_retry(&self, text: &str, task_type: &str) -> Result<Vec<f32>> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay = Duration::from_secs(2_u64.pow(attempt));
                println!(
                    "  ⏳ Rate limited, waiting {}s before retry {}/{}",
                    delay.as_secs(),
                    attempt,
                    self.max_retries
                );
                sleep(delay).await;
            }

            match self.generate_embedding(text, task_type).await {
                Ok(embedding) => return Ok(embedding),
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.contains("429") && attempt < self.max_retries {
                        last_error = Some(e);
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Max retries exceeded")))
    }

    async fn generate_embedding(&self, text: &str, task_type: &str) -> Result<Vec<f32>> {
        let url = format!("{}/{}:embedContent", self.base_url, self.model);

        let request = GeminiEmbedRequest {
            model: format!("models/{}", self.model),
            content: GeminiContent {
                parts: vec![GeminiPart {
                    text: text.to_string(),
                }],
            },
            task_type: task_type.to_string(),
        };

        let response = self
            .client
            .post(&url)
            .query(&[("key", &self.api_key)])
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let embed_response: GeminiEmbedResponse = response
            .json()
            .await
            .context("Failed to parse Gemini API response")?;

        // Convert f64 to f32 for Qdrant
        let embedding: Vec<f32> = embed_response
            .embedding
            .values
            .into_iter()
            .map(|x| x as f32)
            .collect();

        Ok(embedding)
    }
}

async fn create_qdrant_client(url: &str, api_key: Option<&str>) -> Result<Qdrant> {
    let mut client_builder = Qdrant::from_url(url);

    if let Some(key) = api_key {
        client_builder = client_builder.api_key(key);
    }

    // Add improved timeout settings and keep-alive to handle HTTP/2 issues
    let client = client_builder
        .timeout(std::time::Duration::from_secs(60))
        .connect_timeout(std::time::Duration::from_secs(30))
        .keep_alive_while_idle()
        .build()
        .context("Failed to build Qdrant client")?;
    Ok(client)
}

async fn create_collection_if_not_exists(
    client: &Qdrant,
    collection_name: &str,
    vector_size: u64,
) -> Result<()> {
    // Check if collection exists
    let collections = client
        .list_collections()
        .await
        .context("Failed to list collections")?;

    let collection_exists = collections
        .collections
        .iter()
        .any(|c| c.name == collection_name);

    if collection_exists {
        // Check collection info to verify dimensions
        let collection_info = client
            .collection_info(collection_name)
            .await
            .context("Failed to get collection info")?;

        if let Some(config) = collection_info.result.and_then(|r| r.config) {
            if let Some(vectors_config) = config.params.and_then(|p| p.vectors_config) {
                if let Some(qdrant_client::qdrant::vectors_config::Config::Params(params)) =
                    vectors_config.config
                {
                    if params.size == vector_size {
                        println!(
                            "ℹ️  Collection '{}' already exists with correct dimensions ({})",
                            collection_name, vector_size
                        );
                        return Ok(());
                    } else {
                        println!("⚠️  Collection '{}' exists but has wrong dimensions ({} vs {}). Deleting and recreating...",
                               collection_name, params.size, vector_size);

                        // Delete the existing collection
                        client
                            .delete_collection(collection_name)
                            .await
                            .context("Failed to delete existing collection")?;

                        println!("🗑️  Deleted collection '{}'", collection_name);
                    }
                }
            }
        }
    }

    // Create collection
    let vectors_config = VectorsConfig {
        config: Some(Config::Params(VectorParams {
            size: vector_size,
            distance: Distance::Cosine.into(),
            ..Default::default()
        })),
    };

    client
        .create_collection(CreateCollection {
            collection_name: collection_name.to_string(),
            vectors_config: Some(vectors_config),
            ..Default::default()
        })
        .await
        .context("Failed to create collection")?;

    println!(
        "✅ Created collection '{}' with {} dimensions",
        collection_name, vector_size
    );
    Ok(())
}

async fn upload_batch(
    client: &Qdrant,
    collection_name: &str,
    components: &[Component],
    batch_index: usize,
) -> Result<()> {
    let points: Vec<PointStruct> = components
        .iter()
        .map(|component| {
            let embedding = component
                .embedding
                .as_ref()
                .expect("Component missing embedding");

            let mut payload = std::collections::HashMap::new();
            payload.insert("component_id".to_string(), component.data.id.clone().into());
            payload.insert("name".to_string(), component.data.name.clone().into());
            payload.insert("parent".to_string(), component.data.parent.clone().into());
            payload.insert(
                "description".to_string(),
                component.data.description.clone().into(),
            );
            payload.insert(
                "category".to_string(),
                component.data.category.clone().into(),
            );
            payload.insert("usage".to_string(), component.data.usage.clone().into());
            payload.insert(
                "examples".to_string(),
                component.data.examples.clone().into(),
            );
            payload.insert(
                "embedding_text".to_string(),
                component.data.embedding_text.clone().into(),
            );

            // Generate a UUID for Qdrant while keeping the original ID in metadata
            let point_id = Uuid::new_v4().to_string();
            PointStruct::new(point_id, embedding.clone(), payload)
        })
        .collect();

    let upsert_request = UpsertPoints {
        collection_name: collection_name.to_string(),
        points,
        ..Default::default()
    };

    client
        .upsert_points(upsert_request)
        .await
        .with_context(|| format!("Failed to upsert batch {} to Qdrant", batch_index + 1))?;

    Ok(())
}

async fn generate_and_upload_incremental(
    gemini_client: &GeminiClient,
    qdrant_client: &Qdrant,
    collection_name: &str,
    mut components: Vec<Component>,
    delay_ms: u64,
    batch_size: usize,
) -> Result<()> {
    let total = components.len();
    let mut completed = 0;

    // Process in batches for incremental upload
    for (batch_index, batch_start) in (0..total).step_by(batch_size).enumerate() {
        let batch_end = std::cmp::min(batch_start + batch_size, total);
        let current_batch_size = batch_end - batch_start;

        println!(
            "\n📋 Processing batch {} ({}/{} components)",
            batch_index + 1,
            batch_end,
            total
        );

        // Generate embeddings for this batch
        for i in batch_start..batch_end {
            let component_index = i + 1;
            println!("Generating embedding {}/{}", component_index, total);

            let embedding = gemini_client
                .generate_embedding_with_retry(
                    &components[i].data.embedding_text,
                    "RETRIEVAL_DOCUMENT",
                )
                .await?;

            components[i].embedding = Some(embedding);

            // Rate limiting (except for last item in batch)
            if i < batch_end - 1 {
                sleep(Duration::from_millis(delay_ms)).await;
            }
        }

        // Upload this batch immediately
        let batch_components = &components[batch_start..batch_end];
        upload_batch(
            qdrant_client,
            collection_name,
            batch_components,
            batch_index,
        )
        .await?;

        completed += current_batch_size;
        println!(
            "✅ Batch {} uploaded ({}/{} components completed)",
            batch_index + 1,
            completed,
            total
        );
    }

    Ok(())
}

fn load_components(file_path: &str) -> Result<Vec<Component>> {
    let file_content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    let component_data: Vec<ComponentData> =
        serde_json::from_str(&file_content).context("Failed to parse JSON")?;

    let components = component_data
        .into_iter()
        .map(|data| Component {
            data,
            embedding: None,
        })
        .collect();

    Ok(components)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    // Get configuration from environment variables
    let gemini_api_key =
        env::var("GEMINI_API_KEY").context("GEMINI_API_KEY environment variable is required")?;

    let qdrant_url = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6334".to_string());
    let qdrant_api_key = env::var("QDRANT_API_KEY").ok();

    println!("Loading components from {}...", args.components_file);
    let components = load_components(&args.components_file)?;
    println!("Loaded {} components", components.len());

    println!("Connecting to Qdrant at {}...", qdrant_url);
    let qdrant_client = create_qdrant_client(&qdrant_url, qdrant_api_key.as_deref()).await?;

    // Create collection (3072 is the dimension for gemini-embedding-exp-03-07 embeddings)
    create_collection_if_not_exists(&qdrant_client, &args.collection, 3072).await?;

    println!(
        "Starting incremental upload with embeddings (batch size: {})...",
        args.batch_size
    );
    let gemini_client = GeminiClient::new(gemini_api_key, args.max_retries);

    generate_and_upload_incremental(
        &gemini_client,
        &qdrant_client,
        &args.collection,
        components,
        args.delay,
        args.batch_size,
    )
    .await?;

    println!("✅ Upload completed successfully!");

    Ok(())
}
