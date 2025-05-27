use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub mod api_debug;
pub mod syntax_editor_widget; // Renamed module
pub mod corner_decoration;
pub mod themed_loading_widget;
pub mod themed_progress_widget;
pub mod eva_theme;
pub mod themed_typing_indicator_widget; // Renamed module
pub mod home_layout;
pub mod learning_efficiency;
pub mod learning_unit_status;
pub mod llm_call;
pub mod llm_info;
// pub mod loading; // loading.rs was deleted
pub mod network_activity;
// pub mod progress_overview; // progress_overview.rs was deleted
pub mod stats_bar;
pub mod syntax_highlighter;
pub mod system_monitor;
pub mod text_editor;
pub mod typing_speed;

pub use api_debug::ApiDebugWidget;
// pub use code_editor::{CodeEditorWidget, CodeLanguage}; // Removed
pub use corner_decoration::{CornerDecorationWidget, DecoratedBlock};
// EvaLoadingWidget and EvaOperationType removed
// pub use eva_progress::EvaProgressWidget; // Removed
pub use eva_theme::{
    CleanTerminalTheme, EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, EvangelionTheme, // Keeping Eva* for now as they are part of the theme module itself
    Gundam00Theme, Theme, ThemeManager,
};
// pub use eva_typing::EvaTypingWidget; // Removed
pub use home_layout::HomeLayoutWidget;
pub use learning_efficiency::LearningEfficiencyWidget;
pub use learning_unit_status::LearningUnitStatusWidget;
pub use llm_call::{LLMCallInfo, LLMCallStatus, LLMCallWidget};
pub use llm_info::{LLMInfoWidget, LLMStreamInfo, LLMStreamStatus};
// pub use loading::LoadingWidget; // Removed
pub use network_activity::NetworkActivityWidget;
// pub use progress_overview::ProgressOverviewWidget; // Removed
pub use stats_bar::StatsBarWidget;
// Add new exports for ThemedLoadingWidget
pub use themed_loading_widget::{LoadingOperationType, ThemedLoadingWidget};
// Add new exports for ThemedProgressWidget
pub use themed_progress_widget::ThemedProgressWidget;
// Add new exports for ThemedTypingIndicatorWidget
pub use themed_typing_indicator_widget::ThemedTypingIndicatorWidget;
// Add new exports for SyntaxEditorWidget
pub use syntax_editor_widget::{CodeLanguage, SyntaxEditorWidget};
pub use system_monitor::{SystemMetrics, SystemMonitorWidget};
pub use text_editor::TextEditor;
pub use typing_speed::TypingSpeedWidget;

/// Base trait for all widgets in the application
pub trait Widget {
    /// Render the widget to the given area
    fn render(&self, f: &mut Frame, area: Rect);

    /// Get the title of the widget (optional)
    fn title(&self) -> Option<&str> {
        None
    }

    /// Get the border style for the widget
    fn border_style(&self) -> Style {
        Style::default().fg(Color::Blue)
    }

    /// Whether the widget should have borders
    fn has_borders(&self) -> bool {
        true
    }

    /// Create a block with title and borders for the widget
    fn create_block(&self) -> Block {
        let mut block = Block::default();

        if self.has_borders() {
            block = block
                .borders(Borders::ALL)
                .border_style(self.border_style());
        }

        if let Some(title) = self.title() {
            block = block.title(title);
        }

        block
    }
}

/// Widget for displaying loading states with animation
pub struct LoadingIndicator {
    pub message: String,
    pub is_loading: bool,
}

impl LoadingIndicator {
    pub fn new(message: String, is_loading: bool) -> Self {
        Self {
            message,
            is_loading,
        }
    }

    fn get_animation_frame(&self) -> &'static str {
        if !self.is_loading {
            return "";
        }

        let frame = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            / 200)
            % 8;

        match frame {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            3 => "⠸",
            4 => "⠼",
            5 => "⠴",
            6 => "⠦",
            _ => "⠧",
        }
    }
}

impl Widget for LoadingIndicator {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = if self.is_loading {
            format!("{} {}", self.get_animation_frame(), self.message)
        } else {
            self.message.clone()
        };

        let color = if self.is_loading {
            Color::Yellow
        } else {
            Color::Green
        };

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("Status")
    }

    fn border_style(&self) -> Style {
        if self.is_loading {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        }
    }
}

/// Simple progress bar widget
pub struct ProgressBar {
    pub label: String,
    pub progress: f64, // 0.0 to 1.0
    pub color: Color,
}

impl ProgressBar {
    pub fn new(label: String, progress: f64, color: Color) -> Self {
        Self {
            label,
            progress,
            color,
        }
    }
}

impl Widget for ProgressBar {
    fn render(&self, f: &mut Frame, area: Rect) {
        let gauge = Gauge::default()
            .block(self.create_block())
            .gauge_style(Style::default().fg(self.color))
            .ratio(self.progress.clamp(0.0, 1.0))
            .label(format!("{} ({:.1}%)", self.label, self.progress * 100.0));

        f.render_widget(gauge, area);
    }

    fn border_style(&self) -> Style {
        Style::default().fg(self.color)
    }
}
