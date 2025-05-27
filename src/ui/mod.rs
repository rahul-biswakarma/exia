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
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, ScrollbarState, Wrap},
    Frame, Terminal,
};
use std::io;

use widgets::{
    CornerDecorationWidget,
    DecoratedBlock,
    EvaBorders,
    EvaColors,
    EvaFormat, // EvaLoadingWidget, EvaOperationType, EvaProgressWidget removed
    EvaStyles,
    EvaSymbols, // EvaTypingWidget removed. Eva* specific items might be removed later
    HomeLayoutWidget,
    LLMCallInfo,
    LLMCallWidget,
    LLMStreamInfo,
    LLMStreamStatus,
    LoadingOperationType,
    SystemMetrics,
    TextEditor,
    Theme,
    ThemeManager,
    ThemedLoadingWidget,
    ThemedProgressWidget,
    ThemedTypingIndicatorWidget, // Added new types
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
    LLMCallView,
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
    pub compilation_error: Option<String>,
    pub is_loading: bool,
    pub is_llm_loading: bool,
    pub api_calls: Vec<ApiCall>,
    pub error_count: usize,
    pub success_count: usize,
    pub cost_analytics: Option<CostAnalytics>,
    pub user_analytics: Option<UserAnalytics>,
    pub current_llm_usage: Vec<LLMUsage>,
    pub network_activity: Vec<NetworkActivity>,
    pub typing_speed: TypingMetrics,
    pub recent_questions: Vec<Question>,
    pub theme_manager: ThemeManager,
    pub current_llm_call: Option<LLMCallInfo>,
    pub previous_state: Option<AppState>,
    // Real-time system metrics
    pub current_system_metrics: Option<SystemMetrics>,
    pub cpu_history: std::collections::VecDeque<(f64, f64)>,
    pub ram_history: std::collections::VecDeque<(f64, f64)>,
    pub last_metrics_update: Option<std::time::Instant>,
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
            compilation_error: None,
            is_loading: false,
            is_llm_loading: false,
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
            theme_manager: ThemeManager::new(),
            current_llm_call: None,
            previous_state: None,
            // Initialize system metrics
            current_system_metrics: None,
            cpu_history: std::collections::VecDeque::with_capacity(60),
            ram_history: std::collections::VecDeque::with_capacity(60),
            last_metrics_update: None,
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

    pub fn theme<'a>(&self, app: &'a App) -> &'a dyn Theme {
        app.data.theme_manager.current_theme()
    }

    pub fn update_system_metrics(&self, app: &mut App) {
        let now = std::time::Instant::now();

        // Update metrics every second
        if let Some(last_update) = app.data.last_metrics_update {
            if now.duration_since(last_update).as_secs() < 1 {
                return;
            }
        }

        // Get real system metrics
        let (cpu_usage, ram_usage, ram_total) = self.get_real_system_metrics();

        // Create new metrics
        let metrics = SystemMetrics {
            cpu_usage,
            ram_usage,
            ram_total,
            timestamp: now,
        };

        // Update current metrics
        app.data.current_system_metrics = Some(metrics);

        // Add to history with time in seconds since start
        let time_seconds = app.data.cpu_history.len() as f64;

        // Add to CPU history
        app.data.cpu_history.push_back((time_seconds, cpu_usage));
        if app.data.cpu_history.len() > 60 {
            app.data.cpu_history.pop_front();
        }

        // Add to RAM history (convert to percentage)
        let ram_percentage = if ram_total > 0.0 {
            (ram_usage / ram_total) * 100.0
        } else {
            0.0
        };
        app.data
            .ram_history
            .push_back((time_seconds, ram_percentage));
        if app.data.ram_history.len() > 60 {
            app.data.ram_history.pop_front();
        }

        app.data.last_metrics_update = Some(now);
    }

    fn get_real_system_metrics(&self) -> (f64, f64, f64) {
        // Try to get real system metrics
        // For cross-platform compatibility, we'll use a simple approach

        #[cfg(target_os = "macos")]
        {
            self.get_macos_metrics()
        }

        #[cfg(target_os = "linux")]
        {
            self.get_linux_metrics()
        }

        #[cfg(target_os = "windows")]
        {
            self.get_windows_metrics()
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            // Fallback to simulated metrics for unsupported platforms
            self.get_simulated_metrics()
        }
    }

    #[cfg(target_os = "macos")]
    fn get_macos_metrics(&self) -> (f64, f64, f64) {
        use std::process::Command;

        // Get CPU usage using top command
        let cpu_usage = Command::new("top")
            .args(&["-l", "1", "-n", "0"])
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse CPU usage from top output
                for line in output_str.lines() {
                    if line.contains("CPU usage:") {
                        // Format: "CPU usage: 5.32% user, 9.44% sys, 85.23% idle"
                        // We want to calculate total usage = user + sys
                        let mut user_cpu = 0.0;
                        let mut sys_cpu = 0.0;

                        // Extract user CPU
                        if let Some(user_start) = line.find("CPU usage:") {
                            let rest = &line[user_start + 10..];
                            if let Some(user_end) = rest.find("% user") {
                                let user_str = rest[..user_end].trim();
                                user_cpu = user_str.parse().unwrap_or(0.0);
                            }
                        }

                        // Extract sys CPU
                        if let Some(sys_start) = line.find("% user, ") {
                            let rest = &line[sys_start + 8..];
                            if let Some(sys_end) = rest.find("% sys") {
                                let sys_str = rest[..sys_end].trim();
                                sys_cpu = sys_str.parse().unwrap_or(0.0);
                            }
                        }

                        return Some(user_cpu + sys_cpu);
                    }
                }
                None
            })
            .unwrap_or(0.0);

        // Get memory usage using vm_stat
        let (ram_usage, ram_total) = Command::new("vm_stat")
            .output()
            .ok()
            .and_then(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut pages_free = 0u64;
                let mut pages_active = 0u64;
                let mut pages_inactive = 0u64;
                let mut pages_speculative = 0u64;
                let mut pages_wired = 0u64;

                let mut page_size = 4096u64; // Default page size

                for line in output_str.lines() {
                    if line.contains("page size of") {
                        // Extract page size from first line: "Mach Virtual Memory Statistics: (page size of 16384 bytes)"
                        if let Some(start) = line.find("page size of ") {
                            let rest = &line[start + 13..];
                            if let Some(end) = rest.find(" bytes") {
                                let size_str = &rest[..end];
                                page_size = size_str.parse().unwrap_or(4096);
                            }
                        }
                    } else if line.contains("Pages free:") {
                        pages_free = line
                            .split_whitespace()
                            .nth(2)?
                            .trim_end_matches('.')
                            .parse()
                            .ok()?;
                    } else if line.contains("Pages active:") {
                        pages_active = line
                            .split_whitespace()
                            .nth(2)?
                            .trim_end_matches('.')
                            .parse()
                            .ok()?;
                    } else if line.contains("Pages inactive:") {
                        pages_inactive = line
                            .split_whitespace()
                            .nth(2)?
                            .trim_end_matches('.')
                            .parse()
                            .ok()?;
                    } else if line.contains("Pages speculative:") {
                        pages_speculative = line
                            .split_whitespace()
                            .nth(2)?
                            .trim_end_matches('.')
                            .parse()
                            .ok()?;
                    } else if line.contains("Pages wired down:") {
                        pages_wired = line
                            .split_whitespace()
                            .nth(3)?
                            .trim_end_matches('.')
                            .parse()
                            .ok()?;
                    }
                }
                let total_pages =
                    pages_free + pages_active + pages_inactive + pages_speculative + pages_wired;
                let used_pages = pages_active + pages_inactive + pages_speculative + pages_wired;

                let ram_total_gb = (total_pages * page_size) as f64 / (1024.0 * 1024.0 * 1024.0);
                let ram_used_gb = (used_pages * page_size) as f64 / (1024.0 * 1024.0 * 1024.0);

                Some((ram_used_gb, ram_total_gb))
            })
            .unwrap_or((2.0, 8.0)); // Fallback values

        (cpu_usage, ram_usage, ram_total)
    }

    #[cfg(target_os = "linux")]
    fn get_linux_metrics(&self) -> (f64, f64, f64) {
        use std::fs;

        // Get CPU usage from /proc/stat
        let cpu_usage = fs::read_to_string("/proc/stat")
            .ok()
            .and_then(|content| {
                let line = content.lines().next()?;
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 && parts[0] == "cpu" {
                    let user: u64 = parts[1].parse().ok()?;
                    let nice: u64 = parts[2].parse().ok()?;
                    let system: u64 = parts[3].parse().ok()?;
                    let idle: u64 = parts[4].parse().ok()?;

                    let total = user + nice + system + idle;
                    let used = user + nice + system;

                    if total > 0 {
                        Some((used as f64 / total as f64) * 100.0)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or(0.0);

        // Get memory usage from /proc/meminfo
        let (ram_usage, ram_total) = fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|content| {
                let mut mem_total = 0u64;
                let mut mem_available = 0u64;

                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        mem_total = line.split_whitespace().nth(1)?.parse().ok()?;
                    } else if line.starts_with("MemAvailable:") {
                        mem_available = line.split_whitespace().nth(1)?.parse().ok()?;
                    }
                }

                if mem_total > 0 {
                    let ram_total_gb = mem_total as f64 / (1024.0 * 1024.0);
                    let ram_used_gb = (mem_total - mem_available) as f64 / (1024.0 * 1024.0);
                    Some((ram_used_gb, ram_total_gb))
                } else {
                    None
                }
            })
            .unwrap_or((2.0, 8.0));

        (cpu_usage, ram_usage, ram_total)
    }

    #[cfg(target_os = "windows")]
    fn get_windows_metrics(&self) -> (f64, f64, f64) {
        // For Windows, we'll use a simulated approach for now
        // In a real implementation, you'd use Windows APIs
        self.get_simulated_metrics()
    }

    fn get_simulated_metrics(&self) -> (f64, f64, f64) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // Generate realistic-looking metrics
        let cpu_usage = (45.0 + 15.0 * (time * 0.1).sin() + 5.0 * (time * 0.3).cos())
            .max(0.0)
            .min(100.0);
        let ram_usage = 2.5 + 1.0 * (time * 0.05).sin();
        let ram_total = 8.0;

        (cpu_usage, ram_usage, ram_total)
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        // Update system metrics before drawing
        self.update_system_metrics(app);

        let ui_ref = self as *const Self;
        self.terminal.draw(move |f| {
            let ui = unsafe { &*ui_ref };

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
        let theme = app.data.theme_manager.current_theme();

        let title = match app.state {
            AppState::Home => "EXIA COMMAND CENTER",
            AppState::AllQuestions => "ALGORITHM DATABASE",
            AppState::QuestionView => "PROBLEM ANALYSIS",
            AppState::CodeEditor => "CODE SYNTHESIS INTERFACE",
            AppState::Results => "EXECUTION REPORT",
            AppState::Statistics => "LEARNING ANALYTICS",
            AppState::Settings => "SYSTEM CONFIGURATION",
            AppState::Help => "OPERATION GUIDANCE",
            AppState::LLMCallView => "NEURAL NETWORK PROCESSING",
        };

        let corner_symbol = theme.symbols().corner_decoration();
        let header_text = format!("{} {} {}", corner_symbol, title, corner_symbol);
        let header = Paragraph::new(header_text)
            .style(theme.styles().text_highlight())
            .alignment(Alignment::Center)
            .block(theme.borders().header_block("EXIA SYSTEM"));

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
            AppState::LLMCallView => self.render_llm_call_view(f, area, app),
        }
    }

    fn render_home(&self, f: &mut Frame, area: Rect, app: &App) {
        let theme = app.data.theme_manager.current_theme();

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Loading indicator (if active)
                Constraint::Min(0),    // Main content
            ])
            .split(area);

        let content_area = if app.data.is_loading {
            // Show loading indicator at the top
            let loading_widget = ThemedLoadingWidget::new(
                app.data.status_message.clone(),
                app.data.is_loading,
                app.data.theme_manager.current_theme(),
            );
            // Assuming ThemedLoadingWidget now implements the Widget trait directly for render
            // If it needs to be rendered as &ThemedLoadingWidget, adjust accordingly.
            f.render_widget(loading_widget, main_chunks[0]);
            main_chunks[1]
        } else {
            area
        };

        // Use the new HomeLayoutWidget if statistics are available
        if let Some(stats) = &app.data.statistics {
            // Only show LLM stream info if there's actual LLM activity
            let llm_stream_info = if app.data.is_llm_loading || app.data.current_llm_call.is_some()
            {
                // Create LLM stream info based on current activity
                app.data.current_llm_call.as_ref().map(|call_info| {
                    use crate::ui::widgets::{LLMStreamInfo, LLMStreamStatus};
                    let mut info = LLMStreamInfo::new(call_info.operation_type.clone());

                    info.status = match call_info.status {
                        crate::ui::widgets::LLMCallStatus::Initializing => {
                            LLMStreamStatus::Initializing
                        }
                        crate::ui::widgets::LLMCallStatus::InProgress => LLMStreamStatus::Streaming,
                        crate::ui::widgets::LLMCallStatus::Success => LLMStreamStatus::Complete,
                        crate::ui::widgets::LLMCallStatus::Error => LLMStreamStatus::Error,
                    };

                    info.progress = call_info.progress;
                    info.input_tokens = call_info.input_tokens;
                    info.output_tokens = call_info.output_tokens;
                    info.estimated_cost_usd = call_info.cost_usd;
                    info.model_name = call_info.model_name.clone();
                    info.error_message = call_info.error_message.clone();
                    info.start_time = call_info.start_time;
                    info.duration_ms = call_info.duration_ms;

                    // Set appropriate content based on operation type
                    info.streamed_content = match call_info.operation_type.as_str() {
                        "Question Generation" => {
                            "Generating new algorithm challenge...".to_string()
                        }
                        "Code Analysis" => "Analyzing your code implementation...".to_string(),
                        "Feedback Generation" => "Generating detailed feedback...".to_string(),
                        _ => format!("Processing {}...", call_info.operation_type),
                    };

                    info
                })
            } else {
                None
            };

            // Create Exia operations widget
            let exia_widget = self.create_exia_operations_widget(app, theme);

            // Use the new home layout with real system metrics
            let home_layout =
                HomeLayoutWidget::new(stats, &app.data.cpu_history, &app.data.ram_history)
                    .with_cost_analytics(app.data.cost_analytics.as_ref())
                    .with_llm_stream_info(llm_stream_info.as_ref())
                    .with_system_metrics(app.data.current_system_metrics.as_ref())
                    .with_exia_operations(exia_widget)
                    .with_animation_frame(0);

            home_layout.render(f, content_area);
        } else {
            // Fallback to loading widget
            let loading_widget = ThemedLoadingWidget::new(
                "Loading Statistics".to_string(),
                true,
                app.data.theme_manager.current_theme(),
            )
            .with_operation_type(LoadingOperationType::MagiCalculation);
            f.render_widget(loading_widget, content_area);
        }
    }

    fn create_exia_operations_widget<'a>(
        &self,
        app: &'a App,
        theme: &'a dyn Theme,
    ) -> Box<dyn Widget + 'a> {
        use ratatui::widgets::{List, ListItem};

        struct ExiaOperationsWidget<'a> {
            app: &'a App,
            theme: &'a dyn Theme,
        }

        impl<'a> Widget for ExiaOperationsWidget<'a> {
            fn render(&self, f: &mut Frame, area: Rect) {
                let shapes = self.theme.symbols().geometric_shapes();
                let actions = vec![
                    format!("{} Generate New Algorithm Challenge", shapes[0]),
                    format!("{} Access Problem Database", shapes[1]),
                    format!("{} Review Learning Analytics", shapes[2]),
                    format!("{} System Configuration", shapes[3]),
                    format!(
                        "{} Technical Documentation",
                        self.theme.symbols().operational()
                    ),
                    format!("{} Terminate Session", self.theme.symbols().error()),
                ];

                let action_items: Vec<ListItem> = actions
                    .iter()
                    .enumerate()
                    .map(|(i, action)| {
                        let style = if i == self.app.data.selected_tab {
                            self.theme.styles().selected()
                        } else {
                            self.theme.styles().text_primary()
                        };
                        ListItem::new(action.as_str()).style(style)
                    })
                    .collect();

                let actions_list = List::new(action_items)
                    .block(
                        self.theme
                            .borders()
                            .default_block()
                            .title("Exia Operations"),
                    )
                    .highlight_style(self.theme.styles().selected());

                let mut list_state = self.app.data.list_state.clone();
                f.render_stateful_widget(actions_list, area, &mut list_state);
            }

            fn title(&self) -> Option<&str> {
                Some("EXIA OPERATIONS")
            }
        }

        Box::new(ExiaOperationsWidget { app, theme })
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
                "→",
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
                            (EvaStyles::text_error(), EvaSymbols::CRITICAL)
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
                Constraint::Length(8), // Changed to accommodate ThemedProgressWidget's typical main display height
                Constraint::Length(5), // Network activity widget
                Constraint::Min(0),    // API debug widget
            ])
            .split(area);

        // Progress overview widget (now ThemedProgressWidget)
        let progress_widget =
            ThemedProgressWidget::new(stats, app.data.theme_manager.current_theme())
                .with_cost_analytics(app.data.cost_analytics.as_ref())
                .with_details(false); // Assuming 'false' shows the main_display
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
            // Check if we have enough width for side-by-side layout (minimum 120 columns)
            let use_side_by_side = area.width >= 120;

            if use_side_by_side {
                // Side-by-side layout: Technical specification on left, code editor on right
                let main_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3), // Title
                        Constraint::Min(0),    // Main content
                    ])
                    .split(area);

                // Question title (full width)
                let difficulty_symbol = match question.difficulty {
                    crate::models::Difficulty::Easy => self.theme(app).symbols().operational(),
                    crate::models::Difficulty::Medium => self.theme(app).symbols().warning(),
                    crate::models::Difficulty::Hard => self.theme(app).symbols().error(),
                };

                let title_text = format!(
                    "{} PROBLEM DESIGNATION: {} | COMPLEXITY LEVEL: {:?} | ALGORITHM TYPE: {:?}",
                    difficulty_symbol, question.title, question.difficulty, question.topic
                );
                let title = Paragraph::new(title_text)
                    .style(self.theme(app).styles().text_highlight())
                    .alignment(Alignment::Center)
                    .block(self.theme(app).borders().header_block("ALGORITHM BRIEFING"));
                f.render_widget(title, main_chunks[0]);

                // Split main content horizontally
                let content_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(main_chunks[1]);

                // Left side: Technical specification and test cases
                let left_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(8),    // Description
                        Constraint::Length(6), // Test cases
                    ])
                    .split(content_chunks[0]);

                // Question description
                let description_text = format!(
                    "{} PROBLEM ANALYSIS:\n\n{}",
                    self.theme(app)
                        .symbols()
                        .geometric_shapes()
                        .get(0)
                        .cloned()
                        .unwrap_or_default(),
                    question.description
                );
                let description = Paragraph::new(description_text)
                    .style(self.theme(app).styles().text_primary())
                    .wrap(Wrap { trim: true })
                    .scroll((app.data.scroll_offset as u16, 0))
                    .block(
                        self.theme(app)
                            .borders()
                            .default_block()
                            .title(self.theme(app).formats().title("TECHNICAL SPECIFICATION")),
                    );
                f.render_widget(description, left_chunks[0]);

                // Test cases
                let test_cases_text = format!(
                    "{} TEST SCENARIOS:\n\n{}",
                    self.theme(app)
                        .symbols()
                        .geometric_shapes()
                        .get(2)
                        .cloned()
                        .unwrap_or_default(),
                    question
                        .test_cases
                        .iter()
                        .enumerate()
                        .map(|(i, tc)| {
                            format!(
                                "{} TEST CASE {}: INPUT: {} | EXPECTED: {}",
                                "→",
                                i + 1,
                                tc.input,
                                tc.expected_output
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let test_cases = Paragraph::new(test_cases_text)
                    .style(self.theme(app).styles().text_secondary())
                    .wrap(Wrap { trim: true })
                    .block(
                        self.theme(app)
                            .borders()
                            .operational_block()
                            .title(self.theme(app).formats().title("VALIDATION PROTOCOLS")),
                    );
                f.render_widget(test_cases, left_chunks[1]);

                // Right side: Code editor
                let right_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(8),
                        Constraint::Min(0), // Code editor
                    ])
                    .split(content_chunks[1]);

                // Typing indicator widget (compact)
                let typing_indicator =
                    ThemedTypingIndicatorWidget::new(&app.data.typing_speed, self.theme(app));
                typing_indicator.render(f, right_chunks[0]);

                // Syntax editor widget
                use widgets::*; // Ensure CodeLanguage is in scope
                let syntax_editor_widget = SyntaxEditorWidget::new(&app.data.text_editor)
                    .with_language(CodeLanguage::Rust); // Renamed
                syntax_editor_widget.render(f, right_chunks[1]);

                // Hints panel (if enabled) for side-by-side mode
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
                // Original vertical layout for smaller screens
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
                    crate::models::Difficulty::Easy => self.theme(app).symbols().operational(), // EvaSymbols replaced
                    crate::models::Difficulty::Medium => self.theme(app).symbols().warning(), // EvaSymbols replaced
                    crate::models::Difficulty::Hard => self.theme(app).symbols().error(), // EvaSymbols replaced
                };

                let title_text = format!(
                    "{} PROBLEM DESIGNATION: {} | COMPLEXITY LEVEL: {:?} | ALGORITHM TYPE: {:?}",
                    difficulty_symbol, question.title, question.difficulty, question.topic
                );
                let title = Paragraph::new(title_text)
                    .style(self.theme(app).styles().text_highlight()) // EvaStyles replaced
                    .alignment(Alignment::Center)
                    .block(self.theme(app).borders().header_block("ALGORITHM BRIEFING")); // EvaBorders replaced
                f.render_widget(title, chunks[0]);

                // Question description
                let description_text = format!(
                    "{} PROBLEM ANALYSIS:\n\n{}",
                    self.theme(app)
                        .symbols()
                        .geometric_shapes()
                        .get(0)
                        .cloned()
                        .unwrap_or_default(), // EvaSymbols replaced
                    question.description
                );
                let description = Paragraph::new(description_text)
                    .style(self.theme(app).styles().text_primary()) // EvaStyles replaced
                    .wrap(Wrap { trim: true })
                    .scroll((app.data.scroll_offset as u16, 0))
                    .block(
                        self.theme(app)
                            .borders()
                            .default_block()
                            .title(self.theme(app).formats().title("TECHNICAL SPECIFICATION")),
                    ); // EvaBorders and EvaFormat replaced
                f.render_widget(description, chunks[1]);

                // Test cases
                let test_cases_text = format!(
                    "{} TEST SCENARIOS:\n\n{}",
                    self.theme(app)
                        .symbols()
                        .geometric_shapes()
                        .get(2)
                        .cloned()
                        .unwrap_or_default(), // EvaSymbols replaced (triangle)
                    question
                        .test_cases
                        .iter()
                        .enumerate()
                        .map(|(i, tc)| {
                            format!(
                                "{} TEST CASE {}: INPUT: {} | EXPECTED: {}",
                                "→", // Standard arrow
                                i + 1,
                                tc.input,
                                tc.expected_output
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                );

                let test_cases = Paragraph::new(test_cases_text)
                    .style(self.theme(app).styles().text_secondary()) // EvaStyles replaced (sync_rate to text_secondary)
                    .wrap(Wrap { trim: true })
                    .block(
                        self.theme(app)
                            .borders()
                            .operational_block()
                            .title(self.theme(app).formats().title("VALIDATION PROTOCOLS")), // EvaBorders and EvaFormat replaced
                    );
                f.render_widget(test_cases, chunks[2]);

                // Hints panel (if enabled) for compact mode
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
                Constraint::Length(8), // Typing indicator widget - increased height for ThemedTypingIndicatorWidget
                Constraint::Min(10),   // Code editor
                Constraint::Length(3), // Status/Loading bar
            ])
            .split(area);

        // Typing indicator widget (full width)
        let typing_indicator =
            ThemedTypingIndicatorWidget::new(&app.data.typing_speed, self.theme(app));
        typing_indicator.render(f, chunks[0]);

        // Enhanced code editor widget
        let syntax_editor_widget =
            SyntaxEditorWidget::new(&app.data.text_editor).with_language(CodeLanguage::Rust); // Renamed
        syntax_editor_widget.render(f, chunks[1]);

        // Status/Loading widget
        let (status_message, is_loading, operation_type) = if app.data.is_llm_loading {
            (
                "NEURAL NETWORK PROCESSING - GENERATING FEEDBACK".to_string(),
                true,
                LoadingOperationType::MagiCalculation,
            )
        } else if app.data.is_loading {
            (
                "COMPILING EVA UNIT COMBAT PROTOCOLS".to_string(),
                true,
                LoadingOperationType::EvaActivation,
            )
        } else if let Some(ref error) = app.data.compilation_error {
            (
                format!("COMPILATION ERROR: {}", error),
                false,
                LoadingOperationType::SyncTest,
            )
        } else {
            (
                "ENTRY PLUG SYNCHRONIZED - AWAITING PILOT COMMANDS".to_string(),
                false,
                LoadingOperationType::SyncTest,
            )
        };

        let themed_loading = ThemedLoadingWidget::new(
            status_message,
            is_loading,
            app.data.theme_manager.current_theme(),
        )
        .with_operation_type(operation_type);
        f.render_widget(themed_loading, chunks[2]);
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

    fn render_settings(&self, f: &mut Frame, area: Rect, app: &App) {
        let theme = app.data.theme_manager.current_theme();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Theme selection
                Constraint::Min(0),    // Other settings
            ])
            .split(area);

        // Theme selection
        let available_themes = app.data.theme_manager.available_themes();
        let current_theme_name = theme.name();

        let theme_items: Vec<ListItem> = available_themes
            .iter()
            .map(|theme_name| {
                let prefix = if *theme_name == current_theme_name {
                    "● "
                } else {
                    "○ "
                };
                let text = format!("{}{}", prefix, theme_name);
                let style = if *theme_name == current_theme_name {
                    theme.styles().selected()
                } else {
                    theme.styles().text_primary()
                };
                ListItem::new(text).style(style)
            })
            .collect();

        let theme_list = List::new(theme_items)
            .block(theme.borders().default_block().title("Themes (T to cycle)"))
            .highlight_style(theme.styles().selected());

        f.render_widget(theme_list, chunks[0]);

        // Other settings
        let settings_text = format!(
            "🔑 Gemini API Key: Set via GEMINI_API_KEY environment variable\n📁 Data Directory: ~/.local/share/dsa_learning_assistant/\n\n🔊 Sound: Disabled\n📊 Auto-save: Enabled\n\nControls:\n• T: Cycle themes\n• ESC: Return to main menu"
        );

        let settings = Paragraph::new(settings_text)
            .style(theme.styles().text_primary())
            .wrap(Wrap { trim: true })
            .block(theme.borders().default_block().title("Configuration"));

        f.render_widget(settings, chunks[1]);
    }

    fn render_help(&self, f: &mut Frame, area: Rect, app: &App) {
        let help_text = r#"🎯 Exia - AI-Powered Coding Assistant - Help

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

    fn render_llm_call_view(&self, f: &mut Frame, area: Rect, app: &App) {
        // Create a centered modal-like area
        let modal_area = Rect {
            x: area.width / 8,
            y: area.height / 8,
            width: (area.width * 3) / 4,
            height: (area.height * 3) / 4,
        };

        // Clear the background
        f.render_widget(Clear, modal_area);

        // Render the LLM call widget in full detail
        if let Some(call_info) = &app.data.current_llm_call {
            let llm_widget = LLMCallWidget::new(Some(call_info))
                .with_theme(app.data.theme_manager.current_theme())
                .with_details(true);
            llm_widget.render(f, modal_area);
        } else {
            // Fallback if no call info (shouldn't happen)
            let fallback = Paragraph::new("No LLM operation in progress")
                .style(EvaStyles::text_secondary())
                .alignment(Alignment::Center)
                .block(EvaBorders::panel().title(EvaFormat::title("NEURAL NETWORK INTERFACE")));
            f.render_widget(fallback, modal_area);
        }
    }

    fn render_footer(&self, f: &mut Frame, area: Rect, app: &App) {
        let theme = app.data.theme_manager.current_theme();

        let footer_text = match app.state {
            AppState::Home => {
                "↑↓ Navigate | Enter: Execute | G: Generate | R: Archive | S: Analytics | Q: Quit"
            }
            AppState::AllQuestions => "↑↓ Browse | Enter: Select | ESC: Return | Q: Quit",
            AppState::QuestionView => {
                // Check if we have enough width for side-by-side layout
                if area.width >= 120 {
                    "Type in Editor | Ctrl+S: Submit | C: Full Editor | H: Hints | ESC: Return | Q: Quit"
                } else {
                    "C: Start Coding | H: Hints | ESC: Return | Q: Quit"
                }
            }
            AppState::CodeEditor => "Ctrl+S: Submit | Ctrl+H: Hint | ESC: Return | Q: Quit",
            AppState::Results => "F: Feedback | R: Retry | N: Next | ESC: Return | Q: Quit",
            AppState::Statistics => "ESC: Return | Q: Quit",
            AppState::Settings => "T: Cycle Theme | ESC: Return | Q: Quit",
            AppState::Help => "↑↓ Scroll | ESC: Return | Q: Quit",
            AppState::LLMCallView => "Processing... Please wait | ESC: Cancel | Q: Quit",
        };

        let footer = Paragraph::new(footer_text)
            .style(theme.styles().text_secondary())
            .alignment(Alignment::Center)
            .block(theme.borders().default_block().title("Controls"));

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
