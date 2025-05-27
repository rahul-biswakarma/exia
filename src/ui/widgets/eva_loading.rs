use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Widget};
use ratatui::{layout::Rect, style::Style, widgets::Paragraph, Frame};

pub struct EvaLoadingWidget {
    pub message: String,
    pub is_loading: bool,
    pub show_progress: bool,
    pub progress: f64, // 0.0 to 1.0
    pub operation_type: EvaOperationType,
}

#[derive(Debug, Clone)]
pub enum EvaOperationType {
    SystemBoot,
    AngelDetection,
    EvaActivation,
    SyncTest,
    DataAnalysis,
    MagiCalculation,
    AtFieldGeneration,
}

impl EvaLoadingWidget {
    pub fn new(message: String, is_loading: bool) -> Self {
        Self {
            message,
            is_loading,
            show_progress: false,
            progress: 0.0,
            operation_type: EvaOperationType::SystemBoot,
        }
    }

    pub fn with_progress(mut self, progress: f64) -> Self {
        self.show_progress = true;
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn with_operation_type(mut self, operation_type: EvaOperationType) -> Self {
        self.operation_type = operation_type;
        self
    }

    fn get_operation_prefix(&self) -> &'static str {
        match self.operation_type {
            EvaOperationType::SystemBoot => "System Initialization",
            EvaOperationType::AngelDetection => "Pattern Analysis",
            EvaOperationType::EvaActivation => "Unit Activation",
            EvaOperationType::SyncTest => "Synchronization Test",
            EvaOperationType::DataAnalysis => "Data Processing",
            EvaOperationType::MagiCalculation => "Calculation",
            EvaOperationType::AtFieldGeneration => "Field Generation",
        }
    }

    fn get_loading_animation(&self) -> String {
        if !self.is_loading {
            return format!("{} {}", EvaSymbols::OPERATIONAL, "Complete");
        }

        // Slower animation - update every 500ms instead of 200ms
        let frame = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            / 500)
            % EvaSymbols::LOADING_FRAMES.len() as u128;

        let animation_char = EvaSymbols::LOADING_FRAMES[frame as usize];
        format!("{} {}", animation_char, self.get_operation_prefix())
    }

    fn get_progress_display(&self) -> String {
        if !self.show_progress {
            return String::new();
        }

        let progress_bar = EvaFormat::progress_bar(self.progress, 20);
        let percentage = self.progress * 100.0;

        format!("\n\nProgress: {:.1}%\n[{}]", percentage, progress_bar)
    }

    fn get_status_readouts(&self) -> String {
        let timestamp = EvaFormat::timestamp();
        let status = if self.is_loading {
            "In Progress"
        } else {
            "Complete"
        };

        format!("\n\nStatus: {}\nTime: {}", status, timestamp)
    }

    fn format_display(&self) -> String {
        let animation = self.get_loading_animation();
        let progress = self.get_progress_display();
        let status = self.get_status_readouts();

        // Remove flashing dots - just show static message
        format!("{}\n\n{}{}{}", animation, self.message, progress, status)
    }

    fn get_border_style(&self) -> ratatui::widgets::Block<'static> {
        if self.is_loading {
            EvaBorders::warning()
        } else {
            EvaBorders::operational()
        }
    }

    fn get_text_style(&self) -> Style {
        if self.is_loading {
            EvaStyles::text_warning()
        } else {
            EvaStyles::text_success()
        }
    }
}

impl Widget for EvaLoadingWidget {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.format_display();

        let paragraph = Paragraph::new(content)
            .style(self.get_text_style())
            .block(self.get_border_style().title("Operations"));

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("Operations")
    }

    fn border_style(&self) -> Style {
        if self.is_loading {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        }
    }
}
