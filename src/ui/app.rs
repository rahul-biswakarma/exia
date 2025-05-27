use super::{
    ApiCall, ApiCallStatus, AppData, AppState, NetworkActivity, NetworkActivityType, NetworkStatus,
};
use crate::compiler::RustCompiler;
use crate::llm::GeminiClient;
use crate::models::{ActionContext, ActionType, LLMUsage, UserAction};
use crate::storage::Storage;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tui_input::backend::crossterm::EventHandler;
use uuid::Uuid;

pub struct App {
    pub state: AppState,
    pub data: AppData,
    pub should_quit: bool,
    storage: Storage,
    llm_client: Option<GeminiClient>,
    compiler: Arc<Mutex<Option<RustCompiler>>>,
    current_session_id: Option<Uuid>,
    user_id: String,
    action_start_time: Option<Instant>,
}

impl App {
    async fn log_user_action(
        &mut self,
        action_type: ActionType,
        screen: &str,
        element: Option<&str>,
        previous_screen: Option<&str>,
    ) {
        if let Some(session_id) = self.current_session_id {
            let duration_ms = self
                .action_start_time
                .map(|start| start.elapsed().as_millis() as u64);

            let mut metadata = HashMap::new();
            metadata.insert("user_id".to_string(), self.user_id.clone());
            metadata.insert("app_state".to_string(), format!("{:?}", self.state));

            let action = UserAction {
                id: Uuid::new_v4(),
                session_id,
                timestamp: chrono::Utc::now(),
                action_type,
                context: ActionContext {
                    screen: screen.to_string(),
                    element: element.map(|s| s.to_string()),
                    previous_screen: previous_screen.map(|s| s.to_string()),
                    question_id: self.data.current_question.as_ref().map(|q| q.id),
                    solution_id: self.data.current_solution.as_ref().map(|s| s.id),
                },
                duration_ms,
                metadata,
            };

            let _ = self.storage.save_user_action(&action);
            self.action_start_time = Some(Instant::now());
        }
    }

    async fn log_llm_usage(&mut self, usage: LLMUsage) {
        self.data.current_llm_usage.push(usage.clone());
        let _ = self.storage.save_llm_usage(&usage);

        // Update cost analytics
        if let Ok(cost_analytics) = self.storage.get_cost_analytics() {
            self.data.cost_analytics = Some(cost_analytics);
        }
    }

    fn log_network_activity(
        &mut self,
        endpoint: &str,
        activity_type: NetworkActivityType,
        status: NetworkStatus,
        latency_ms: u64,
    ) {
        let activity = NetworkActivity {
            timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
            activity_type,
            endpoint: endpoint.to_string(),
            status,
            latency_ms,
            bytes_sent: 0, // Could be enhanced to track actual bytes
            bytes_received: 0,
        };

        self.data.network_activity.push(activity);

        // Keep only last 10 activities
        if self.data.network_activity.len() > 10 {
            self.data.network_activity.remove(0);
        }
    }

    fn update_typing_speed(&mut self, char_count: usize) {
        let now = Instant::now();

        if let Some(last_keystroke) = self.data.typing_speed.last_keystroke {
            let interval_ms = now.duration_since(last_keystroke).as_millis() as u64;

            // Add interval to history
            self.data.typing_speed.keystroke_intervals.push(interval_ms);

            // Keep only last 10 intervals for WPM calculation
            if self.data.typing_speed.keystroke_intervals.len() > 10 {
                self.data.typing_speed.keystroke_intervals.remove(0);
            }

            // Calculate current WPM based on recent intervals
            if self.data.typing_speed.keystroke_intervals.len() >= 2 {
                let total_time_ms: u64 = self.data.typing_speed.keystroke_intervals.iter().sum();
                let avg_interval_ms =
                    total_time_ms as f64 / self.data.typing_speed.keystroke_intervals.len() as f64;

                if avg_interval_ms > 0.0 {
                    // WPM = (characters per minute) / 5 (average word length)
                    let chars_per_minute = 60000.0 / avg_interval_ms;
                    self.data.typing_speed.current_wpm = chars_per_minute / 5.0;
                }
            }
        }

        // Update totals
        self.data.typing_speed.total_characters += char_count as u64;
        self.data.typing_speed.last_keystroke = Some(now);

        // Update average WPM
        if self.data.typing_speed.total_time_ms > 0 {
            let total_minutes = self.data.typing_speed.total_time_ms as f64 / 60000.0;
            let total_words = self.data.typing_speed.total_characters as f64 / 5.0;
            self.data.typing_speed.average_wpm = total_words / total_minutes;
        }
    }

    fn log_api_call(&mut self, endpoint: &str, status: ApiCallStatus, message: &str) {
        let timestamp = chrono::Utc::now().format("%H:%M:%S").to_string();
        let api_call = ApiCall {
            timestamp,
            endpoint: endpoint.to_string(),
            status: status.clone(),
            message: message.to_string(),
        };
        self.data.api_calls.push(api_call);

        // Update error/success counters
        match status {
            ApiCallStatus::Success => self.data.success_count += 1,
            ApiCallStatus::Error => self.data.error_count += 1,
            ApiCallStatus::Pending => {} // Don't count pending
        }

        // Keep only last 10 calls
        if self.data.api_calls.len() > 10 {
            self.data.api_calls.remove(0);
        }
    }

    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;

        // Try to initialize Gemini client (might fail if API key not set)
        let llm_client = match GeminiClient::new() {
            Ok(client) => Some(client),
            Err(_) => None,
        };

        let user_id = format!("user_{}", Uuid::new_v4().to_string()[..8].to_string());

        let mut app = Self {
            state: AppState::Home,
            data: AppData::default(),
            should_quit: false,
            storage,
            llm_client,
            compiler: Arc::new(Mutex::new(None)),
            current_session_id: None,
            user_id: user_id.clone(),
            action_start_time: None,
        };

        // Load initial statistics
        if let Ok(stats) = app.storage.get_statistics() {
            app.data.statistics = Some(stats);
        }

        // Load recent questions
        if let Ok(questions) = app.storage.get_recent_questions(5) {
            app.data.recent_questions = questions;
        }

        // Start a new session
        if let Ok(session) = app.storage.start_session() {
            app.current_session_id = Some(session.id);

            // Note: Session start logging will be done after app is created
        }

        // Load analytics data
        if let Ok(cost_analytics) = app.storage.get_cost_analytics() {
            app.data.cost_analytics = Some(cost_analytics);
        }

        if let Ok(user_analytics) = app.storage.get_user_analytics(&user_id) {
            app.data.user_analytics = Some(user_analytics);
        }

        Ok(app)
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    if let Some(session_id) = self.current_session_id {
                        let _ = self.storage.end_session(&session_id);
                    }
                }
                KeyCode::Esc => {
                    self.handle_escape().await?;
                }
                _ => {
                    self.handle_key_event(key.code, key.modifiers).await?;
                }
            },
            _ => {}
        }
        Ok(())
    }

    async fn handle_escape(&mut self) -> Result<()> {
        match self.state {
            AppState::Home => {
                self.should_quit = true;
                if let Some(session_id) = self.current_session_id {
                    let _ = self.storage.end_session(&session_id);
                }
            }
            AppState::QuestionView => {
                self.state = AppState::Home;
                self.data.show_hints = false;
                self.data.hint_index = 0;
            }
            AppState::CodeEditor => {
                self.state = AppState::QuestionView;
                self.data.is_loading = false;
            }
            AppState::Results => {
                self.state = AppState::QuestionView;
                self.data.feedback_text.clear();
            }
            AppState::Statistics | AppState::Settings | AppState::Help => {
                self.state = AppState::Home;
            }
        }
        Ok(())
    }

    async fn handle_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match self.state {
            AppState::Home => self.handle_home_keys(key).await?,
            AppState::QuestionView => self.handle_question_keys(key).await?,
            AppState::CodeEditor => self.handle_editor_keys(key, modifiers).await?,
            AppState::Results => self.handle_results_keys(key).await?,
            AppState::Statistics | AppState::Settings => {
                // These screens only handle Esc (already handled above)
            }
            AppState::Help => self.handle_help_keys(key).await?,
        }
        Ok(())
    }

    async fn handle_home_keys(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Up => {
                if self.data.selected_tab > 0 {
                    self.data.selected_tab -= 1;
                }
            }
            KeyCode::Down => {
                if self.data.selected_tab < 5 {
                    // 6 total options (0-5)
                    self.data.selected_tab += 1;
                }
            }
            KeyCode::Left => {
                // Navigate recent questions list
                if !self.data.recent_questions.is_empty() {
                    let current = self.data.recent_questions_state.selected().unwrap_or(0);
                    if current > 0 {
                        self.data.recent_questions_state.select(Some(current - 1));
                    }
                }
            }
            KeyCode::Right => {
                // Navigate recent questions list
                if !self.data.recent_questions.is_empty() {
                    let current = self.data.recent_questions_state.selected().unwrap_or(0);
                    if current < self.data.recent_questions.len() - 1 {
                        self.data.recent_questions_state.select(Some(current + 1));
                    }
                }
            }
            KeyCode::Enter => match self.data.selected_tab {
                0 => {
                    self.log_api_call(
                        "user_input",
                        ApiCallStatus::Pending,
                        "Selected 'Generate New Question'",
                    );
                    self.data.is_loading = true; // Set loading state immediately
                    self.data.status_message = "Preparing to generate question...".to_string();
                    self.generate_new_question().await?;
                }
                1 => self.view_recent_questions().await?,
                2 => self.view_statistics().await?,
                3 => self.state = AppState::Settings,
                4 => self.state = AppState::Help,
                5 => {
                    self.should_quit = true;
                    if let Some(session_id) = self.current_session_id {
                        let _ = self.storage.end_session(&session_id);
                    }
                }
                _ => {}
            },
            KeyCode::Char('g') => {
                self.log_api_call("user_input", ApiCallStatus::Pending, "Pressed 'g' key");
                self.data.is_loading = true; // Set loading state immediately
                self.data.status_message = "Preparing to generate question...".to_string();
                self.generate_new_question().await?;
            }
            KeyCode::Char('r') => self.view_recent_questions().await?,
            KeyCode::Char('s') => self.view_statistics().await?,
            KeyCode::Char('h') => self.state = AppState::Help,
            KeyCode::Tab => {
                // Select a recent question if one is highlighted
                if let Some(selected) = self.data.recent_questions_state.selected() {
                    if selected < self.data.recent_questions.len() {
                        self.data.current_question =
                            Some(self.data.recent_questions[selected].clone());
                        self.state = AppState::QuestionView;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_question_keys(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('c') => {
                self.state = AppState::CodeEditor;
                // Initialize code editor with a better template if empty
                if self.data.text_editor.content().is_empty() {
                    let template = if let Some(question) = &self.data.current_question {
                        match question.topic {
                            crate::models::Topic::Arrays => {
                                r#"fn solution(input: &str) -> String {
    // Parse input - example for array problems
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Your solution here
    // TODO: Implement your algorithm

    // Return result as string
    "0".to_string()
}"#
                            }
                            crate::models::Topic::Strings => {
                                r#"fn solution(input: &str) -> String {
    // Parse input string
    let s = input.trim();

    // Your solution here
    // TODO: Implement your string algorithm

    // Return result
    s.to_string()
}"#
                            }
                            _ => {
                                r#"fn solution(input: &str) -> String {
    // Parse input based on problem requirements
    let data = input.trim();

    // Your solution here
    // TODO: Implement your algorithm

    // Return result as string
    "0".to_string()
}"#
                            }
                        }
                    } else {
                        r#"fn solution(input: &str) -> String {
    // Your solution here
    // TODO: Implement your algorithm

    input.to_string()
}"#
                    };
                    self.data.text_editor.set_content(template.to_string());
                }
            }
            KeyCode::Char('h') => {
                self.data.show_hints = !self.data.show_hints;
                if self.data.show_hints {
                    self.data.hint_index = 0;
                }
            }
            KeyCode::Char('n') => {
                if self.data.show_hints {
                    if let Some(question) = &self.data.current_question {
                        if self.data.hint_index < question.hints.len() - 1 {
                            self.data.hint_index += 1;
                        }
                    }
                }
            }
            KeyCode::Char('p') => {
                if self.data.show_hints && self.data.hint_index > 0 {
                    self.data.hint_index -= 1;
                }
            }
            KeyCode::Up => {
                if self.data.scroll_offset > 0 {
                    self.data.scroll_offset -= 1;
                }
            }
            KeyCode::Down => {
                self.data.scroll_offset += 1;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_editor_keys(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (key, modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                self.submit_solution().await?;
            }
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.get_hint_for_code().await?;
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.data.text_editor.clear();
            }
            // Arrow key navigation
            (KeyCode::Up, KeyModifiers::NONE) => {
                self.data.text_editor.move_cursor_up();
            }
            (KeyCode::Down, KeyModifiers::NONE) => {
                self.data.text_editor.move_cursor_down();
            }
            (KeyCode::Left, KeyModifiers::NONE) => {
                self.data.text_editor.move_cursor_left();
            }
            (KeyCode::Right, KeyModifiers::NONE) => {
                self.data.text_editor.move_cursor_right();
            }
            (KeyCode::Home, KeyModifiers::NONE) => {
                self.data.text_editor.move_to_line_start();
            }
            (KeyCode::End, KeyModifiers::NONE) => {
                self.data.text_editor.move_to_line_end();
            }
            (KeyCode::PageUp, KeyModifiers::NONE) => {
                self.data.text_editor.page_up(10);
            }
            (KeyCode::PageDown, KeyModifiers::NONE) => {
                self.data.text_editor.page_down(10);
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                self.data.text_editor.delete_char();
                self.update_typing_speed(1);
            }
            (KeyCode::Delete, KeyModifiers::NONE) => {
                self.data.text_editor.delete_forward();
                self.update_typing_speed(1);
            }
            (KeyCode::Enter, KeyModifiers::NONE) => {
                self.data.text_editor.insert_char('\n');
                self.update_typing_speed(1);
            }
            (KeyCode::Tab, KeyModifiers::NONE) => {
                self.data.text_editor.insert_str("    "); // 4 spaces for tab
                self.update_typing_speed(4);
            }
            (KeyCode::Char(ch), KeyModifiers::NONE) => {
                self.data.text_editor.insert_char(ch);
                self.update_typing_speed(1);
            }
            (KeyCode::Char(ch), KeyModifiers::SHIFT) => {
                self.data.text_editor.insert_char(ch);
                self.update_typing_speed(1);
            }
            _ => {
                // Ignore other key combinations
            }
        }
        Ok(())
    }

    async fn handle_results_keys(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('f') => {
                self.get_detailed_feedback().await?;
            }
            KeyCode::Char('r') => {
                self.state = AppState::CodeEditor;
            }
            KeyCode::Char('n') => {
                self.generate_new_question().await?;
            }
            KeyCode::Up => {
                if self.data.scroll_offset > 0 {
                    self.data.scroll_offset -= 1;
                }
            }
            KeyCode::Down => {
                self.data.scroll_offset += 1;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_help_keys(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Up => {
                if self.data.scroll_offset > 0 {
                    self.data.scroll_offset -= 1;
                }
            }
            KeyCode::Down => {
                self.data.scroll_offset += 1;
            }
            _ => {}
        }
        Ok(())
    }

    async fn generate_new_question(&mut self) -> Result<()> {
        self.log_api_call(
            "generate_question",
            ApiCallStatus::Pending,
            "Starting question generation",
        );

        if self.llm_client.is_none() {
            self.log_api_call("generate_question", ApiCallStatus::Error, "No API key set");
            self.data.status_message =
                "Error: Gemini API key not set. Please set GEMINI_API_KEY environment variable."
                    .to_string();
            return Ok(());
        }

        self.data.is_loading = true;
        self.data.status_message = "Generating new question...".to_string();

        let progress = self.storage.get_progress()?;

        if let Some(client) = &self.llm_client {
            let client = client.clone(); // Clone the client to avoid borrowing issues
            self.log_api_call("gemini_api", ApiCallStatus::Pending, "Calling Gemini API");

            // Log network activity start
            self.log_network_activity(
                "Gemini API",
                NetworkActivityType::ApiCall,
                NetworkStatus::InProgress,
                0,
            );

            if let Some(session_id) = self.current_session_id {
                let start_time = Instant::now();
                match client.generate_question(&progress, session_id).await {
                    Ok((question, usage)) => {
                        let latency = start_time.elapsed().as_millis() as u64;

                        self.log_api_call(
                            "gemini_api",
                            ApiCallStatus::Success,
                            "Question generated",
                        );
                        self.log_llm_usage(usage).await;

                        // Log successful network activity
                        self.log_network_activity(
                            "Gemini API",
                            NetworkActivityType::ApiCall,
                            NetworkStatus::Success,
                            latency,
                        );

                        // Save question to storage
                        self.storage.save_question(&question)?;

                        // Add to current session
                        let _ = self
                            .storage
                            .add_question_to_session(&session_id, &question.id);

                        self.data.current_question = Some(question.clone());

                        // Update recent questions list
                        self.data.recent_questions.insert(0, question);
                        if self.data.recent_questions.len() > 5 {
                            self.data.recent_questions.truncate(5);
                        }

                        self.state = AppState::QuestionView;
                        self.data.status_message = "Question generated successfully!".to_string();
                        self.log_api_call("generate_question", ApiCallStatus::Success, "Complete");

                        // Log navigation action
                        self.log_user_action(
                            ActionType::Navigation,
                            "QuestionView",
                            None,
                            Some("Home"),
                        )
                        .await;
                    }
                    Err(e) => {
                        let latency = start_time.elapsed().as_millis() as u64;

                        self.log_api_call(
                            "gemini_api",
                            ApiCallStatus::Error,
                            &format!("API Error: {}", e),
                        );

                        // Log failed network activity
                        self.log_network_activity(
                            "Gemini API",
                            NetworkActivityType::ApiCall,
                            NetworkStatus::Failed,
                            latency,
                        );

                        self.data.status_message = format!("Error generating question: {}", e);

                        // Log error action
                        self.log_user_action(
                            ActionType::Error,
                            "Home",
                            Some("question_generation"),
                            None,
                        )
                        .await;
                    }
                }
            }
        }

        self.data.is_loading = false;
        Ok(())
    }

    async fn view_recent_questions(&mut self) -> Result<()> {
        match self.storage.get_recent_questions(10) {
            Ok(questions) => {
                if let Some(question) = questions.first() {
                    self.data.current_question = Some(question.clone());
                    self.state = AppState::QuestionView;
                } else {
                    self.data.status_message =
                        "No recent questions found. Generate a new question first.".to_string();
                }
            }
            Err(e) => {
                self.data.status_message = format!("Error loading recent questions: {}", e);
            }
        }
        Ok(())
    }

    async fn view_statistics(&mut self) -> Result<()> {
        match self.storage.get_statistics() {
            Ok(stats) => {
                self.data.statistics = Some(stats);
                self.state = AppState::Statistics;
            }
            Err(e) => {
                self.data.status_message = format!("Error loading statistics: {}", e);
            }
        }
        Ok(())
    }

    async fn submit_solution(&mut self) -> Result<()> {
        if self.data.current_question.is_none() {
            self.data.status_message = "No question loaded.".to_string();
            return Ok(());
        }

        self.data.is_loading = true;
        self.data.status_message = "Compiling and testing solution...".to_string();

        let code = self.data.text_editor.content().to_string();
        let question = self.data.current_question.as_ref().unwrap();

        // Initialize compiler if needed
        {
            let mut compiler_guard = self.compiler.lock().await;
            if compiler_guard.is_none() {
                match RustCompiler::new() {
                    Ok(compiler) => *compiler_guard = Some(compiler),
                    Err(e) => {
                        self.data.status_message = format!("Error initializing compiler: {}", e);
                        self.data.is_loading = false;
                        return Ok(());
                    }
                }
            }
        }

        // Compile and test
        let compiler_guard = self.compiler.lock().await;
        if let Some(compiler) = compiler_guard.as_ref() {
            match compiler.compile_and_test(&code, &question.test_cases).await {
                Ok(mut solution) => {
                    solution.question_id = question.id;

                    // Save solution
                    self.storage.save_solution(&solution)?;

                    // Add to current session
                    if let Some(session_id) = self.current_session_id {
                        let _ = self
                            .storage
                            .add_solution_to_session(&session_id, &solution.id);
                    }

                    self.data.current_solution = Some(solution);
                    self.state = AppState::Results;

                    // Update statistics
                    if let Ok(stats) = self.storage.get_statistics() {
                        self.data.statistics = Some(stats);
                    }
                }
                Err(e) => {
                    self.data.status_message = format!("Error compiling solution: {}", e);
                }
            }
        }

        self.data.is_loading = false;
        Ok(())
    }

    async fn get_hint_for_code(&mut self) -> Result<()> {
        if let (Some(client), Some(question), Some(session_id)) = (
            &self.llm_client,
            &self.data.current_question,
            self.current_session_id,
        ) {
            let code = self.data.text_editor.content();
            match client.generate_hint(question, code, session_id).await {
                Ok((hint, usage)) => {
                    self.log_llm_usage(usage).await;
                    self.data.status_message = format!("Hint: {}", hint);
                }
                Err(e) => {
                    self.data.status_message = format!("Error getting hint: {}", e);
                }
            }
        } else {
            self.data.status_message =
                "Gemini API not available or no question loaded.".to_string();
        }
        Ok(())
    }

    async fn get_detailed_feedback(&mut self) -> Result<()> {
        if let (Some(client), Some(solution), Some(question), Some(session_id)) = (
            &self.llm_client,
            &self.data.current_solution,
            &self.data.current_question,
            self.current_session_id,
        ) {
            self.data.feedback_text = "Generating detailed feedback...".to_string();

            match client
                .provide_feedback(solution, question, session_id)
                .await
            {
                Ok((feedback, usage)) => {
                    self.log_llm_usage(usage).await;
                    self.data.feedback_text = feedback;
                }
                Err(e) => {
                    self.data.feedback_text = format!("Error generating feedback: {}", e);
                }
            }
        } else {
            self.data.feedback_text =
                "Cannot generate feedback: missing data or API not available.".to_string();
        }
        Ok(())
    }
}
