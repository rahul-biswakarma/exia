use super::{gemini_api, vector_search};

pub async fn generate_ui_with_llm(prompt: &str) -> Result<serde_json::Value, String> {
    // Try vector search + enhanced LLM generation first
    match vector_search::create_enhanced_ui_with_vector_search(prompt).await {
        Ok(schema) => Ok(schema),
        Err(vector_error) => {
            println!(
                "Vector search failed: {}, falling back to pure LLM",
                vector_error
            );

            // Fallback to pure Gemini API
            gemini_api::generate_ui_schema(prompt).await
        }
    }
}
