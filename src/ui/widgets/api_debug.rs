use super::Widget;
use crate::ui::{ApiCall, ApiCallStatus};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub struct ApiDebugWidget<'a> {
    pub api_calls: &'a [ApiCall],
    pub is_loading: bool,
    pub show_detailed: bool,
}

impl<'a> ApiDebugWidget<'a> {
    pub fn new(api_calls: &'a [ApiCall], is_loading: bool) -> Self {
        Self {
            api_calls,
            is_loading,
            show_detailed: false,
        }
    }

    pub fn with_details(mut self, show_detailed: bool) -> Self {
        self.show_detailed = show_detailed;
        self
    }

    fn get_status_icon(&self, status: &ApiCallStatus) -> &'static str {
        match status {
            ApiCallStatus::Pending => "⏳",
            ApiCallStatus::Success => "✅",
            ApiCallStatus::Error => "❌",
        }
    }

    fn get_loading_animation(&self) -> String {
        if !self.is_loading {
            return String::new();
        }

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

        format!("🔄 Loading{}\n", dots)
    }

    fn get_summary(&self) -> String {
        if self.api_calls.is_empty() {
            return "No API calls yet\nPress 'g' to generate a question".to_string();
        }

        let total = self.api_calls.len();
        let success = self
            .api_calls
            .iter()
            .filter(|c| matches!(c.status, ApiCallStatus::Success))
            .count();
        let errors = self
            .api_calls
            .iter()
            .filter(|c| matches!(c.status, ApiCallStatus::Error))
            .count();
        let pending = self
            .api_calls
            .iter()
            .filter(|c| matches!(c.status, ApiCallStatus::Pending))
            .count();

        format!(
            "📊 Total: {} | ✅ Success: {} | ❌ Errors: {} | ⏳ Pending: {}",
            total, success, errors, pending
        )
    }

    fn get_recent_calls(&self) -> String {
        if self.api_calls.is_empty() {
            return String::new();
        }

        let mut content = String::new();
        content.push_str("\n\n📋 Recent Calls:\n");

        for call in self.api_calls.iter().rev().take(4) {
            content.push_str(&format!(
                "{} {} - {}\n",
                self.get_status_icon(&call.status),
                call.endpoint,
                call.message
            ));
        }

        content
    }

    fn get_detailed_view(&self) -> String {
        let mut content = self.get_loading_animation();
        content.push_str(&self.get_summary());

        if self.show_detailed {
            content.push_str(&self.get_recent_calls());
        }

        content
    }
}

impl<'a> Widget for ApiDebugWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.get_detailed_view();

        let color = if self.is_loading {
            Color::Yellow
        } else if self
            .api_calls
            .iter()
            .any(|c| matches!(c.status, ApiCallStatus::Error))
        {
            Color::Red
        } else {
            Color::Gray
        };

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(color))
            .wrap(Wrap { trim: true })
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("🔧 API Debug")
    }

    fn border_style(&self) -> Style {
        let color = if self.is_loading {
            Color::Yellow
        } else if self
            .api_calls
            .iter()
            .any(|c| matches!(c.status, ApiCallStatus::Error))
        {
            Color::Red
        } else {
            Color::Gray
        };

        Style::default().fg(color)
    }
}
