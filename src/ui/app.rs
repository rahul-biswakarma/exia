use super::{AppData, AppState};
use crate::compiler::RustCompiler;
use crate::llm::GeminiClient;
// use crate::models::{Question, Solution, SolutionStatus};
use crate::storage::Storage;
use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::sync::Arc;
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
}

impl App {
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;

        // Try to initialize Gemini client (might fail if API key not set)
        let llm_client = match GeminiClient::new() {
            Ok(client) => Some(client),
            Err(_) => None,
        };

        let mut app = Self {
            state: AppState::Home,
            data: AppData::default(),
            should_quit: false,
            storage,
            llm_client,
            compiler: Arc::new(Mutex::new(None)),
            current_session_id: None,
        };

        // Load initial statistics
        if let Ok(stats) = app.storage.get_statistics() {
            app.data.statistics = Some(stats);
        }

        // Start a new session
        if let Ok(session) = app.storage.start_session() {
            app.current_session_id = Some(session.id);
        }

        Ok(app)
    }

    pub async fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => {
                match key.code {
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
                }
            }
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
            KeyCode::Enter => {
                match self.data.selected_tab {
                    0 => self.generate_new_question().await?,
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
                }
            }
            KeyCode::Char('g') => self.generate_new_question().await?,
            KeyCode::Char('r') => self.view_recent_questions().await?,
            KeyCode::Char('s') => self.view_statistics().await?,
            KeyCode::Char('h') => self.state = AppState::Help,
            _ => {}
        }
        Ok(())
    }

    async fn handle_question_keys(&mut self, key: KeyCode) -> Result<()> {
        match key {
            KeyCode::Char('c') => {
                self.state = AppState::CodeEditor;
                // Initialize code editor with a template if empty
                if self.data.code_input.value().is_empty() {
                    let template = r#"fn solution(input: &str) -> String {
    // Your solution here
    input.to_string()
}"#;
                    self.data.code_input = tui_input::Input::new(template.to_string());
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
                self.data.code_input = tui_input::Input::default();
            }
            _ => {
                // Handle regular text input
                self.data.code_input.handle_event(&Event::Key(crossterm::event::KeyEvent {
                    code: key,
                    modifiers,
                    kind: crossterm::event::KeyEventKind::Press,
                    state: crossterm::event::KeyEventState::NONE,
                }));
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
        if self.llm_client.is_none() {
            self.data.status_message = "Error: Gemini API key not set. Please set GEMINI_API_KEY environment variable.".to_string();
            return Ok(());
        }

        self.data.is_loading = true;
        self.data.status_message = "Generating new question...".to_string();

        let progress = self.storage.get_progress()?;

        if let Some(client) = &self.llm_client {
            match client.generate_question(&progress).await {
                Ok(question) => {
                    // Save question to storage
                    self.storage.save_question(&question)?;

                    // Add to current session
                    if let Some(session_id) = self.current_session_id {
                        let _ = self.storage.add_question_to_session(&session_id, &question.id);
                    }

                    self.data.current_question = Some(question);
                    self.state = AppState::QuestionView;
                    self.data.status_message = "Question generated successfully!".to_string();
                }
                Err(e) => {
                    self.data.status_message = format!("Error generating question: {}", e);
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
                    self.data.status_message = "No recent questions found. Generate a new question first.".to_string();
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

        let code = self.data.code_input.value().to_string();
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
                        let _ = self.storage.add_solution_to_session(&session_id, &solution.id);
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
        if let (Some(client), Some(question)) = (&self.llm_client, &self.data.current_question) {
            let code = self.data.code_input.value();
            match client.generate_hint(question, code).await {
                Ok(hint) => {
                    self.data.status_message = format!("Hint: {}", hint);
                }
                Err(e) => {
                    self.data.status_message = format!("Error getting hint: {}", e);
                }
            }
        } else {
            self.data.status_message = "Gemini API not available or no question loaded.".to_string();
        }
        Ok(())
    }

    async fn get_detailed_feedback(&mut self) -> Result<()> {
        if let (Some(client), Some(solution), Some(question)) = (
            &self.llm_client,
            &self.data.current_solution,
            &self.data.current_question,
        ) {
            self.data.feedback_text = "Generating detailed feedback...".to_string();

            match client.provide_feedback(solution, question).await {
                Ok(feedback) => {
                    self.data.feedback_text = feedback;
                }
                Err(e) => {
                    self.data.feedback_text = format!("Error generating feedback: {}", e);
                }
            }
        } else {
            self.data.feedback_text = "Cannot generate feedback: missing data or API not available.".to_string();
        }
        Ok(())
    }
}
