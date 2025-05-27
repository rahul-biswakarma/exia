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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMUsage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub model_name: String,
    pub endpoint: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    pub cost_usd: f64,
    pub latency_ms: u64,
    pub request_type: LLMRequestType,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMRequestType {
    QuestionGeneration,
    HintGeneration,
    FeedbackGeneration,
    CodeAnalysis,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAction {
    pub id: Uuid,
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action_type: ActionType,
    pub context: ActionContext,
    pub duration_ms: Option<u64>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Navigation,
    KeyPress,
    CodeEdit,
    QuestionGeneration,
    SolutionSubmission,
    HintRequest,
    FeedbackRequest,
    ScrollAction,
    TabSwitch,
    SessionStart,
    SessionEnd,
    Error,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub screen: String,
    pub element: Option<String>,
    pub previous_screen: Option<String>,
    pub question_id: Option<Uuid>,
    pub solution_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehaviorPattern {
    pub id: Uuid,
    pub user_id: String, // Could be device ID or user identifier
    pub pattern_type: BehaviorPatternType,
    pub frequency: u32,
    pub last_occurrence: DateTime<Utc>,
    pub confidence_score: f64,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorPatternType {
    FrequentKeySequence,
    NavigationPattern,
    CodingStyle,
    ErrorPattern,
    TimePattern,
    PerformancePattern,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalytics {
    pub total_cost_usd: f64,
    pub cost_by_model: std::collections::HashMap<String, f64>,
    pub cost_by_request_type: std::collections::HashMap<String, f64>,
    pub tokens_used: u64,
    pub requests_count: u64,
    pub average_cost_per_request: f64,
    pub cost_trend: Vec<CostDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostDataPoint {
    pub timestamp: DateTime<Utc>,
    pub cumulative_cost: f64,
    pub session_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnalytics {
    pub user_id: String,
    pub total_sessions: u64,
    pub total_time_spent_ms: u64,
    pub actions_count: u64,
    pub most_used_features: Vec<(String, u64)>,
    pub error_rate: f64,
    pub productivity_score: f64,
    pub learning_velocity: f64,
    pub behavior_patterns: Vec<UserBehaviorPattern>,
    pub cost_analytics: CostAnalytics,
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
