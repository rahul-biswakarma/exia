use super::Widget;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct LoadingWidget {
    pub message: String,
    pub is_loading: bool,
    pub show_progress: bool,
    pub progress: f64, // 0.0 to 1.0
}

impl LoadingWidget {
    pub fn new(message: String, is_loading: bool) -> Self {
        Self {
            message,
            is_loading,
            show_progress: false,
            progress: 0.0,
        }
    }

    pub fn with_progress(mut self, progress: f64) -> Self {
        self.show_progress = true;
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    fn get_spinner_frame(&self) -> &'static str {
        if !self.is_loading {
            return "✅";
        }

        let frame = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            / 150)
            % 12;

        match frame {
            0 => "⠋",
            1 => "⠙",
            2 => "⠹",
            3 => "⠸",
            4 => "⠼",
            5 => "⠴",
            6 => "⠦",
            7 => "⠧",
            8 => "⠇",
            9 => "⠏",
            10 => "⠋",
            _ => "⠙",
        }
    }

    fn get_progress_bar(&self) -> String {
        if !self.show_progress {
            return String::new();
        }

        let width = 20;
        let filled = (self.progress * width as f64) as usize;
        let empty = width - filled;

        format!(
            "\n[{}{}] {:.1}%",
            "█".repeat(filled),
            "░".repeat(empty),
            self.progress * 100.0
        )
    }

    fn get_loading_message(&self) -> String {
        let dots = match (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            / 500)
            % 4
        {
            0 => "",
            1 => ".",
            2 => "..",
            _ => "...",
        };

        if self.is_loading {
            format!("{}{}", self.message, dots)
        } else {
            self.message.clone()
        }
    }
}

impl Widget for LoadingWidget {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = format!(
            "{} {}{}",
            self.get_spinner_frame(),
            self.get_loading_message(),
            self.get_progress_bar()
        );

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
        Some("🔄 Status")
    }

    fn border_style(&self) -> Style {
        if self.is_loading {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Green)
        }
    }
}
