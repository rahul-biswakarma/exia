use anyhow::{Context, Result};
use qdrant_client::qdrant::{ScoredPoint, SearchPoints};
use qdrant_client::Qdrant;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

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

#[derive(Debug, Clone)]
pub struct ComponentMatch {
    pub score: f32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub usage: String,
    pub examples: Vec<String>,
}

pub struct VectorSearchClient {
    gemini_client: Client,
    qdrant_client: Option<Qdrant>,
    gemini_api_key: String,
}

impl VectorSearchClient {
    pub async fn new() -> Result<Self, String> {
        let gemini_api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

        // Try to connect to Qdrant, but don't fail if it's not available
        let qdrant_client = Self::try_connect_qdrant().await.ok();

        Ok(Self {
            gemini_client: Client::new(),
            qdrant_client,
            gemini_api_key,
        })
    }

    async fn try_connect_qdrant() -> Result<Qdrant> {
        let qdrant_url =
            env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
        let qdrant_api_key = env::var("QDRANT_API_KEY").ok();

        // Configure with more robust settings
        let mut client_builder = Qdrant::from_url(&qdrant_url);

        if let Some(key) = qdrant_api_key {
            client_builder = client_builder.api_key(key);
        }

        // Add timeout and connection settings to avoid HTTP/2 frame size errors
        client_builder = client_builder
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10));

        client_builder
            .build()
            .context("Failed to connect to Qdrant")
    }

    pub async fn search_similar_components(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<ComponentMatch>, String> {
        let qdrant_client = match &self.qdrant_client {
            Some(client) => client,
            None => return Ok(Vec::new()), // No vector DB available, return empty results
        };

        // Generate embedding for the query
        let query_embedding = self.generate_embedding(query).await?;

        // Search in Qdrant with better error handling
        let search_points = SearchPoints {
            collection_name: "components".to_string(),
            vector: query_embedding,
            limit: limit as u64,
            with_payload: Some(true.into()),
            ..Default::default()
        };

        let search_result = qdrant_client
            .search_points(search_points)
            .await
            .map_err(|e| {
                let error_msg = format!("Vector search failed: {}", e);
                // Check if it's a collection not found error
                if error_msg.contains("not found") || error_msg.contains("Not found") {
                    format!("Collection 'components' not found. Please run the vector database setup first: ./vector_db/upload.sh")
                } else {
                    error_msg
                }
            })?;

        // Convert results to ComponentMatch
        let matches: Vec<ComponentMatch> = search_result
            .result
            .into_iter()
            .map(|point: ScoredPoint| {
                let payload = Self::extract_payload_to_hashmap(point.payload);

                ComponentMatch {
                    score: point.score,
                    name: payload
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string(),
                    description: payload
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    category: payload
                        .get("category")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    usage: payload
                        .get("usage")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    examples: payload
                        .get("examples")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|item| item.as_str())
                                .map(|s| s.to_string())
                                .collect()
                        })
                        .unwrap_or_default(),
                }
            })
            .collect();

        Ok(matches)
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, String> {
        let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-exp-03-07:embedContent";

        let request = GeminiEmbedRequest {
            model: "models/gemini-embedding-exp-03-07".to_string(),
            content: GeminiContent {
                parts: vec![GeminiPart {
                    text: text.to_string(),
                }],
            },
            task_type: "RETRIEVAL_QUERY".to_string(),
        };

        let response = self
            .gemini_client
            .post(url)
            .query(&[("key", &self.gemini_api_key)])
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to generate embedding: {}", e))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("Embedding API request failed: {}", error_text));
        }

        let embed_response: GeminiEmbedResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse embedding response: {}", e))?;

        // Convert f64 to f32 for Qdrant
        let embedding: Vec<f32> = embed_response
            .embedding
            .values
            .into_iter()
            .map(|x| x as f32)
            .collect();

        Ok(embedding)
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
                Some(qdrant_client::qdrant::value::Kind::BoolValue(b)) => {
                    serde_json::Value::Bool(b)
                }
                Some(qdrant_client::qdrant::value::Kind::ListValue(list)) => {
                    let array: Vec<serde_json::Value> = list
                        .values
                        .into_iter()
                        .filter_map(|v| match v.kind {
                            Some(qdrant_client::qdrant::value::Kind::StringValue(s)) => {
                                Some(serde_json::Value::String(s))
                            }
                            _ => None,
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

    pub fn is_vector_db_available(&self) -> bool {
        self.qdrant_client.is_some()
    }
}

pub async fn create_enhanced_ui_with_vector_search(prompt: &str) -> Result<Value, String> {
    let vector_client = VectorSearchClient::new().await?;

    // First, try to find similar components in the vector database
    if vector_client.is_vector_db_available() {
        let similar_components = vector_client.search_similar_components(prompt, 3).await?;

        // If we found high-scoring matches (>0.8), use them to inform the LLM
        let high_quality_matches: Vec<&ComponentMatch> = similar_components
            .iter()
            .filter(|m| m.score > 0.8)
            .collect();

        if !high_quality_matches.is_empty() {
            println!(
                "🎯 Found {} similar components in vector DB",
                high_quality_matches.len()
            );
            return create_ui_from_vector_matches(prompt, &high_quality_matches).await;
        } else if !similar_components.is_empty() {
            println!(
                "📋 Found {} related components, enhancing LLM prompt",
                similar_components.len()
            );
            return create_enhanced_llm_ui(prompt, &similar_components).await;
        }
    }

    // Fallback to pure LLM generation
    println!("🤖 No vector matches found, using pure LLM generation");
    super::gemini_api::generate_ui_schema(prompt).await
}

async fn create_ui_from_vector_matches(
    prompt: &str,
    matches: &[&ComponentMatch],
) -> Result<Value, String> {
    // Create enhanced prompt using the matched components
    let context = matches
        .iter()
        .map(|m| format!("- {}: {} ({})", m.name, m.description, m.usage))
        .collect::<Vec<_>>()
        .join("\n");

    let enhanced_prompt = format!(
        "Based on these existing components:\n{}\n\nUser request: {}\n\nGenerate a UI that leverages these existing components where appropriate.",
        context, prompt
    );

    super::gemini_api::generate_ui_schema(&enhanced_prompt).await
}

async fn create_enhanced_llm_ui(
    prompt: &str,
    similar_components: &[ComponentMatch],
) -> Result<Value, String> {
    let context = similar_components
        .iter()
        .take(3) // Limit to top 3 to avoid token limits
        .map(|m| format!("- {}: {}", m.name, m.description))
        .collect::<Vec<_>>()
        .join("\n");

    let enhanced_prompt = format!(
        "Consider these related components for inspiration:\n{}\n\nUser request: {}\n\nCreate a new UI that may incorporate similar patterns or concepts.",
        context, prompt
    );

    super::gemini_api::generate_ui_schema(&enhanced_prompt).await
}
