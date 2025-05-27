use crate::models::{
    ActionContext, ActionType, BehaviorPatternType, CostAnalytics, CostDataPoint, Difficulty,
    DifficultyProgress, LLMRequestType, LLMUsage, LearningProgress, Question, Session, Solution,
    Topic, TopicProgress, UserAction, UserAnalytics, UserBehaviorPattern,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub questions: HashMap<Uuid, Question>,
    pub solutions: HashMap<Uuid, Solution>,
    pub progress: LearningProgress,
    pub sessions: HashMap<Uuid, Session>,
    pub llm_usage: HashMap<Uuid, LLMUsage>,
    pub user_actions: HashMap<Uuid, UserAction>,
    pub behavior_patterns: HashMap<Uuid, UserBehaviorPattern>,
    pub user_analytics: Option<UserAnalytics>,
}

pub struct Storage {
    data_dir: PathBuf,
    db_file: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow!("Could not find data directory"))?
            .join("dsa_learning_assistant");

        fs::create_dir_all(&data_dir)?;

        let db_file = data_dir.join("database.json");

        Ok(Self { data_dir, db_file })
    }

    pub fn load_database(&self) -> Result<Database> {
        if !self.db_file.exists() {
            return Ok(self.create_default_database());
        }

        let content = fs::read_to_string(&self.db_file)?;

        // Try to parse with new schema first
        match serde_json::from_str::<Database>(&content) {
            Ok(database) => Ok(database),
            Err(_) => {
                // If parsing fails, try to migrate from old schema
                self.migrate_database(&content)
            }
        }
    }

    fn migrate_database(&self, content: &str) -> Result<Database> {
        // Try to parse as old database format
        #[derive(serde::Deserialize)]
        struct OldDatabase {
            questions: HashMap<Uuid, Question>,
            solutions: HashMap<Uuid, Solution>,
            progress: LearningProgress,
            sessions: HashMap<Uuid, Session>,
        }

        match serde_json::from_str::<OldDatabase>(content) {
            Ok(old_db) => {
                // Migrate to new format
                let new_db = Database {
                    questions: old_db.questions,
                    solutions: old_db.solutions,
                    progress: old_db.progress,
                    sessions: old_db.sessions,
                    llm_usage: HashMap::new(),
                    user_actions: HashMap::new(),
                    behavior_patterns: HashMap::new(),
                    user_analytics: None,
                };

                // Save migrated database
                self.save_database(&new_db)?;
                Ok(new_db)
            }
            Err(e) => Err(anyhow!("Failed to migrate database: {}", e))
        }
    }

    pub fn save_database(&self, database: &Database) -> Result<()> {
        let content = serde_json::to_string_pretty(database)?;
        fs::write(&self.db_file, content)?;
        Ok(())
    }

    pub fn save_question(&self, question: &Question) -> Result<()> {
        let mut db = self.load_database()?;
        db.questions.insert(question.id, question.clone());
        self.save_database(&db)
    }

    pub fn save_solution(&self, solution: &Solution) -> Result<()> {
        let mut db = self.load_database()?;
        db.solutions.insert(solution.id, solution.clone());

        // Update learning progress
        self.update_progress_with_solution(&mut db.progress, solution, &db.questions)?;

        self.save_database(&db)
    }

    pub fn get_question(&self, id: &Uuid) -> Result<Option<Question>> {
        let db = self.load_database()?;
        Ok(db.questions.get(id).cloned())
    }

    pub fn get_solution(&self, id: &Uuid) -> Result<Option<Solution>> {
        let db = self.load_database()?;
        Ok(db.solutions.get(id).cloned())
    }

    pub fn get_progress(&self) -> Result<LearningProgress> {
        let db = self.load_database()?;
        Ok(db.progress)
    }

    pub fn get_recent_questions(&self, limit: usize) -> Result<Vec<Question>> {
        let db = self.load_database()?;
        let mut questions: Vec<Question> = db.questions.values().cloned().collect();
        questions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        questions.truncate(limit);
        Ok(questions)
    }

    pub fn get_solutions_for_question(&self, question_id: &Uuid) -> Result<Vec<Solution>> {
        let db = self.load_database()?;
        let solutions: Vec<Solution> = db
            .solutions
            .values()
            .filter(|s| s.question_id == *question_id)
            .cloned()
            .collect();
        Ok(solutions)
    }

    pub fn start_session(&self) -> Result<Session> {
        let session = Session {
            id: Uuid::new_v4(),
            started_at: Utc::now(),
            ended_at: None,
            questions_attempted: Vec::new(),
            solutions_submitted: Vec::new(),
        };

        let mut db = self.load_database()?;
        db.sessions.insert(session.id, session.clone());
        self.save_database(&db)?;

        Ok(session)
    }

    pub fn end_session(&self, session_id: &Uuid) -> Result<()> {
        let mut db = self.load_database()?;
        if let Some(session) = db.sessions.get_mut(session_id) {
            session.ended_at = Some(Utc::now());
            self.save_database(&db)?;
        }
        Ok(())
    }

    pub fn add_question_to_session(&self, session_id: &Uuid, question_id: &Uuid) -> Result<()> {
        let mut db = self.load_database()?;
        if let Some(session) = db.sessions.get_mut(session_id) {
            session.questions_attempted.push(*question_id);
            self.save_database(&db)?;
        }
        Ok(())
    }

    pub fn add_solution_to_session(&self, session_id: &Uuid, solution_id: &Uuid) -> Result<()> {
        let mut db = self.load_database()?;
        if let Some(session) = db.sessions.get_mut(session_id) {
            session.solutions_submitted.push(*solution_id);
            self.save_database(&db)?;
        }
        Ok(())
    }

    pub fn get_statistics(&self) -> Result<Statistics> {
        let db = self.load_database()?;

        let total_questions = db.questions.len();
        let total_solutions = db.solutions.len();
        let accepted_solutions = db
            .solutions
            .values()
            .filter(|s| matches!(s.status, crate::models::SolutionStatus::Accepted))
            .count();

        let avg_execution_time = if !db.solutions.is_empty() {
            db.solutions
                .values()
                .filter_map(|s| s.execution_time)
                .sum::<u64>() as f64
                / db.solutions.len() as f64
        } else {
            0.0
        };

        let topic_distribution: HashMap<Topic, usize> =
            db.questions.values().fold(HashMap::new(), |mut acc, q| {
                *acc.entry(q.topic.clone()).or_insert(0) += 1;
                acc
            });

        let difficulty_distribution: HashMap<Difficulty, usize> =
            db.questions.values().fold(HashMap::new(), |mut acc, q| {
                *acc.entry(q.difficulty.clone()).or_insert(0) += 1;
                acc
            });

        Ok(Statistics {
            total_questions,
            total_solutions,
            accepted_solutions,
            success_rate: if total_solutions > 0 {
                accepted_solutions as f64 / total_solutions as f64 * 100.0
            } else {
                0.0
            },
            avg_execution_time,
            topic_distribution,
            difficulty_distribution,
            current_streak: db.progress.streak,
        })
    }

    pub fn export_data(&self, export_path: &Path) -> Result<()> {
        let db = self.load_database()?;
        let content = serde_json::to_string_pretty(&db)?;
        fs::write(export_path, content)?;
        Ok(())
    }

    pub fn import_data(&self, import_path: &Path) -> Result<()> {
        let content = fs::read_to_string(import_path)?;
        let imported_db: Database = serde_json::from_str(&content)?;
        self.save_database(&imported_db)
    }

    // Analytics and Cost Tracking Methods
    pub fn save_llm_usage(&self, usage: &LLMUsage) -> Result<()> {
        let mut db = self.load_database()?;
        db.llm_usage.insert(usage.id, usage.clone());
        self.save_database(&db)
    }

    pub fn save_user_action(&self, action: &UserAction) -> Result<()> {
        let mut db = self.load_database()?;
        db.user_actions.insert(action.id, action.clone());
        self.save_database(&db)
    }

    pub fn get_cost_analytics(&self) -> Result<CostAnalytics> {
        let db = self.load_database()?;

        let total_cost_usd = db.llm_usage.values().map(|u| u.cost_usd).sum();
        let tokens_used = db.llm_usage.values().map(|u| u.total_tokens as u64).sum();
        let requests_count = db.llm_usage.len() as u64;

        let cost_by_model = db.llm_usage.values().fold(HashMap::new(), |mut acc, usage| {
            *acc.entry(usage.model_name.clone()).or_insert(0.0) += usage.cost_usd;
            acc
        });

        let cost_by_request_type = db.llm_usage.values().fold(HashMap::new(), |mut acc, usage| {
            let key = match &usage.request_type {
                LLMRequestType::QuestionGeneration => "Question Generation".to_string(),
                LLMRequestType::HintGeneration => "Hint Generation".to_string(),
                LLMRequestType::FeedbackGeneration => "Feedback Generation".to_string(),
                LLMRequestType::CodeAnalysis => "Code Analysis".to_string(),
                LLMRequestType::Other(s) => s.clone(),
            };
            *acc.entry(key).or_insert(0.0) += usage.cost_usd;
            acc
        });

        let average_cost_per_request = if requests_count > 0 {
            total_cost_usd / requests_count as f64
        } else {
            0.0
        };

        // Generate cost trend data
        let mut cost_trend = Vec::new();
        let mut cumulative_cost = 0.0;
        let mut usage_by_time: Vec<_> = db.llm_usage.values().collect();
        usage_by_time.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        for usage in usage_by_time {
            cumulative_cost += usage.cost_usd;
            cost_trend.push(CostDataPoint {
                timestamp: usage.timestamp,
                cumulative_cost,
                session_cost: usage.cost_usd,
            });
        }

        Ok(CostAnalytics {
            total_cost_usd,
            cost_by_model,
            cost_by_request_type,
            tokens_used,
            requests_count,
            average_cost_per_request,
            cost_trend,
        })
    }

    pub fn get_user_analytics(&self, user_id: &str) -> Result<UserAnalytics> {
        let db = self.load_database()?;

        let user_actions: Vec<_> = db.user_actions.values()
            .filter(|a| a.session_id.to_string().contains(user_id)) // Simple user filtering
            .collect();

        let total_sessions = db.sessions.len() as u64;
        let actions_count = user_actions.len() as u64;

        let total_time_spent_ms = user_actions.iter()
            .filter_map(|a| a.duration_ms)
            .sum();

        let most_used_features = user_actions.iter()
            .fold(HashMap::new(), |mut acc, action| {
                let feature = match &action.action_type {
                    ActionType::Navigation => "Navigation".to_string(),
                    ActionType::KeyPress => "Key Press".to_string(),
                    ActionType::CodeEdit => "Code Editing".to_string(),
                    ActionType::QuestionGeneration => "Question Generation".to_string(),
                    ActionType::SolutionSubmission => "Solution Submission".to_string(),
                    ActionType::HintRequest => "Hint Request".to_string(),
                    ActionType::FeedbackRequest => "Feedback Request".to_string(),
                    ActionType::Other(s) => s.clone(),
                    _ => "Other".to_string(),
                };
                *acc.entry(feature).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect::<Vec<_>>();

        let error_actions = user_actions.iter()
            .filter(|a| matches!(a.action_type, ActionType::Error))
            .count();

        let error_rate = if actions_count > 0 {
            error_actions as f64 / actions_count as f64
        } else {
            0.0
        };

        // Simple productivity and learning velocity calculations
        let productivity_score = if total_time_spent_ms > 0 {
            (actions_count as f64 / (total_time_spent_ms as f64 / 1000.0)) * 100.0
        } else {
            0.0
        };

        let learning_velocity = if total_sessions > 0 {
            db.progress.questions_solved as f64 / total_sessions as f64
        } else {
            0.0
        };

        let behavior_patterns = db.behavior_patterns.values()
            .filter(|p| p.user_id == user_id)
            .cloned()
            .collect();

        let cost_analytics = self.get_cost_analytics()?;

        Ok(UserAnalytics {
            user_id: user_id.to_string(),
            total_sessions,
            total_time_spent_ms,
            actions_count,
            most_used_features,
            error_rate,
            productivity_score,
            learning_velocity,
            behavior_patterns,
            cost_analytics,
        })
    }

    pub fn analyze_behavior_patterns(&self, user_id: &str) -> Result<Vec<UserBehaviorPattern>> {
        let db = self.load_database()?;
        let user_actions: Vec<_> = db.user_actions.values()
            .filter(|a| a.session_id.to_string().contains(user_id))
            .collect();

        let mut patterns = Vec::new();

        // Analyze frequent key sequences
        let key_sequences = self.find_key_sequences(&user_actions);
        for (sequence, frequency) in key_sequences {
            if frequency >= 3 { // Pattern threshold
                patterns.push(UserBehaviorPattern {
                    id: Uuid::new_v4(),
                    user_id: user_id.to_string(),
                    pattern_type: BehaviorPatternType::FrequentKeySequence,
                    frequency,
                    last_occurrence: Utc::now(),
                    confidence_score: (frequency as f64 / user_actions.len() as f64).min(1.0),
                    metadata: {
                        let mut map = HashMap::new();
                        map.insert("sequence".to_string(), sequence);
                        map
                    },
                });
            }
        }

        // Analyze navigation patterns
        let nav_patterns = self.find_navigation_patterns(&user_actions);
        for (pattern, frequency) in nav_patterns {
            patterns.push(UserBehaviorPattern {
                id: Uuid::new_v4(),
                user_id: user_id.to_string(),
                pattern_type: BehaviorPatternType::NavigationPattern,
                frequency,
                last_occurrence: Utc::now(),
                confidence_score: (frequency as f64 / user_actions.len() as f64).min(1.0),
                metadata: {
                    let mut map = HashMap::new();
                    map.insert("pattern".to_string(), pattern);
                    map
                },
            });
        }

        Ok(patterns)
    }

    fn find_key_sequences(&self, actions: &[&UserAction]) -> HashMap<String, u32> {
        let mut sequences = HashMap::new();

        let key_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a.action_type, ActionType::KeyPress))
            .collect();

        for window in key_actions.windows(3) {
            let sequence = window.iter()
                .map(|a| a.metadata.get("key").unwrap_or(&"unknown".to_string()).clone())
                .collect::<Vec<_>>()
                .join("->");
            *sequences.entry(sequence).or_insert(0) += 1;
        }

        sequences
    }

    fn find_navigation_patterns(&self, actions: &[&UserAction]) -> HashMap<String, u32> {
        let mut patterns = HashMap::new();

        let nav_actions: Vec<_> = actions.iter()
            .filter(|a| matches!(a.action_type, ActionType::Navigation))
            .collect();

        for window in nav_actions.windows(2) {
            let pattern = format!("{} -> {}",
                window[0].context.screen,
                window[1].context.screen
            );
            *patterns.entry(pattern).or_insert(0) += 1;
        }

        patterns
    }

    fn create_default_database(&self) -> Database {
        Database {
            questions: HashMap::new(),
            solutions: HashMap::new(),
            progress: LearningProgress {
                user_id: "default_user".to_string(),
                total_questions_attempted: 0,
                questions_solved: 0,
                topic_progress: HashMap::new(),
                difficulty_progress: HashMap::new(),
                streak: 0,
                last_activity: Utc::now(),
                strengths: Vec::new(),
                weaknesses: Vec::new(),
            },
            sessions: HashMap::new(),
            llm_usage: HashMap::new(),
            user_actions: HashMap::new(),
            behavior_patterns: HashMap::new(),
            user_analytics: None,
        }
    }

    fn update_progress_with_solution(
        &self,
        progress: &mut LearningProgress,
        solution: &Solution,
        questions: &HashMap<Uuid, Question>,
    ) -> Result<()> {
        if let Some(question) = questions.get(&solution.question_id) {
            progress.total_questions_attempted += 1;
            progress.last_activity = Utc::now();

            let is_solved = matches!(solution.status, crate::models::SolutionStatus::Accepted);

            if is_solved {
                progress.questions_solved += 1;
                progress.streak += 1;
            } else {
                progress.streak = 0;
            }

            // Update topic progress
            let topic_progress = progress
                .topic_progress
                .entry(question.topic.clone())
                .or_insert(TopicProgress {
                    attempted: 0,
                    solved: 0,
                    average_attempts: 0.0,
                    last_practiced: None,
                });

            topic_progress.attempted += 1;
            if is_solved {
                topic_progress.solved += 1;
            }
            topic_progress.last_practiced = Some(Utc::now());
            topic_progress.average_attempts =
                topic_progress.attempted as f32 / topic_progress.solved.max(1) as f32;

            // Update difficulty progress
            let difficulty_progress = progress
                .difficulty_progress
                .entry(question.difficulty.clone())
                .or_insert(DifficultyProgress {
                    attempted: 0,
                    solved: 0,
                    success_rate: 0.0,
                });

            difficulty_progress.attempted += 1;
            if is_solved {
                difficulty_progress.solved += 1;
            }
            difficulty_progress.success_rate =
                difficulty_progress.solved as f32 / difficulty_progress.attempted as f32 * 100.0;

            // Update strengths and weaknesses
            self.update_strengths_and_weaknesses(progress);
        }

        Ok(())
    }

    fn update_strengths_and_weaknesses(&self, progress: &mut LearningProgress) {
        let mut topic_scores: Vec<(Topic, f32)> = progress
            .topic_progress
            .iter()
            .filter(|(_, tp)| tp.attempted >= 3) // Only consider topics with at least 3 attempts
            .map(|(topic, tp)| {
                let success_rate = tp.solved as f32 / tp.attempted as f32;
                (topic.clone(), success_rate)
            })
            .collect();

        topic_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Top 3 topics are strengths
        progress.strengths = topic_scores
            .iter()
            .take(3)
            .filter(|(_, score)| *score >= 0.7) // At least 70% success rate
            .map(|(topic, _)| topic.clone())
            .collect();

        // Bottom 3 topics are weaknesses
        progress.weaknesses = topic_scores
            .iter()
            .rev()
            .take(3)
            .filter(|(_, score)| *score < 0.5) // Less than 50% success rate
            .map(|(topic, _)| topic.clone())
            .collect();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_questions: usize,
    pub total_solutions: usize,
    pub accepted_solutions: usize,
    pub success_rate: f64,
    pub avg_execution_time: f64,
    pub topic_distribution: HashMap<Topic, usize>,
    pub difficulty_distribution: HashMap<Difficulty, usize>,
    pub current_streak: u32,
}
