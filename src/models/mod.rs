use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub topic: Topic,
    pub hints: Vec<String>,
    pub test_cases: Vec<TestCase>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected_output: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Topic {
    Arrays,
    LinkedLists,
    Stacks,
    Queues,
    Trees,
    Graphs,
    DynamicProgramming,
    Sorting,
    Searching,
    Hashing,
    Strings,
    Recursion,
    Backtracking,
    Greedy,
    Math,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub id: Uuid,
    pub question_id: Uuid,
    pub code: String,
    pub language: String,
    pub status: SolutionStatus,
    pub execution_time: Option<u64>,
    pub memory_usage: Option<u64>,
    pub test_results: Vec<TestResult>,
    pub submitted_at: DateTime<Utc>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SolutionStatus {
    Pending,
    Accepted,
    WrongAnswer,
    CompilationError,
    RuntimeError,
    TimeLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_case_index: usize,
    pub passed: bool,
    pub actual_output: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProgress {
    pub user_id: String,
    pub total_questions_attempted: u32,
    pub questions_solved: u32,
    pub topic_progress: HashMap<Topic, TopicProgress>,
    pub difficulty_progress: HashMap<Difficulty, DifficultyProgress>,
    pub streak: u32,
    pub last_activity: DateTime<Utc>,
    pub strengths: Vec<Topic>,
    pub weaknesses: Vec<Topic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicProgress {
    pub attempted: u32,
    pub solved: u32,
    pub average_attempts: f32,
    pub last_practiced: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyProgress {
    pub attempted: u32,
    pub solved: u32,
    pub success_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub questions_attempted: Vec<Uuid>,
    pub solutions_submitted: Vec<Uuid>,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Topic::Arrays => write!(f, "Arrays"),
            Topic::LinkedLists => write!(f, "Linked Lists"),
            Topic::Stacks => write!(f, "Stacks"),
            Topic::Queues => write!(f, "Queues"),
            Topic::Trees => write!(f, "Trees"),
            Topic::Graphs => write!(f, "Graphs"),
            Topic::DynamicProgramming => write!(f, "Dynamic Programming"),
            Topic::Sorting => write!(f, "Sorting"),
            Topic::Searching => write!(f, "Searching"),
            Topic::Hashing => write!(f, "Hashing"),
            Topic::Strings => write!(f, "Strings"),
            Topic::Recursion => write!(f, "Recursion"),
            Topic::Backtracking => write!(f, "Backtracking"),
            Topic::Greedy => write!(f, "Greedy"),
            Topic::Math => write!(f, "Math"),
        }
    }
}

impl std::fmt::Display for SolutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolutionStatus::Pending => write!(f, "Pending"),
            SolutionStatus::Accepted => write!(f, "Accepted"),
            SolutionStatus::WrongAnswer => write!(f, "Wrong Answer"),
            SolutionStatus::CompilationError => write!(f, "Compilation Error"),
            SolutionStatus::RuntimeError => write!(f, "Runtime Error"),
            SolutionStatus::TimeLimitExceeded => write!(f, "Time Limit Exceeded"),
        }
    }
}
