use crate::models::{Difficulty, LearningProgress, Question, Solution, TestCase, Topic};
use anyhow::{anyhow, Result};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
// use serde_json::json;
use std::env;
use uuid::Uuid;

#[derive(Clone)]
pub struct GeminiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
    content: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedQuestion {
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub topic: String,
    pub hints: Vec<String>,
    pub test_cases: Vec<GeneratedTestCase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedTestCase {
    pub input: String,
    pub expected_output: String,
    pub description: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GEMINI_API_KEY")
            .map_err(|_| anyhow!("GEMINI_API_KEY environment variable not set"))?;

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url:
                "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash-preview-05-20:generateContent"
                    .to_string(),
        })
    }

    pub async fn generate_question(&self, progress: &LearningProgress) -> Result<Question> {
        let prompt = self.create_question_prompt(progress);
        let response = self.call_gemini(&prompt).await?;
        self.parse_question_response(&response)
    }

    pub async fn provide_feedback(
        &self,
        solution: &Solution,
        question: &Question,
    ) -> Result<String> {
        let prompt = self.create_feedback_prompt(solution, question);
        let response = self.call_gemini(&prompt).await?;
        Ok(response)
    }

    pub async fn generate_hint(&self, question: &Question, current_code: &str) -> Result<String> {
        let prompt = format!(
            "Given this DSA problem:\n\nTitle: {}\nDescription: {}\n\nAnd the current code attempt:\n```rust\n{}\n```\n\nProvide a helpful hint to guide the solution without giving away the complete answer. Focus on the approach or a specific technique that might help.",
            question.title, question.description, current_code
        );

        let response = self.call_gemini(&prompt).await?;
        Ok(response)
    }

    async fn call_gemini(&self, prompt: &str) -> Result<String> {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generation_config: GenerationConfig {
                temperature: 0.7,
                top_k: 40,
                top_p: 0.95,
                max_output_tokens: 2048,
            },
        };

        let url = format!("{}?key={}", self.base_url, self.api_key);

        let response = self.client.post(&url).json(&request).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Gemini API error: {}", error_text));
        }

        let gemini_response: GeminiResponse = response.json().await?;

        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                return Ok(part.text.clone());
            }
        }

        Err(anyhow!("No response from Gemini API"))
    }

    fn create_question_prompt(&self, progress: &LearningProgress) -> String {
        let weak_topics: Vec<String> = progress.weaknesses.iter().map(|t| t.to_string()).collect();
        let strong_topics: Vec<String> = progress.strengths.iter().map(|t| t.to_string()).collect();

        let difficulty_suggestion = if progress.questions_solved < 10 {
            "Easy"
        } else if progress.questions_solved < 50 {
            "Medium"
        } else {
            "Hard"
        };

        format!(
            r#"Generate a Data Structures and Algorithms question for a Rust programmer with the following learning profile:

Learning Progress:
- Total questions attempted: {}
- Questions solved: {}
- Current streak: {}
- Weak areas: {}
- Strong areas: {}

Please generate a {} level question focusing on areas where the user needs improvement.

Return the response in the following JSON format:
{{
    "title": "Question Title",
    "description": "Detailed problem description with constraints and examples",
    "difficulty": "Easy|Medium|Hard",
    "topic": "Arrays|LinkedLists|Stacks|Queues|Trees|Graphs|DynamicProgramming|Sorting|Searching|Hashing|Strings|Recursion|Backtracking|Greedy|Math",
    "hints": ["hint1", "hint2", "hint3"],
    "test_cases": [
        {{
            "input": "sample input",
            "expected_output": "expected output",
            "description": "test case description"
        }}
    ]
}}

Make sure the question is:
1. Appropriate for the user's skill level
2. Focused on improving weak areas
3. Includes clear examples and constraints
4. Has comprehensive test cases
5. Is solvable in Rust programming language"#,
            progress.total_questions_attempted,
            progress.questions_solved,
            progress.streak,
            if weak_topics.is_empty() {
                "None identified yet".to_string()
            } else {
                weak_topics.join(", ")
            },
            if strong_topics.is_empty() {
                "None identified yet".to_string()
            } else {
                strong_topics.join(", ")
            },
            difficulty_suggestion
        )
    }

    fn create_feedback_prompt(&self, solution: &Solution, question: &Question) -> String {
        let test_results_summary: Vec<String> = solution
            .test_results
            .iter()
            .map(|tr| {
                format!(
                    "Test {}: {} (Output: {})",
                    tr.test_case_index + 1,
                    if tr.passed { "PASSED" } else { "FAILED" },
                    tr.actual_output
                )
            })
            .collect();

        format!(
            r#"Analyze this Rust solution for a DSA problem and provide constructive feedback:

Problem: {}
Description: {}

Solution Code:
```rust
{}
```

Solution Status: {}
Test Results:
{}

Please provide:
1. Code quality assessment
2. Algorithm efficiency analysis (time/space complexity)
3. Suggestions for improvement
4. If the solution failed, explain what went wrong
5. Alternative approaches if applicable
6. Rust-specific best practices

Keep the feedback constructive and educational."#,
            question.title,
            question.description,
            solution.code,
            solution.status,
            test_results_summary.join("\n")
        )
    }

    fn parse_question_response(&self, response: &str) -> Result<Question> {
        // Try to extract JSON from the response
        let json_start = response
            .find('{')
            .ok_or_else(|| anyhow!("No JSON found in response"))?;
        let json_end = response
            .rfind('}')
            .ok_or_else(|| anyhow!("No JSON found in response"))?;
        let json_str = &response[json_start..=json_end];

        let generated: GeneratedQuestion = serde_json::from_str(json_str)
            .map_err(|e| anyhow!("Failed to parse question JSON: {}", e))?;

        let difficulty = match generated.difficulty.to_lowercase().as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            _ => Difficulty::Medium,
        };

        let topic = match generated.topic.to_lowercase().as_str() {
            "arrays" => Topic::Arrays,
            "linkedlists" | "linked lists" => Topic::LinkedLists,
            "stacks" => Topic::Stacks,
            "queues" => Topic::Queues,
            "trees" => Topic::Trees,
            "graphs" => Topic::Graphs,
            "dynamicprogramming" | "dynamic programming" => Topic::DynamicProgramming,
            "sorting" => Topic::Sorting,
            "searching" => Topic::Searching,
            "hashing" => Topic::Hashing,
            "strings" => Topic::Strings,
            "recursion" => Topic::Recursion,
            "backtracking" => Topic::Backtracking,
            "greedy" => Topic::Greedy,
            "math" => Topic::Math,
            _ => Topic::Arrays,
        };

        let test_cases: Vec<TestCase> = generated
            .test_cases
            .into_iter()
            .map(|tc| TestCase {
                input: tc.input,
                expected_output: tc.expected_output,
                description: tc.description,
            })
            .collect();

        Ok(Question {
            id: Uuid::new_v4(),
            title: generated.title,
            description: generated.description,
            difficulty,
            topic,
            hints: generated.hints,
            test_cases,
            created_at: Utc::now(),
        })
    }
}
