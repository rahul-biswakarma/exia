use crate::models::{Solution, SolutionStatus, TestCase, TestResult};
use anyhow::Result;
use std::fs;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::time::timeout;

pub struct RustCompiler {
    temp_dir: TempDir,
}

impl RustCompiler {
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        Ok(Self { temp_dir })
    }

    pub async fn compile_and_test(&self, code: &str, test_cases: &[TestCase]) -> Result<Solution> {
        let start_time = Instant::now();

        // Create a temporary Rust project
        let project_path = self.temp_dir.path().join("solution");
        fs::create_dir_all(&project_path)?;

        // Create Cargo.toml
        let cargo_toml = r#"[package]
name = "solution"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "solution"
path = "src/main.rs"
"#;
        fs::write(project_path.join("Cargo.toml"), cargo_toml)?;

        // Create src directory
        let src_path = project_path.join("src");
        fs::create_dir_all(&src_path)?;

        // Write the solution code
        let main_rs_content = self.wrap_solution_code(code);
        fs::write(src_path.join("main.rs"), main_rs_content)?;

        // Compile the code
        let compile_result = self.compile_code(&project_path).await?;
        if !compile_result.success {
            return Ok(Solution {
                id: uuid::Uuid::new_v4(),
                question_id: uuid::Uuid::new_v4(), // This will be set by the caller
                code: code.to_string(),
                language: "rust".to_string(),
                status: SolutionStatus::CompilationError,
                execution_time: Some(start_time.elapsed().as_millis() as u64),
                memory_usage: None,
                test_results: vec![],
                submitted_at: chrono::Utc::now(),
                feedback: Some(compile_result.error_message),
            });
        }

        // Run test cases
        let test_results = self.run_test_cases(&project_path, test_cases).await?;

        // Determine overall status
        let status = if test_results.iter().all(|tr| tr.passed) {
            SolutionStatus::Accepted
        } else {
            SolutionStatus::WrongAnswer
        };

        Ok(Solution {
            id: uuid::Uuid::new_v4(),
            question_id: uuid::Uuid::new_v4(), // This will be set by the caller
            code: code.to_string(),
            language: "rust".to_string(),
            status,
            execution_time: Some(start_time.elapsed().as_millis() as u64),
            memory_usage: None,
            test_results,
            submitted_at: chrono::Utc::now(),
            feedback: None,
        })
    }

    async fn compile_code(&self, project_path: &std::path::Path) -> Result<CompileResult> {
        let output = timeout(
            Duration::from_secs(30),
            tokio::task::spawn_blocking({
                let project_path = project_path.to_owned();
                move || {
                    Command::new("cargo")
                        .args(&["build", "--release"])
                        .current_dir(&project_path)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                }
            }),
        )
        .await???;

        let success = output.status.success();
        let error_message = if !success {
            String::from_utf8_lossy(&output.stderr).to_string()
        } else {
            String::new()
        };

        Ok(CompileResult {
            success,
            error_message,
        })
    }

    async fn run_test_cases(
        &self,
        project_path: &std::path::Path,
        test_cases: &[TestCase],
    ) -> Result<Vec<TestResult>> {
        let mut results = Vec::new();

        for (index, test_case) in test_cases.iter().enumerate() {
            let result = self.run_single_test(project_path, test_case, index).await?;
            results.push(result);
        }

        Ok(results)
    }

    async fn run_single_test(
        &self,
        project_path: &std::path::Path,
        test_case: &TestCase,
        index: usize,
    ) -> Result<TestResult> {
        let binary_path = project_path.join("target/release/solution");

        let output_result = timeout(
            Duration::from_secs(5), // 5 second timeout per test case
            tokio::task::spawn_blocking({
                let binary_path = binary_path.clone();
                let input = test_case.input.clone();
                move || {
                    let mut child = Command::new(&binary_path)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()?;

                    if let Some(stdin) = child.stdin.as_mut() {
                        use std::io::Write;
                        stdin.write_all(input.as_bytes())?;
                    }

                    child.wait_with_output()
                }
            }),
        )
        .await;

        match output_result {
            Ok(Ok(Ok(output))) => {
                if !output.status.success() {
                    return Ok(TestResult {
                        test_case_index: index,
                        passed: false,
                        actual_output: String::from_utf8_lossy(&output.stderr).to_string(),
                        error_message: Some("Runtime error".to_string()),
                    });
                }

                let actual_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let expected_output = test_case.expected_output.trim();
                let passed = actual_output == expected_output;

                Ok(TestResult {
                    test_case_index: index,
                    passed,
                    actual_output: actual_output.clone(),
                    error_message: if !passed {
                        Some(format!(
                            "Expected: {}, Got: {}",
                            expected_output, actual_output
                        ))
                    } else {
                        None
                    },
                })
            }
            Ok(Ok(Err(e))) => Ok(TestResult {
                test_case_index: index,
                passed: false,
                actual_output: String::new(),
                error_message: Some(format!("Execution error: {}", e)),
            }),
            Ok(Err(_)) | Err(_) => Ok(TestResult {
                test_case_index: index,
                passed: false,
                actual_output: String::new(),
                error_message: Some("Time limit exceeded".to_string()),
            }),
        }
    }

    fn wrap_solution_code(&self, code: &str) -> String {
        // Check if the code already has a main function
        if code.contains("fn main") {
            return code.to_string();
        }

        // Check if the code already defines a solution function
        if code.contains("fn solution") {
            // If solution function exists, just add main function
            format!(
                r#"use std::io::{{self, Read}};

{}

fn main() {{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let result = solution(&input.trim());
    println!("{{}}", result);
}}
"#,
                code
            )
        } else {
            // If no solution function, provide a default one
            format!(
                r#"use std::io::{{self, Read}};

{}

fn main() {{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let result = solution(&input.trim());
    println!("{{}}", result);
}}

// Default solution function if not provided
#[allow(dead_code)]
fn solution(input: &str) -> String {{
    // This is a placeholder - the actual solution should be implemented above
    input.to_string()
}}
"#,
                code
            )
        }
    }
}

struct CompileResult {
    success: bool,
    error_message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_compilation() {
        let compiler = RustCompiler::new().unwrap();
        let code = r#"
fn solution(input: &str) -> String {
    let n: i32 = input.parse().unwrap();
    (n * 2).to_string()
}
"#;

        let test_cases = vec![TestCase {
            input: "5".to_string(),
            expected_output: "10".to_string(),
            description: "Double the input".to_string(),
        }];

        let result = compiler.compile_and_test(code, &test_cases).await.unwrap();
        assert_eq!(result.status, SolutionStatus::Accepted);
    }

    #[tokio::test]
    async fn test_compilation_error() {
        let compiler = RustCompiler::new().unwrap();
        let code = r#"
fn solution(input: &str) -> String {
    let n: i32 = input.parse().unwrap();
    // Missing return statement and syntax error
    n * 2
}
"#;

        let test_cases = vec![];
        let result = compiler.compile_and_test(code, &test_cases).await.unwrap();
        assert_eq!(result.status, SolutionStatus::CompilationError);
    }
}
