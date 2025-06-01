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

    let api_key = env::var("GEMINI_API_KEY")
        .map_err(|_| "GEMINI_API_KEY environment variable not set".to_string())?;

    let client = reqwest::Client::new();


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

      ],
      "events": {{
        "onClick": {{
          "action": "action_name",
          "target": "component_id (required for some actions)",
          "payload": {{  }}
        }}
      }}
    }}
  ]
}}

Available element types: card, button, input, label, form, nav, header, main, footer, div
Available button variants: primary, secondary, danger, outline
Available input types: text, email, password, number

AVAILABLE ACTIONS (use these for interactive functionality):

VISIBILITY ACTIONS:
- "show": Makes a component visible (requires target)
  Example: {{"action": "show", "target": "modal-dialog"}}

- "hide": Hides a component (requires target)
  Example: {{"action": "hide", "target": "notification-banner"}}

- "toggle": Toggles component visibility (requires target)
  Example: {{"action": "toggle", "target": "nav-menu"}}

CONTENT ACTIONS:
- "update": Updates component content/properties (requires target and payload)
  Example: {{"action": "update", "target": "submit-button", "payload": {{"content": "Loading..."}}}}

LIFECYCLE ACTIONS:
- "create": Creates new components dynamically (requires payload, target optional)
  Example: {{"action": "create", "target": "todo-list", "payload": {{"type": "todo-item", "text": "{{new-todo-input.value}}", "id": "todo-item-{{timestamp}}", "completed": false}}}}

- "destroy": Removes components permanently (requires target) - can also use "delete"
  Example: {{"action": "destroy", "target": "todo-item-1"}}

STATE ACTIONS:
- "setState": Updates global or component state (requires payload, target optional)
  Example: {{"action": "setState", "payload": {{"user": {{"authenticated": true}}}}}}

DATA ACTIONS:
- "submit": Submits form data to endpoint (requires payload, target optional)
  Example: {{"action": "submit", "payload": {{"fields": ["username", "password"], "endpoint": "/api/login", "onSuccess": {{"action": "navigate", "payload": {{"route": "/dashboard"}}}}}}}}

- "validate": Validates form fields (requires payload, target optional)
  Example: {{"action": "validate", "payload": {{"fields": {{"email": {{"required": true, "pattern": "email"}}}}, "onValid": {{"action": "submit", "payload": {{"endpoint": "/api/register"}}}}}}}}

- "collect": Collects form data locally (requires payload, target optional)
  Example: {{"action": "collect", "payload": {{"fields": ["firstName", "lastName"], "collectionId": "user-data"}}}}

NAVIGATION ACTIONS:
- "navigate": Navigates to different page/route (requires payload)
  Example: {{"action": "navigate", "payload": {{"route": "/dashboard", "replace": false}}}}

ANIMATION ACTIONS:
- "animate": Triggers component animations (requires target, payload optional)
  Example: {{"action": "animate", "target": "submit-button", "payload": {{"type": "pulse", "duration": 300}}}}

IMPORTANT USAGE GUIDELINES:
1. For TODO/list apps: Use "create" to add items, "destroy"/"delete" to remove items
2. For forms: Use "submit" for form submission, "validate" for validation, "collect" for data gathering
3. For dynamic content: Use "{{element-id.value}}" to reference input values in payloads
4. For modals/dialogs: Use "show" to open, "hide" to close
5. For navigation: Use "navigate" after successful operations
6. Always provide proper target IDs and meaningful payloads

Use Tailwind CSS classes for styling. Create interactive, modern, and visually appealing UIs.
Make sure all IDs are unique and descriptive. Include proper actions for all interactive elements.

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
        "https:
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


    let json_str = json_str.trim();


    let cleaned_json = if let Some(start) = json_str.find('{') {
        if let Some(end) = json_str.rfind('}') {
            &json_str[start..=end]
        } else {

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
