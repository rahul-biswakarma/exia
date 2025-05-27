use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme, Widget};
use crate::models::LLMUsage;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct LLMStreamInfo {
    pub operation_type: String,
    pub status: LLMStreamStatus,
    pub progress: f64, // 0.0 to 1.0
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub estimated_cost_usd: Option<f64>,
    pub model_name: Option<String>,
    pub error_message: Option<String>,
    pub start_time: Instant,
    pub duration_ms: Option<u64>,
    pub streamed_content: String,
    pub streaming_position: usize,
    pub last_update: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LLMStreamStatus {
    Initializing,
    Connecting,
    Streaming,
    Processing,
    Complete,
    Error,
}

pub struct LLMInfoWidget<'a> {
    stream_info: Option<&'a LLMStreamInfo>,
    theme: Option<&'a dyn Theme>,
    show_detailed: bool,
    animation_frame: usize,
}

impl<'a> LLMInfoWidget<'a> {
    pub fn new(stream_info: Option<&'a LLMStreamInfo>) -> Self {
        Self {
            stream_info,
            theme: None,
            show_detailed: true,
            animation_frame: 0,
        }
    }

    pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_details(mut self, show_detailed: bool) -> Self {
        self.show_detailed = show_detailed;
        self
    }

    pub fn with_animation_frame(mut self, frame: usize) -> Self {
        self.animation_frame = frame;
        self
    }

    fn get_status_symbol(&self, status: &LLMStreamStatus) -> &'static str {
        match status {
            LLMStreamStatus::Initializing => EvaSymbols::HEXAGON,
            LLMStreamStatus::Connecting => EvaSymbols::OPERATIONAL,
            LLMStreamStatus::Streaming => EvaSymbols::SYNC,
            LLMStreamStatus::Processing => EvaSymbols::DIAMOND,
            LLMStreamStatus::Complete => EvaSymbols::OPERATIONAL,
            LLMStreamStatus::Error => EvaSymbols::CRITICAL,
        }
    }

    fn get_status_color(&self, status: &LLMStreamStatus) -> Color {
        match status {
            LLMStreamStatus::Initializing => Color::Yellow,
            LLMStreamStatus::Connecting => EvaColors::ORANGE,
            LLMStreamStatus::Streaming => Color::Cyan,
            LLMStreamStatus::Processing => Color::Magenta,
            LLMStreamStatus::Complete => EvaColors::STATUS_NORMAL,
            LLMStreamStatus::Error => EvaColors::STATUS_CRITICAL,
        }
    }

    fn get_streaming_animation(&self) -> &'static str {
        let frames = [
            "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▂",
        ];
        frames[self.animation_frame % frames.len()]
    }

    fn get_neural_pattern(&self) -> String {
        let patterns = [
            "◢◣◤◥",
            "◤◥◢◣",
            "◥◢◣◤",
            "◣◤◥◢",
            "▲▼◆◇",
            "◆◇▲▼",
            "▼◆◇▲",
            "◇▲▼◆",
        ];
        patterns[self.animation_frame % patterns.len()].to_string()
    }

    fn render_streaming_text(&self, content: &str, max_chars: usize) -> String {
        if content.is_empty() {
            return "AWAITING NEURAL RESPONSE...".to_string();
        }

        let visible_chars = (max_chars).min(content.len());
        let mut displayed = content.chars().take(visible_chars).collect::<String>();

        // Add streaming cursor effect
        if visible_chars < content.len() {
            let cursor_frames = ["▌", "▐", "█", "▐"];
            let cursor = cursor_frames[self.animation_frame % cursor_frames.len()];
            displayed.push_str(cursor);
        }

        displayed
    }

    fn render_detailed_view(&self, f: &mut Frame, area: Rect, info: &LLMStreamInfo) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Progress bar
                Constraint::Length(5), // Token and cost info
                Constraint::Min(0),    // Streaming content
            ])
            .split(area);

        // Header with operation type and neural pattern
        let status_symbol = self.get_status_symbol(&info.status);
        let neural_pattern = self.get_neural_pattern();

        let header_text = format!(
            "{} {} LLM NEURAL INTERFACE {} | MODEL: {} | {}",
            neural_pattern,
            status_symbol,
            neural_pattern,
            info.operation_type.to_uppercase(),
            info.model_name.as_deref().unwrap_or("UNKNOWN")
        );

        let header_style = Style::default()
            .fg(self.get_status_color(&info.status))
            .add_modifier(Modifier::BOLD);

        let header = Paragraph::new(header_text)
            .style(header_style)
            .alignment(Alignment::Center)
            .block(EvaBorders::header("NEURAL NETWORK INTERFACE"));

        f.render_widget(header, chunks[0]);

        // Progress bar with streaming animation
        let progress_label = match info.status {
            LLMStreamStatus::Initializing => "INITIALIZING NEURAL PATHWAYS",
            LLMStreamStatus::Connecting => "ESTABLISHING NEURAL LINK",
            LLMStreamStatus::Streaming => {
                &format!("STREAMING DATA {}", self.get_streaming_animation())
            }
            LLMStreamStatus::Processing => "PROCESSING NEURAL PATTERNS",
            LLMStreamStatus::Complete => "NEURAL PROCESSING COMPLETE",
            LLMStreamStatus::Error => "NEURAL LINK ERROR",
        };

        let progress_color = self.get_status_color(&info.status);

        let progress_gauge = Gauge::default()
            .block(EvaBorders::operational().title(EvaFormat::title("PROCESSING STATUS")))
            .gauge_style(Style::default().fg(progress_color))
            .percent((info.progress * 100.0) as u16)
            .label(progress_label);

        f.render_widget(progress_gauge, chunks[1]);

        // Token and cost information
        self.render_metrics_panel(f, chunks[2], info);

        // Streaming content display
        self.render_streaming_content(f, chunks[3], info);
    }

    fn render_metrics_panel(&self, f: &mut Frame, area: Rect, info: &LLMStreamInfo) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // Input tokens
                Constraint::Percentage(25), // Output tokens
                Constraint::Percentage(25), // Cost
                Constraint::Percentage(25), // Duration
            ])
            .split(area);

        // Input tokens
        let input_text = format!(
            "{} INPUT\n{}",
            EvaSymbols::TRIANGLE,
            info.input_tokens
                .map(|t| format!("{} TOKENS", t))
                .unwrap_or_else(|| "PENDING".to_string())
        );
        let input_widget = Paragraph::new(input_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(input_widget, chunks[0]);

        // Output tokens
        let output_text = format!(
            "{} OUTPUT\n{}",
            EvaSymbols::TRIANGLE,
            info.output_tokens
                .map(|t| format!("{} TOKENS", t))
                .unwrap_or_else(|| "STREAMING".to_string())
        );
        let output_widget = Paragraph::new(output_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(output_widget, chunks[1]);

        // Cost information
        let cost_text = format!(
            "{} COST\n{}",
            EvaSymbols::HEXAGON,
            info.estimated_cost_usd
                .map(|c| format!("${:.4} USD", c))
                .unwrap_or_else(|| "CALCULATING".to_string())
        );
        let cost_widget = Paragraph::new(cost_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(cost_widget, chunks[2]);

        // Duration
        let elapsed = info.start_time.elapsed().as_millis();
        let duration_text = format!("{} TIME\n{}ms", EvaSymbols::CLOCK, elapsed);
        let duration_widget = Paragraph::new(duration_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(duration_widget, chunks[3]);
    }

    fn render_streaming_content(&self, f: &mut Frame, area: Rect, info: &LLMStreamInfo) {
        let max_chars = (area.width as usize * area.height as usize).saturating_sub(20);
        let displayed_content = self.render_streaming_text(&info.streamed_content, max_chars);

        let content_style = match info.status {
            LLMStreamStatus::Streaming => Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
            LLMStreamStatus::Complete => Style::default().fg(EvaColors::STATUS_NORMAL),
            LLMStreamStatus::Error => Style::default().fg(EvaColors::STATUS_CRITICAL),
            _ => Style::default().fg(Color::Gray),
        };

        let content_widget = Paragraph::new(displayed_content)
            .style(content_style)
            .wrap(Wrap { trim: true })
            .block(EvaBorders::operational().title(EvaFormat::title("NEURAL STREAM OUTPUT")));

        f.render_widget(content_widget, area);
    }

    fn render_compact_view(&self, f: &mut Frame, area: Rect, info: &LLMStreamInfo) {
        let status_symbol = self.get_status_symbol(&info.status);
        let progress_bar = EvaFormat::progress_bar(info.progress, 15);

        let compact_text = format!(
            "{} {} | {} | {:.1}%\n{}",
            status_symbol,
            info.operation_type.to_uppercase(),
            info.model_name.as_deref().unwrap_or("UNKNOWN"),
            info.progress * 100.0,
            progress_bar
        );

        let compact_widget = Paragraph::new(compact_text)
            .style(Style::default().fg(self.get_status_color(&info.status)))
            .block(EvaBorders::panel().title(EvaFormat::title("LLM STATUS")));

        f.render_widget(compact_widget, area);
    }
}

impl<'a> Widget for LLMInfoWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        if let Some(info) = self.stream_info {
            if self.show_detailed {
                self.render_detailed_view(f, area, info);
            } else {
                self.render_compact_view(f, area, info);
            }
        } else {
            // Render empty state with more visible styling
            let empty_text = format!(
                "{} NEURAL INTERFACE STANDBY\n\n{} AWAITING LLM OPERATIONS...\n\n{} STATUS: IDLE\n{} READY FOR NEURAL PROCESSING",
                EvaSymbols::HEXAGON,
                EvaSymbols::OPERATIONAL,
                EvaSymbols::DIAMOND,
                EvaSymbols::TRIANGLE
            );
            let empty_widget = Paragraph::new(empty_text)
                .style(EvaStyles::text_highlight())
                .alignment(Alignment::Center)
                .block(EvaBorders::operational().title(EvaFormat::title("LLM NEURAL INTERFACE")));
            f.render_widget(empty_widget, area);
        }
    }

    fn title(&self) -> Option<&str> {
        Some("LLM NEURAL INTERFACE")
    }

    fn border_style(&self) -> Style {
        if let Some(info) = self.stream_info {
            Style::default().fg(self.get_status_color(&info.status))
        } else {
            Style::default().fg(Color::DarkGray)
        }
    }
}

impl LLMStreamInfo {
    pub fn new(operation_type: String) -> Self {
        Self {
            operation_type,
            status: LLMStreamStatus::Initializing,
            progress: 0.0,
            input_tokens: None,
            output_tokens: None,
            total_tokens: None,
            estimated_cost_usd: None,
            model_name: None,
            error_message: None,
            start_time: Instant::now(),
            duration_ms: None,
            streamed_content: String::new(),
            streaming_position: 0,
            last_update: Instant::now(),
        }
    }

    pub fn set_connecting(&mut self) {
        self.status = LLMStreamStatus::Connecting;
        self.progress = 0.1;
        self.last_update = Instant::now();
    }

    pub fn set_streaming(&mut self, progress: f64) {
        self.status = LLMStreamStatus::Streaming;
        self.progress = progress.clamp(0.1, 0.9);
        self.last_update = Instant::now();
    }

    pub fn append_content(&mut self, content: &str) {
        self.streamed_content.push_str(content);
        self.streaming_position = self.streamed_content.len();
        self.last_update = Instant::now();
    }

    pub fn set_complete(&mut self, usage: &LLMUsage) {
        self.status = LLMStreamStatus::Complete;
        self.progress = 1.0;
        self.input_tokens = Some(usage.input_tokens);
        self.output_tokens = Some(usage.output_tokens);
        self.total_tokens = Some(usage.total_tokens);
        self.estimated_cost_usd = Some(usage.cost_usd);
        self.duration_ms = Some(usage.latency_ms);
        self.last_update = Instant::now();
    }

    pub fn set_error(&mut self, error_message: String) {
        self.status = LLMStreamStatus::Error;
        self.error_message = Some(error_message);
        self.last_update = Instant::now();
    }
}
