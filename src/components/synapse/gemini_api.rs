use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "topK")]
    top_k: i32,
    #[serde(rename = "topP")]
    top_p: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: i32,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

pub async fn generate_ui_schema(prompt: &str) -> Result<Value, String> {
    // Get API key from environment
    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

    let client = reqwest::Client::new();

    // Create a detailed prompt for UI generation
    let system_prompt = format!(
        r#"You are a UI generator that creates JSON schemas for web interfaces.
Based on the user's description, generate a detailed UI schema that follows this exact format:

{{
  "ui_elements": [
    {{
      "id": "unique-element-id",
      "type": "element_type",
      "content": "text content or empty string",
      "properties": {{
        "className": "tailwind css classes",
        "variant": "primary|secondary|danger|outline (for buttons)",
        "type": "input type (for inputs)",
        "placeholder": "placeholder text (for inputs)"
      }},
      "children": [
        // nested elements following same structure
      ],
      "events": {{
        "onClick": {{
          "action": "setState|show|hide|toggle|create|navigate",
          "target": "component_id",
          "payload": {{ /* action-specific data */ }}
        }}
      }}
    }}
  ]
}}

Available element types: card, button, input, label, form, nav, header, main, footer, div
Available button variants: primary, secondary, danger, outline
Available input types: text, email, password, number

Use Tailwind CSS classes for styling. Create interactive, modern, and visually appealing UIs.
Make sure all IDs are unique and descriptive.

User request: {}"#,
        prompt
    );

    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: system_prompt,
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.7,
            top_k: 40,
            top_p: 0.95,
            max_output_tokens: 2048,
        },
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key
    );

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API request failed: {}", error_text));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let generated_text = gemini_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| &p.text)
        .ok_or("No response content found")?;

    // Extract JSON from the response (it might be wrapped in markdown code blocks)
    let json_str = if generated_text.contains("```json") {
        let start = generated_text.find("```json").unwrap() + 7;
        let end = generated_text[start..]
            .find("```")
            .unwrap_or(generated_text.len() - start);
        &generated_text[start..start + end]
    } else if generated_text.contains("```") {
        let start = generated_text.find("```").unwrap() + 3;
        let end = generated_text[start..]
            .find("```")
            .unwrap_or(generated_text.len() - start);
        &generated_text[start..start + end]
    } else {
        generated_text
    };

    // Parse the JSON with better error handling
    let json_str = json_str.trim();

    // Try to find valid JSON if the response is truncated or malformed
    let cleaned_json = if let Some(start) = json_str.find('{') {
        if let Some(end) = json_str.rfind('}') {
            &json_str[start..=end]
        } else {
            // If no closing brace, try to find the last complete object
            let mut brace_count = 0;
            let mut last_valid_end = start;

            for (i, c) in json_str[start..].char_indices() {
                match c {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            last_valid_end = start + i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            &json_str[start..=last_valid_end]
        }
    } else {
        json_str
    };

    let ui_schema: Value = serde_json::from_str(cleaned_json).map_err(|e| {
        format!(
            "Failed to parse generated JSON: {} | Raw response: {}",
            e, generated_text
        )
    })?;

    Ok(ui_schema)
}
