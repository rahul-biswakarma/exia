use crate::models::{CostAnalytics, LLMUsage, Question, Solution, SolutionStatus, UserAnalytics};
use crate::storage::Statistics;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{
        Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, ScrollbarState, Wrap,
    },
    Frame, Terminal,
};
use std::io;
use tui_input::Input;
use widgets::{
    EvaBorders, EvaColors, EvaFormat, EvaGradientWidget, EvaLoadingWidget, EvaOperationType,
    EvaProgressWidget, EvaStyles, EvaSymbols, EvaTypingWidget, TextEditor,
};

pub mod app;
pub mod components;
pub mod widgets;

pub use app::App;
pub use widgets::Widget;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Home,
    AllQuestions,
    QuestionView,
    CodeEditor,
    Results,
    Statistics,
    Settings,
    Help,
}

#[derive(Debug, Clone)]
pub struct AppData {
    pub current_question: Option<Question>,
    pub current_solution: Option<Solution>,
    pub statistics: Option<Statistics>,
    pub text_editor: TextEditor,
    pub selected_tab: usize,
    pub list_state: ListState,
    pub recent_questions_state: ListState,
    pub scroll_state: ScrollbarState,
    pub scroll_offset: usize,
    pub show_hints: bool,
    pub hint_index: usize,
    pub feedback_text: String,
    pub status_message: String,
    pub is_loading: bool,
    pub api_calls: Vec<ApiCall>,
    pub error_count: usize,
    pub success_count: usize,
    pub cost_analytics: Option<CostAnalytics>,
    pub user_analytics: Option<UserAnalytics>,
    pub current_llm_usage: Vec<LLMUsage>,
    pub network_activity: Vec<NetworkActivity>,
    pub typing_speed: TypingMetrics,
    pub recent_questions: Vec<Question>,
}

#[derive(Debug, Clone)]
pub struct ApiCall {
    pub timestamp: String,
    pub endpoint: String,
    pub status: ApiCallStatus,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ApiCallStatus {
    Pending,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub struct NetworkActivity {
    pub timestamp: String,
    pub activity_type: NetworkActivityType,
    pub endpoint: String,
    pub status: NetworkStatus,
    pub latency_ms: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[derive(Debug, Clone)]
pub enum NetworkActivityType {
    ApiCall,
    DataSync,
    FileUpload,
    FileDownload,
}

#[derive(Debug, Clone)]
pub enum NetworkStatus {
    InProgress,
    Success,
    Failed,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct TypingMetrics {
    pub current_wpm: f64,
    pub average_wpm: f64,
    pub total_characters: u64,
    pub total_time_ms: u64,
    pub last_keystroke: Option<std::time::Instant>,
    pub keystroke_intervals: Vec<u64>, // Last 10 intervals for WPM calculation
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            current_question: None,
            current_solution: None,
            statistics: None,
            text_editor: TextEditor::default(),
            selected_tab: 0,
            list_state: ListState::default(),
            recent_questions_state: ListState::default(),
            scroll_state: ScrollbarState::default(),
            scroll_offset: 0,
            show_hints: false,
            hint_index: 0,
            feedback_text: String::new(),
            status_message: String::new(),
            is_loading: false,
            api_calls: Vec::new(),
            error_count: 0,
            success_count: 0,
            cost_analytics: None,
            user_analytics: None,
            current_llm_usage: Vec::new(),
            network_activity: Vec::new(),
            typing_speed: TypingMetrics {
                current_wpm: 0.0,
                average_wpm: 0.0,
                total_characters: 0,
                total_time_ms: 0,
                last_keystroke: None,
                keystroke_intervals: Vec::new(),
            },
            recent_questions: Vec::new(),
        }
    }
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn draw(&mut self, app: &App) -> Result<()> {
        let ui_ref = self as *const Self;
        self.terminal.draw(move |f| {
            let ui = unsafe { &*ui_ref };

            // Render gradient background first
            let gradient = EvaGradientWidget::new();
            gradient.render(f, f.size());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header
                    Constraint::Min(0),    // Main content
                    Constraint::Length(3), // Footer
                ])
                .split(f.size());

            ui.render_header(f, chunks[0], app);
            ui.render_main_content(f, chunks[1], app);
            ui.render_footer(f, chunks[2], app);
        })?;
        Ok(())
    }

    fn render_header(&self, f: &mut Frame, area: Rect, app: &App) {
        use widgets::*;

        let title = match app.state {
            AppState::Home => "CENTRAL DOGMA - DSA LEARNING COMMAND CENTER",
            AppState::AllQuestions => "ALGORITHM DATABASE - PROBLEM ARCHIVE",
            AppState::QuestionView => "PROBLEM ANALYSIS - ALGORITHM BRIEFING",
            AppState::CodeEditor => "CODE SYNTHESIS INTERFACE - SOLUTION MODE",
            AppState::Results => "EXECUTION REPORT - SOLUTION ANALYSIS",
            AppState::Statistics => "MAGI ANALYTICS - LEARNING PERFORMANCE",
            AppState::Settings => "SYSTEM CONFIGURATION - LEARNING PARAMETERS",
            AppState::Help => "TECHNICAL MANUAL - OPERATION GUIDANCE",
        };

        let header_text = format!("{} {} {}", EvaSymbols::HEXAGON, title, EvaSymbols::HEXAGON);
        let header = Paragraph::new(header_text)
            .style(EvaStyles::text_highlight())
            .alignment(Alignment::Center)
            .block(EvaBorders::header("NERV HEADQUARTERS"));

        f.render_widget(header, area);
    }

    fn render_main_content(&self, f: &mut Frame, area: Rect, app: &App) {
        match app.state {
            AppState::Home => self.render_home(f, area, app),
            AppState::AllQuestions => self.render_all_questions(f, area, app),
            AppState::QuestionView => self.render_question_view(f, area, app),
            AppState::CodeEditor => self.render_code_editor(f, area, app),
            AppState::Results => self.render_results(f, area, app),
            AppState::Statistics => self.render_statistics(f, area, app),
            AppState::Settings => self.render_settings(f, area, app),
            AppState::Help => self.render_help(f, area, app),
        }
    }

    fn render_home(&self, f: &mut Frame, area: Rect, app: &App) {
        use widgets::*;

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Loading indicator (if active)
                Constraint::Min(0),    // Main content
            ])
            .split(area);

        let content_area = if app.data.is_loading {
            // Show loading indicator at the top
            let loading_widget =
                LoadingWidget::new(app.data.status_message.clone(), app.data.is_loading);
            loading_widget.render(f, main_chunks[0]);
            main_chunks[1]
        } else {
            area
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area);

        // Left panel - Quick actions
        let actions = vec![
            format!("{} GENERATE NEW ALGORITHM CHALLENGE", EvaSymbols::TRIANGLE),
            format!("{} ACCESS PROBLEM DATABASE", EvaSymbols::SQUARE),
            format!("{} REVIEW LEARNING ANALYTICS", EvaSymbols::DIAMOND),
            format!("{} SYSTEM CONFIGURATION", EvaSymbols::HEXAGON),
            format!("{} TECHNICAL DOCUMENTATION", EvaSymbols::OPERATIONAL),
            format!("{} TERMINATE SESSION", EvaSymbols::CRITICAL),
        ];

        let action_items: Vec<ListItem> = actions
            .iter()
            .enumerate()
            .map(|(i, action)| {
                let style = if i == app.data.selected_tab {
                    EvaStyles::selected()
                } else {
                    EvaStyles::text_primary()
                };
                ListItem::new(action.as_str()).style(style)
            })
            .collect();

        let actions_list = List::new(action_items)
            .block(EvaBorders::panel().title(EvaFormat::title("LEARNING OPERATIONS")))
            .highlight_style(EvaStyles::selected());

        f.render_stateful_widget(actions_list, chunks[0], &mut app.data.list_state.clone());

        // Right panel - Progress overview only
        if let Some(stats) = &app.data.statistics {
            let eva_progress = EvaProgressWidget::new(stats)
                .with_cost_analytics(app.data.cost_analytics.as_ref())
                .with_details(false);
            eva_progress.render(f, chunks[1]);
        } else {
            let loading_widget = EvaLoadingWidget::new("LOADING MAGI STATISTICS".to_string(), true)
                .with_operation_type(EvaOperationType::MagiCalculation);
            loading_widget.render(f, chunks[1]);
        }
    }

    fn render_recent_questions(&self, f: &mut Frame, area: Rect, app: &App) {
        let recent_questions = &app.data.recent_questions;

        if recent_questions.is_empty() {
            let no_questions = Paragraph::new(
                "No recent questions found.\nPress 'g' to generate your first question!",
            )
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("📚 Recent Questions")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            );
            f.render_widget(no_questions, area);
        } else {
            let question_items: Vec<ListItem> = recent_questions
                .iter()
                .enumerate()
                .map(|(i, question)| {
                    let difficulty_color = match question.difficulty {
                        crate::models::Difficulty::Easy => Color::Green,
                        crate::models::Difficulty::Medium => Color::Yellow,
                        crate::models::Difficulty::Hard => Color::Red,
                    };

                    let content = format!(
                        "{}. {} [{}]",
                        i + 1,
                        question.title,
                        format!("{:?}", question.difficulty)
                    );

                    ListItem::new(content).style(Style::default().fg(difficulty_color))
                })
                .collect();

            let questions_list = List::new(question_items)
                .block(
                    Block::default()
                        .title("📚 Recent Questions (Enter to select)")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue)),
                )
                .highlight_style(Style::default().bg(Color::DarkGray));

            f.render_stateful_widget(
                questions_list,
                area,
                &mut app.data.recent_questions_state.clone(),
            );
        }
    }

    fn render_all_questions(&self, f: &mut Frame, area: Rect, app: &App) {
        let all_questions = &app.data.recent_questions; // For now, use recent questions as all questions

        if all_questions.is_empty() {
            let no_questions_text = format!(
                "{} NO PROBLEMS DETECTED\n\n{} INITIATE PROBLEM GENERATION PROTOCOL\n{} PRESS 'G' TO BEGIN ALGORITHM SYNTHESIS",
                EvaSymbols::OPERATIONAL,
                EvaSymbols::ARROW_RIGHT,
                EvaSymbols::TRIANGLE
            );
            let no_questions = Paragraph::new(no_questions_text)
                .style(EvaStyles::text_secondary())
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .block(EvaBorders::panel().title(EvaFormat::title("ALGORITHM DATABASE")));
            f.render_widget(no_questions, area);
        } else {
            let question_items: Vec<ListItem> = all_questions
                .iter()
                .enumerate()
                .map(|(i, question)| {
                    let (difficulty_color, difficulty_symbol) = match question.difficulty {
                        crate::models::Difficulty::Easy => {
                            (EvaStyles::text_success(), EvaSymbols::OPERATIONAL)
                        }
                        crate::models::Difficulty::Medium => {
                            (EvaStyles::text_warning(), EvaSymbols::WARNING)
                        }
                        crate::models::Difficulty::Hard => {
                            (EvaStyles::text_critical(), EvaSymbols::CRITICAL)
                        }
                    };

                    let content = format!(
                        "{} PROBLEM-{:02}: {} | COMPLEXITY: {:?} | CATEGORY: {:?}",
                        difficulty_symbol,
                        i + 1,
                        question.title,
                        question.difficulty,
                        question.topic
                    );

                    ListItem::new(content).style(difficulty_color)
                })
                .collect();

            let questions_list = List::new(question_items)
                .block(
                    EvaBorders::panel()
                        .title(EvaFormat::title("ALGORITHM ARCHIVE - SELECT PROBLEM")),
                )
                .highlight_style(EvaStyles::selected());

            f.render_stateful_widget(
                questions_list,
                area,
                &mut app.data.recent_questions_state.clone(),
            );
        }
    }

    fn render_progress_overview(&self, f: &mut Frame, area: Rect, stats: &Statistics, app: &App) {
        use widgets::*;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Progress overview widget
                Constraint::Length(5), // Network activity widget
                Constraint::Min(0),    // API debug widget
            ])
            .split(area);

        // Progress overview widget
        let progress_widget = ProgressOverviewWidget::new(stats)
            .with_cost_analytics(app.data.cost_analytics.as_ref())
            .with_details(false);
        progress_widget.render(f, chunks[0]);

        // Network activity widget
        let network_widget =
            NetworkActivityWidget::new(&app.data.network_activity).with_details(false);
        network_widget.render(f, chunks[1]);

        // API debug widget
        let api_widget =
            ApiDebugWidget::new(&app.data.api_calls, app.data.is_loading).with_details(true);
        api_widget.render(f, chunks[2]);
    }

    fn render_question_view(&self, f: &mut Frame, area: Rect, app: &App) {
        if let Some(question) = &app.data.current_question {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Min(10),   // Description
                    Constraint::Length(5), // Test cases
                ])
                .split(area);

            // Question title
            let difficulty_symbol = match question.difficulty {
                crate::models::Difficulty::Easy => EvaSymbols::OPERATIONAL,
                crate::models::Difficulty::Medium => EvaSymbols::WARNING,
                crate::models::Difficulty::Hard => EvaSymbols::CRITICAL,
            };

            let title_text = format!(
                "{} PROBLEM DESIGNATION: {} | COMPLEXITY LEVEL: {:?} | ALGORITHM TYPE: {:?}",
                difficulty_symbol, question.title, question.difficulty, question.topic
            );
            let title = Paragraph::new(title_text)
                .style(EvaStyles::text_highlight())
                .alignment(Alignment::Center)
                .block(EvaBorders::header("ALGORITHM BRIEFING"));
            f.render_widget(title, chunks[0]);

            // Question description
            let description_text = format!(
                "{} PROBLEM ANALYSIS:\n\n{}",
                EvaSymbols::HEXAGON,
                question.description
            );
            let description = Paragraph::new(description_text)
                .style(EvaStyles::text_primary())
                .wrap(Wrap { trim: true })
                .scroll((app.data.scroll_offset as u16, 0))
                .block(EvaBorders::panel().title(EvaFormat::title("TECHNICAL SPECIFICATION")));
            f.render_widget(description, chunks[1]);

            // Test cases
            let test_cases_text = format!(
                "{} TEST SCENARIOS:\n\n{}",
                EvaSymbols::TRIANGLE,
                question
                    .test_cases
                    .iter()
                    .enumerate()
                    .map(|(i, tc)| {
                        format!(
                            "{} TEST CASE {}: INPUT: {} | EXPECTED: {}",
                            EvaSymbols::ARROW_RIGHT,
                            i + 1,
                            tc.input,
                            tc.expected_output
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            let test_cases = Paragraph::new(test_cases_text)
                .style(EvaStyles::sync_rate())
                .wrap(Wrap { trim: true })
                .block(EvaBorders::operational().title(EvaFormat::title("VALIDATION PROTOCOLS")));
            f.render_widget(test_cases, chunks[2]);

            // Hints panel (if enabled)
            if app.data.show_hints && !question.hints.is_empty() {
                let hint_area = Rect {
                    x: area.width / 4,
                    y: area.height / 4,
                    width: area.width / 2,
                    height: area.height / 2,
                };

                f.render_widget(Clear, hint_area);

                let hint_text = if app.data.hint_index < question.hints.len() {
                    &question.hints[app.data.hint_index]
                } else {
                    "No more hints available"
                };

                let hint_popup = Paragraph::new(hint_text)
                    .style(Style::default().fg(Color::Yellow))
                    .wrap(Wrap { trim: true })
                    .block(
                        Block::default()
                            .title(format!(
                                "Hint {} of {}",
                                app.data.hint_index + 1,
                                question.hints.len()
                            ))
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow)),
                    );
                f.render_widget(hint_popup, hint_area);
            }
        } else {
            let no_question =
                Paragraph::new("No question loaded. Press 'g' to generate a new question.")
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Red)),
                    );
            f.render_widget(no_question, area);
        }
    }

    fn render_code_editor(&self, f: &mut Frame, area: Rect, app: &App) {
        use widgets::*;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Stats bar
                Constraint::Min(10),   // Code editor
                Constraint::Length(3), // Status/Loading bar
            ])
            .split(area);

        // Split stats bar into two widgets
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[0]);

        // EVA Unit status (compact display)
        let eva_status_text = format!(
            "{} UNIT STATUS: {} VICTORIES | {} DEFEATS | {} OPERATIONS\n{} NEURAL LINK: {} ACTIVE CONNECTIONS",
            EvaSymbols::OPERATIONAL,
            app.data.success_count,
            app.data.error_count,
            app.data.api_calls.len(),
            EvaSymbols::HEXAGON,
            app.data.network_activity.len()
        );

        let eva_status = Paragraph::new(eva_status_text)
            .style(EvaStyles::text_secondary())
            .block(EvaBorders::panel().title(EvaFormat::title("EVA UNIT-01")));
        f.render_widget(eva_status, stats_chunks[0]);

        // Entry Plug Interface (typing speed)
        let eva_typing = EvaTypingWidget::new(&app.data.typing_speed);
        eva_typing.render(f, stats_chunks[1]);

        // Enhanced code editor widget
        let code_editor_widget =
            CodeEditorWidget::new(&app.data.text_editor).with_language(CodeLanguage::Rust);
        code_editor_widget.render(f, chunks[1]);

        // Loading/Status widget
        let status_message = if app.data.is_loading {
            "COMPILING EVA UNIT COMBAT PROTOCOLS".to_string()
        } else {
            "ENTRY PLUG SYNCHRONIZED - AWAITING PILOT COMMANDS".to_string()
        };

        let operation_type = if app.data.is_loading {
            EvaOperationType::EvaActivation
        } else {
            EvaOperationType::SyncTest
        };

        let eva_loading = EvaLoadingWidget::new(status_message, app.data.is_loading)
            .with_operation_type(operation_type);
        eva_loading.render(f, chunks[2]);
    }

    fn render_results(&self, f: &mut Frame, area: Rect, app: &App) {
        if let Some(solution) = &app.data.current_solution {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Status
                    Constraint::Min(5),    // Test results
                    Constraint::Min(5),    // Feedback
                ])
                .split(area);

            // Solution status
            let status_color = match solution.status {
                SolutionStatus::Accepted => Color::Green,
                SolutionStatus::WrongAnswer => Color::Red,
                SolutionStatus::CompilationError => Color::Magenta,
                SolutionStatus::RuntimeError => Color::Yellow,
                SolutionStatus::TimeLimitExceeded => Color::Cyan,
                SolutionStatus::Pending => Color::Gray,
            };

            let status_text = format!(
                "Status: {} | Execution Time: {}ms",
                solution.status,
                solution.execution_time.unwrap_or(0)
            );

            let status = Paragraph::new(status_text)
                .style(
                    Style::default()
                        .fg(status_color)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(status_color)),
                );
            f.render_widget(status, chunks[0]);

            // Test results
            let test_results_text = solution
                .test_results
                .iter()
                .map(|tr| {
                    let status_icon = if tr.passed { "✅" } else { "❌" };
                    format!(
                        "{} Test {}: {}",
                        status_icon,
                        tr.test_case_index + 1,
                        if tr.passed {
                            "PASSED".to_string()
                        } else {
                            tr.error_message
                                .as_ref()
                                .unwrap_or(&"FAILED".to_string())
                                .clone()
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            let test_results = Paragraph::new(test_results_text)
                .style(Style::default().fg(Color::White))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Test Results")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue)),
                );
            f.render_widget(test_results, chunks[1]);

            // Feedback
            let feedback_text = if !app.data.feedback_text.is_empty() {
                &app.data.feedback_text
            } else {
                "Generating feedback..."
            };

            let feedback = Paragraph::new(feedback_text)
                .style(Style::default().fg(Color::Cyan))
                .wrap(Wrap { trim: true })
                .scroll((app.data.scroll_offset as u16, 0))
                .block(
                    Block::default()
                        .title("AI Feedback")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green)),
                );
            f.render_widget(feedback, chunks[2]);
        } else {
            let no_results = Paragraph::new("No results to display.")
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Red)),
                );
            f.render_widget(no_results, area);
        }
    }

    fn render_statistics(&self, f: &mut Frame, area: Rect, app: &App) {
        if let Some(stats) = &app.data.statistics {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(6), // Overview
                    Constraint::Min(0),    // Detailed stats
                ])
                .split(area);

            // Overview
            let overview_text = format!(
                "📊 Total Questions: {}\n✅ Solved: {}\n📈 Success Rate: {:.1}%\n🔥 Current Streak: {}\n⏱️ Average Execution Time: {:.1}ms",
                stats.total_questions,
                stats.accepted_solutions,
                stats.success_rate,
                stats.current_streak,
                stats.avg_execution_time
            );

            let overview = Paragraph::new(overview_text)
                .style(Style::default().fg(Color::Cyan))
                .block(
                    Block::default()
                        .title("Overview")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Green)),
                );
            f.render_widget(overview, chunks[0]);

            // Detailed breakdown
            let detail_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            // Topic distribution
            let topic_text = stats
                .topic_distribution
                .iter()
                .map(|(topic, count)| format!("{}: {}", topic, count))
                .collect::<Vec<_>>()
                .join("\n");

            let topics = Paragraph::new(topic_text)
                .style(Style::default().fg(Color::Yellow))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Topics")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Blue)),
                );
            f.render_widget(topics, detail_chunks[0]);

            // Difficulty distribution
            let difficulty_text = stats
                .difficulty_distribution
                .iter()
                .map(|(difficulty, count)| format!("{}: {}", difficulty, count))
                .collect::<Vec<_>>()
                .join("\n");

            let difficulties = Paragraph::new(difficulty_text)
                .style(Style::default().fg(Color::Magenta))
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .title("Difficulties")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Red)),
                );
            f.render_widget(difficulties, detail_chunks[1]);
        } else {
            let loading = Paragraph::new("Loading statistics...")
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow)),
                );
            f.render_widget(loading, area);
        }
    }

    fn render_settings(&self, f: &mut Frame, area: Rect, _app: &App) {
        let settings_text = "⚙️ Settings\n\n🔑 Gemini API Key: Set via GEMINI_API_KEY environment variable\n📁 Data Directory: ~/.local/share/dsa_learning_assistant/\n\n🎨 Theme: Dark (default)\n🔊 Sound: Disabled\n📊 Auto-save: Enabled";

        let settings = Paragraph::new(settings_text)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Settings")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            );
        f.render_widget(settings, area);
    }

    fn render_help(&self, f: &mut Frame, area: Rect, app: &App) {
        let help_text = r#"🎯 DSA Learning Assistant - Help

📋 Navigation:
• Tab/Shift+Tab: Navigate between sections
• ↑/↓: Scroll up/down
• Enter: Select/Confirm
• Esc: Go back/Cancel
• q: Quit application

🏠 Home Screen:
• g: Generate new question
• r: View recent questions
• s: View statistics
• h: Show this help

📝 Question View:
• c: Start coding
• h: Show/hide hints
• n/p: Next/previous hint

💻 Code Editor:
• Ctrl+S: Submit solution
• Ctrl+H: Get hint for current code
• Ctrl+C: Clear editor
• Esc: Go back to question

📊 Results:
• f: Get detailed feedback
• r: Retry question
• n: Next question

🔧 Features:
• Adaptive question generation based on your progress
• Real-time code compilation and testing
• AI-powered feedback and hints
• Progress tracking and statistics
• Persistent learning history

💡 Tips:
• Set GEMINI_API_KEY environment variable before starting
• Focus on understanding concepts, not just solving
• Review feedback to improve your coding skills
• Practice regularly to maintain your streak!"#;

        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .scroll((app.data.scroll_offset as u16, 0))
            .block(
                Block::default()
                    .title("Help")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            );
        f.render_widget(help, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect, app: &App) {
        use widgets::*;

        let footer_text = match app.state {
            AppState::Home => format!("{} COMMAND: ↑↓ NAVIGATE | ENTER: EXECUTE | G: GENERATE PROBLEM | R: ARCHIVE | S: ANALYTICS | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::AllQuestions => format!("{} ARCHIVE: ↑↓ BROWSE | ENTER: SELECT PROBLEM | ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::QuestionView => format!("{} BRIEFING: C: START CODING | H: HINTS | ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::CodeEditor => format!("{} CODING: CTRL+S: SUBMIT SOLUTION | CTRL+H: REQUEST HINT | ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::Results => format!("{} REPORT: F: DETAILED ANALYSIS | R: RETRY PROBLEM | N: NEXT PROBLEM | ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::Statistics => format!("{} ANALYTICS: ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::Settings => format!("{} CONFIG: ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
            AppState::Help => format!("{} MANUAL: ↑↓ SCROLL | ESC: RETURN | Q: SHUTDOWN", EvaSymbols::ARROW_RIGHT),
        };

        let footer = Paragraph::new(footer_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel().title(EvaFormat::title("PILOT INTERFACE")));

        f.render_widget(footer, area);
    }

    pub fn handle_events(&self) -> Result<Option<Event>> {
        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            if let Event::Key(key) = &event {
                if key.kind == KeyEventKind::Press {
                    return Ok(Some(event));
                }
            }
        }
        Ok(None)
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}
