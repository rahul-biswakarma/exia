use crate::configs::llm::clients::LLMClients;
use genai::Client;

pub fn get_gemini_client() -> LLMClients {
    // Create a genai client for Gemini
    let _client = Client::default();

    // Return the appropriate LLMClients variant
    // Note: You'll need to update this based on your LLMClients enum structure
    LLMClients::Gemini
}

pub async fn generate_ui_with_llm(prompt: &str) -> Result<String, String> {
    // Try to use the genai client for Gemini
    match std::env::var("GEMINI_API_KEY") {
        Ok(api_key) if !api_key.is_empty() => {
            match make_gemini_call(prompt, &api_key).await {
                Ok(response) => Ok(response),
                Err(e) => {
                    eprintln!("Gemini API call failed: {}", e);
                    // Fallback to mock response for development
                    Ok(get_mock_ui_response(prompt))
                }
            }
        }
        _ => {
            // No API key available, use mock response for development
            eprintln!("No GEMINI_API_KEY found, using mock response");
            Ok(get_mock_ui_response(prompt))
        }
    }
}

async fn make_gemini_call(prompt: &str, _api_key: &str) -> Result<String, String> {
    // For now, we'll use a simple HTTP client approach
    // In a real implementation, you'd use the genai crate properly

    // This is a placeholder implementation
    // You would need to implement the actual Gemini API call here
    Err("Gemini API not implemented yet".to_string())
}

fn get_mock_ui_response(prompt: &str) -> String {
    // Generate different mock responses based on the prompt content
    let prompt_lower = prompt.to_lowercase();

    if prompt_lower.contains("quiz")
        || prompt_lower.contains("knowledge")
        || prompt_lower.contains("gk")
    {
        r#"{
  "uiElements": [
    {
      "layoutType": "grid",
      "rows": 6,
      "cols": 2,
      "gap": 15,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "🧠 General Knowledge Quiz"
            }
          },
          "row": 1,
          "col": 1,
          "colSpan": 2
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Question 1: What is the capital of France?"
            }
          },
          "row": 2,
          "col": 1,
          "colSpan": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "paris",
              "id": "q1_paris"
            }
          },
          "row": 3,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Paris"
            }
          },
          "row": 3,
          "col": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "london",
              "id": "q1_london"
            }
          },
          "row": 4,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "London"
            }
          },
          "row": 4,
          "col": 2
        },
        {
          "atom": {
            "type": "progress",
            "properties": {
              "value": 1,
              "max": 10
            }
          },
          "row": 5,
          "col": 1,
          "colSpan": 2
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Progress: 1/10 questions"
            }
          },
          "row": 6,
          "col": 1,
          "colSpan": 2
        }
      ]
    }
  ]
}"#
        .to_string()
    } else if prompt_lower.contains("calculator") {
        r#"{
  "uiElements": [
    {
      "layoutType": "grid",
      "rows": 6,
      "cols": 4,
      "gap": 10,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "🧮 Calculator"
            }
          },
          "row": 1,
          "col": 1,
          "colSpan": 4
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "0"
            }
          },
          "row": 2,
          "col": 1,
          "colSpan": 4
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "7"
            }
          },
          "row": 3,
          "col": 1
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "8"
            }
          },
          "row": 3,
          "col": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "9"
            }
          },
          "row": 3,
          "col": 3
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "/"
            }
          },
          "row": 3,
          "col": 4
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "4"
            }
          },
          "row": 4,
          "col": 1
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "5"
            }
          },
          "row": 4,
          "col": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "6"
            }
          },
          "row": 4,
          "col": 3
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "*"
            }
          },
          "row": 4,
          "col": 4
        }
      ]
    }
  ]
}"#
        .to_string()
    } else if prompt_lower.contains("snake") || prompt_lower.contains("game") {
        r#"{
  "uiElements": [
    {
      "layoutType": "grid",
      "rows": 4,
      "cols": 3,
      "gap": 15,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "🐍 Snake Game"
            }
          },
          "row": 1,
          "col": 1,
          "colSpan": 3
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Score: 0"
            }
          },
          "row": 2,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "High Score: 150"
            }
          },
          "row": 2,
          "col": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "start"
            }
          },
          "row": 2,
          "col": 3
        },
        {
          "atom": {
            "type": "separator",
            "properties": {}
          },
          "row": 3,
          "col": 1,
          "colSpan": 3
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Use arrow keys to control the snake. Eat food to grow!"
            }
          },
          "row": 4,
          "col": 1,
          "colSpan": 3
        }
      ]
    }
  ]
}"#
        .to_string()
    } else if prompt_lower.contains("todo") || prompt_lower.contains("task") {
        r#"{
  "uiElements": [
    {
      "layoutType": "grid",
      "rows": 5,
      "cols": 3,
      "gap": 10,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "📝 Todo List"
            }
          },
          "row": 1,
          "col": 1,
          "colSpan": 3
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": true,
              "value": "task1"
            }
          },
          "row": 2,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Complete project setup"
            }
          },
          "row": 2,
          "col": 2
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "High"
            }
          },
          "row": 2,
          "col": 3
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "task2"
            }
          },
          "row": 3,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Review code changes"
            }
          },
          "row": 3,
          "col": 2
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Medium"
            }
          },
          "row": 3,
          "col": 3
        },
        {
          "atom": {
            "type": "progress",
            "properties": {
              "value": 50,
              "max": 100
            }
          },
          "row": 4,
          "col": 1,
          "colSpan": 3
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Overall Progress: 50%"
            }
          },
          "row": 5,
          "col": 1,
          "colSpan": 3
        }
      ]
    }
  ]
}"#
        .to_string()
    } else {
        // Default response for any other request
        r#"{
  "uiElements": [
    {
      "layoutType": "grid",
      "rows": 4,
      "cols": 2,
      "gap": 15,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "✨ Custom UI Generated"
            }
          },
          "row": 1,
          "col": 1,
          "colSpan": 2
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Your request has been processed!"
            }
          },
          "row": 2,
          "col": 1,
          "colSpan": 2
        },
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "feature1"
            }
          },
          "row": 3,
          "col": 1
        },
        {
          "atom": {
            "type": "label",
            "properties": {
              "text": "Enable this feature"
            }
          },
          "row": 3,
          "col": 2
        },
        {
          "atom": {
            "type": "slider",
            "properties": {
              "value": 50,
              "min": 0,
              "max": 100,
              "step": 1
            }
          },
          "row": 4,
          "col": 1,
          "colSpan": 2
        }
      ]
    }
  ]
}"#
        .to_string()
    }
}
