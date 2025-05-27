use crate::models::LLMUsage;
use crate::ui::widgets::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

#[derive(Debug, Clone)]
pub struct LLMCallInfo {
    pub operation_type: String,
    pub status: LLMCallStatus,
    pub progress: f64, // 0.0 to 1.0
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub cost_usd: Option<f64>,
    pub model_name: Option<String>,
    pub error_message: Option<String>,
    pub start_time: std::time::Instant,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LLMCallStatus {
    Initializing,
    InProgress,
    Success,
    Error,
}

pub struct LLMCallWidget<'a> {
    call_info: Option<&'a LLMCallInfo>,
    theme: &'a dyn Theme,
    show_detailed: bool,
}

impl<'a> LLMCallWidget<'a> {
    pub fn new(call_info: Option<&'a LLMCallInfo>) -> Self {
        Self {
            call_info,
            theme: &crate::ui::widgets::EvangelionTheme,
            show_detailed: true,
        }
    }

    pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_details(mut self, show_detailed: bool) -> Self {
        self.show_detailed = show_detailed;
        self
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        if let Some(info) = self.call_info {
            if self.show_detailed {
                self.render_detailed(f, area, info);
            } else {
                self.render_compact(f, area, info);
            }
        } else {
            // Render empty state
            let empty_block = Block::default()
                .title("LLM Operations")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            f.render_widget(empty_block, area);
        }
    }

    fn render_detailed(&self, f: &mut Frame, area: Rect, info: &LLMCallInfo) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header with operation type
                Constraint::Length(3), // Progress bar
                Constraint::Length(4), // Token information
                Constraint::Min(2),    // Status and details
            ])
            .split(area);

        // Header with operation type
        let status_symbol = match info.status {
            LLMCallStatus::Initializing => EvaSymbols::HEXAGON,
            LLMCallStatus::InProgress => EvaSymbols::OPERATIONAL,
            LLMCallStatus::Success => EvaSymbols::OPERATIONAL,
            LLMCallStatus::Error => EvaSymbols::CRITICAL,
        };

        let header_text = format!(
            "{} LLM OPERATION: {} | MODEL: {}",
            status_symbol,
            info.operation_type.to_uppercase(),
            info.model_name.as_deref().unwrap_or("UNKNOWN")
        );

        let header_style = match info.status {
            LLMCallStatus::Success => EvaStyles::text_success(),
            LLMCallStatus::Error => EvaStyles::text_error(),
            _ => EvaStyles::text_highlight(),
        };

        let header = Paragraph::new(header_text)
            .style(header_style)
            .alignment(Alignment::Center)
            .block(EvaBorders::header("NEURAL NETWORK INTERFACE"));

        f.render_widget(header, chunks[0]);

        // Progress bar
        let progress_label = match info.status {
            LLMCallStatus::Initializing => "INITIALIZING NEURAL LINK",
            LLMCallStatus::InProgress => "PROCESSING NEURAL PATTERNS",
            LLMCallStatus::Success => "NEURAL PROCESSING COMPLETE",
            LLMCallStatus::Error => "NEURAL LINK ERROR",
        };

        let progress_color = match info.status {
            LLMCallStatus::Success => Color::Green,
            LLMCallStatus::Error => Color::Red,
            LLMCallStatus::InProgress => EvaColors::ORANGE,
            LLMCallStatus::Initializing => Color::Yellow,
        };

        let progress_gauge = Gauge::default()
            .block(EvaBorders::operational().title(EvaFormat::title("PROCESSING STATUS")))
            .gauge_style(Style::default().fg(progress_color))
            .percent((info.progress * 100.0) as u16)
            .label(progress_label);

        f.render_widget(progress_gauge, chunks[1]);

        // Token information
        let token_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(chunks[2]);

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
        f.render_widget(input_widget, token_chunks[0]);

        // Output tokens
        let output_text = format!(
            "{} OUTPUT\n{}",
            EvaSymbols::TRIANGLE,
            info.output_tokens
                .map(|t| format!("{} TOKENS", t))
                .unwrap_or_else(|| "PENDING".to_string())
        );
        let output_widget = Paragraph::new(output_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(output_widget, token_chunks[1]);

        // Cost information
        let cost_text = format!(
            "{} COST\n{}",
            EvaSymbols::HEXAGON,
            info.cost_usd
                .map(|c| format!("${:.4} USD", c))
                .unwrap_or_else(|| "CALCULATING".to_string())
        );
        let cost_widget = Paragraph::new(cost_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel());
        f.render_widget(cost_widget, token_chunks[2]);

        // Status and details
        let status_text = match info.status {
            LLMCallStatus::Initializing => {
                "ESTABLISHING CONNECTION TO NEURAL NETWORK...".to_string()
            }
            LLMCallStatus::InProgress => {
                let elapsed = info.start_time.elapsed().as_millis();
                format!("PROCESSING... ELAPSED: {}ms", elapsed)
            }
            LLMCallStatus::Success => {
                let duration = info.duration_ms.unwrap_or(0);
                let total_tokens = info.total_tokens.unwrap_or(0);
                format!(
                    "OPERATION SUCCESSFUL\nDURATION: {}ms | TOTAL TOKENS: {}",
                    duration, total_tokens
                )
            }
            LLMCallStatus::Error => {
                format!(
                    "OPERATION FAILED\nERROR: {}",
                    info.error_message.as_deref().unwrap_or("UNKNOWN ERROR")
                )
            }
        };

        let status_style = match info.status {
            LLMCallStatus::Success => EvaStyles::text_success(),
            LLMCallStatus::Error => EvaStyles::text_error(),
            _ => EvaStyles::text_primary(),
        };

        let status_widget = Paragraph::new(status_text)
            .style(status_style)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(EvaBorders::panel().title(EvaFormat::title("OPERATION STATUS")));

        f.render_widget(status_widget, chunks[3]);
    }

    fn render_compact(&self, f: &mut Frame, area: Rect, info: &LLMCallInfo) {
        let status_symbol = match info.status {
            LLMCallStatus::Initializing => "⚡",
            LLMCallStatus::InProgress => "🧠",
            LLMCallStatus::Success => "✅",
            LLMCallStatus::Error => "❌",
        };

        let compact_text = match info.status {
            LLMCallStatus::InProgress => {
                let elapsed = info.start_time.elapsed().as_millis();
                format!(
                    "{} {} | {}ms | {}%",
                    status_symbol,
                    info.operation_type,
                    elapsed,
                    (info.progress * 100.0) as u8
                )
            }
            LLMCallStatus::Success => {
                format!(
                    "{} {} | {} tokens | ${:.4}",
                    status_symbol,
                    info.operation_type,
                    info.total_tokens.unwrap_or(0),
                    info.cost_usd.unwrap_or(0.0)
                )
            }
            LLMCallStatus::Error => {
                format!("{} {} | ERROR", status_symbol, info.operation_type)
            }
            LLMCallStatus::Initializing => {
                format!("{} {} | INITIALIZING", status_symbol, info.operation_type)
            }
        };

        let compact_widget = Paragraph::new(compact_text)
            .style(EvaStyles::text_secondary())
            .alignment(Alignment::Center)
            .block(EvaBorders::panel().title(EvaFormat::title("LLM OPERATION")));

        f.render_widget(compact_widget, area);
    }
}

impl LLMCallInfo {
    pub fn new(operation_type: String) -> Self {
        Self {
            operation_type,
            status: LLMCallStatus::Initializing,
            progress: 0.0,
            input_tokens: None,
            output_tokens: None,
            total_tokens: None,
            cost_usd: None,
            model_name: None,
            error_message: None,
            start_time: std::time::Instant::now(),
            duration_ms: None,
        }
    }

    pub fn set_in_progress(&mut self, progress: f64) {
        self.status = LLMCallStatus::InProgress;
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn set_success(&mut self, usage: &LLMUsage) {
        self.status = LLMCallStatus::Success;
        self.progress = 1.0;
        self.input_tokens = Some(usage.input_tokens);
        self.output_tokens = Some(usage.output_tokens);
        self.total_tokens = Some(usage.total_tokens);
        self.cost_usd = Some(usage.cost_usd);
        self.model_name = Some(usage.model_name.clone());
        self.duration_ms = Some(self.start_time.elapsed().as_millis() as u64);
    }

    pub fn set_error(&mut self, error_message: String) {
        self.status = LLMCallStatus::Error;
        self.progress = 0.0;
        self.error_message = Some(error_message);
        self.duration_ms = Some(self.start_time.elapsed().as_millis() as u64);
    }
}
