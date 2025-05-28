use crate::configs::llm::clients::LLMClients;
use genai::Client;

pub fn get_gemini_client() -> LLMClients {
    // Create a genai client for Gemini
    let client = Client::default();

    // Return the appropriate LLMClients variant
    // Note: You'll need to update this based on your LLMClients enum structure
    LLMClients::Gemini
}
