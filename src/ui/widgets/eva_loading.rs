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
            EvaOperationType::SystemBoot => "SYSTEM INITIALIZATION",
            EvaOperationType::AngelDetection => "ANGEL PATTERN ANALYSIS",
            EvaOperationType::EvaActivation => "EVA UNIT ACTIVATION",
            EvaOperationType::SyncTest => "SYNCHRONIZATION TEST",
            EvaOperationType::DataAnalysis => "MAGI DATA PROCESSING",
            EvaOperationType::MagiCalculation => "MAGI CALCULATION",
            EvaOperationType::AtFieldGeneration => "AT FIELD GENERATION",
        }
    }

    fn get_loading_animation(&self) -> String {
        if !self.is_loading {
            return EvaSymbols::OPERATIONAL.to_string();
        }

        let frame = EvaSymbols::loading_frame();
        format!("{} {}", frame, self.get_operation_prefix())
    }

    fn get_progress_display(&self) -> String {
        if !self.show_progress {
            return String::new();
        }

        let progress_bar = EvaFormat::progress_bar(self.progress, 25);
        let percentage = self.progress * 100.0;

        format!(
            "\n\n{} COMPLETION: {:.1}%\n[{}]",
            EvaSymbols::TRIANGLE,
            percentage,
            progress_bar
        )
    }

    fn get_status_readouts(&self) -> String {
        let timestamp = EvaFormat::timestamp();
        let status = if self.is_loading {
            "IN PROGRESS"
        } else {
            "COMPLETE"
        };

        format!(
            "\n\n{} STATUS: {}\n{} TIME: {}",
            EvaSymbols::DIAMOND,
            status,
            EvaSymbols::SQUARE,
            timestamp
        )
    }

    fn format_display(&self) -> String {
        let animation = self.get_loading_animation();
        let progress = self.get_progress_display();
        let status = self.get_status_readouts();

        let dots = if self.is_loading {
            match (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                / 300)
                % 4
            {
                0 => "",
                1 => ".",
                2 => "..",
                _ => "...",
            }
        } else {
            ""
        };

        format!(
            "{}\n\n{} {}{}{}{}",
            animation,
            EvaSymbols::HEXAGON,
            self.message.to_uppercase(),
            dots,
            progress,
            status
        )
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

        let paragraph = Paragraph::new(content).style(self.get_text_style()).block(
            self.get_border_style()
                .title(EvaFormat::title("NERV OPERATIONS")),
        );

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("NERV OPERATIONS")
    }

    fn border_style(&self) -> Style {
        if self.is_loading {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        }
    }
}
