use anyhow::{Context, Result};
use clap::Parser;
use qdrant_client::qdrant::{ScoredPoint, SearchPoints};
use qdrant_client::Qdrant;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Search query
    #[arg(short, long)]
    query: String,

    /// Qdrant collection name to search in
    #[arg(long, default_value = "components")]
    collection: String,

    /// Number of results to return
    #[arg(short, long, default_value = "5")]
    top_k: usize,
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
}

impl GeminiClient {
    fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://generativelanguage.googleapis.com/v1beta/models".to_string(),
            model: "gemini-embedding-exp-03-07".to_string(),
        }
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

    // Add timeout and connection settings to avoid HTTP/2 frame size errors
    let client = client_builder
        .timeout(std::time::Duration::from_secs(60))
        .connect_timeout(std::time::Duration::from_secs(30))
        .build()
        .context("Failed to build Qdrant client")?;
    Ok(client)
}

async fn search_components(
    client: &Qdrant,
    collection_name: &str,
    query_vector: Vec<f32>,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    let search_points = SearchPoints {
        collection_name: collection_name.to_string(),
        vector: query_vector,
        limit: limit as u64,
        with_payload: Some(true.into()),
        ..Default::default()
    };

    let search_result = client
        .search_points(search_points)
        .await
        .context("Failed to search in Qdrant")?;

    let results: Vec<SearchResult> = search_result
        .result
        .into_iter()
        .map(|point: ScoredPoint| {
            let id = match point.id.unwrap().point_id_options {
                Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(uuid)) => uuid,
                Some(qdrant_client::qdrant::point_id::PointIdOptions::Num(num)) => num.to_string(),
                None => "unknown".to_string(),
            };

            SearchResult {
                id,
                score: point.score,
                metadata: extract_payload_to_hashmap(point.payload),
            }
        })
        .collect();

    Ok(results)
}

fn extract_payload_to_hashmap(
    payload: HashMap<String, qdrant_client::qdrant::Value>,
) -> HashMap<String, serde_json::Value> {
    let mut result = HashMap::new();

    for (key, value) in payload {
        let json_value = match value.kind {
            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                serde_json::Value::String(s)
            }
            Some(qdrant_client::qdrant::value::Kind::IntegerValue(i)) => {
                serde_json::Value::Number(serde_json::Number::from(i))
            }
            Some(qdrant_client::qdrant::value::Kind::DoubleValue(d)) => {
                if let Some(num) = serde_json::Number::from_f64(d) {
                    serde_json::Value::Number(num)
                } else {
                    serde_json::Value::Null
                }
            }
            Some(qdrant_client::qdrant::value::Kind::BoolValue(b)) => serde_json::Value::Bool(b),
            Some(qdrant_client::qdrant::value::Kind::ListValue(list)) => {
                let array: Vec<serde_json::Value> = list
                    .values
                    .into_iter()
                    .map(|v| match v.kind {
                        Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                            serde_json::Value::String(s)
                        }
                        _ => serde_json::Value::Null,
                    })
                    .collect();
                serde_json::Value::Array(array)
            }
            _ => serde_json::Value::Null,
        };
        result.insert(key, json_value);
    }

    result
}

#[derive(Debug)]
struct SearchResult {
    id: String,
    score: f32,
    metadata: HashMap<String, serde_json::Value>,
}

fn print_results(query: &str, results: Vec<SearchResult>) {
    println!("\n🔍 Search Query: \"{}\"", query);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if results.is_empty() {
        println!("No results found.");
        return;
    }

    for (i, result) in results.iter().enumerate() {
        let name = result
            .metadata
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let description = result
            .metadata
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("No description");

        let category = result
            .metadata
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let usage = result
            .metadata
            .get("usage")
            .and_then(|v| v.as_str())
            .unwrap_or("No usage info");

        println!("\n{}. {} (Score: {:.3})", i + 1, name, result.score);
        println!("   Category: {}", category);
        println!("   Description: {}", description);
        println!("   Usage: {}", usage);

        if let Some(examples) = result.metadata.get("examples") {
            if let Some(examples_array) = examples.as_array() {
                let examples_str: Vec<String> = examples_array
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                if !examples_str.is_empty() {
                    println!("   Examples: {}", examples_str.join(", "));
                }
            }
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = Args::parse();

    // Get configuration from environment variables
    let gemini_api_key =
        env::var("GEMINI_API_KEY").context("GEMINI_API_KEY environment variable is required")?;

    let qdrant_url = env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let qdrant_api_key = env::var("QDRANT_API_KEY").ok();

    println!("Generating query embedding...");
    let gemini_client = GeminiClient::new(gemini_api_key);
    let query_embedding = gemini_client
        .generate_embedding(&args.query, "RETRIEVAL_QUERY")
        .await?;

    println!("Connecting to Qdrant at {}...", qdrant_url);
    let qdrant_client = create_qdrant_client(&qdrant_url, qdrant_api_key.as_deref()).await?;

    println!("Searching collection '{}'...", args.collection);
    let results = search_components(
        &qdrant_client,
        &args.collection,
        query_embedding,
        args.top_k,
    )
    .await?;

    print_results(&args.query, results);

    Ok(())
}
