use super::Widget;
use crate::ui::{NetworkActivity, NetworkStatus};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub struct NetworkActivityWidget<'a> {
    pub activities: &'a [NetworkActivity],
    pub show_details: bool,
}

impl<'a> NetworkActivityWidget<'a> {
    pub fn new(activities: &'a [NetworkActivity]) -> Self {
        Self {
            activities,
            show_details: false,
        }
    }

    pub fn with_details(mut self, show_details: bool) -> Self {
        self.show_details = show_details;
        self
    }

    fn get_status_icon(&self, status: &NetworkStatus) -> &'static str {
        match status {
            NetworkStatus::InProgress => "🔄",
            NetworkStatus::Success => "✅",
            NetworkStatus::Failed => "❌",
            NetworkStatus::Timeout => "⏰",
        }
    }

    fn get_status_color(&self, status: &NetworkStatus) -> Color {
        match status {
            NetworkStatus::InProgress => Color::Yellow,
            NetworkStatus::Success => Color::Green,
            NetworkStatus::Failed => Color::Red,
            NetworkStatus::Timeout => Color::Magenta,
        }
    }

    fn get_summary(&self) -> String {
        if self.activities.is_empty() {
            return "🌐 No network activity".to_string();
        }

        let total = self.activities.len();
        let success = self
            .activities
            .iter()
            .filter(|a| matches!(a.status, NetworkStatus::Success))
            .count();
        let failed = self
            .activities
            .iter()
            .filter(|a| matches!(a.status, NetworkStatus::Failed))
            .count();
        let in_progress = self
            .activities
            .iter()
            .filter(|a| matches!(a.status, NetworkStatus::InProgress))
            .count();

        let avg_latency = if !self.activities.is_empty() {
            self.activities.iter().map(|a| a.latency_ms).sum::<u64>() / self.activities.len() as u64
        } else {
            0
        };

        format!(
            "📊 Total: {} | ✅ Success: {} | ❌ Failed: {} | 🔄 Active: {}\n⚡ Avg Latency: {}ms",
            total, success, failed, in_progress, avg_latency
        )
    }

    fn get_detailed_view(&self) -> String {
        if self.activities.is_empty() {
            return "No recent network activity".to_string();
        }

        let mut content = self.get_summary();
        content.push_str("\n\n📋 Recent Activity:\n");

        for activity in self.activities.iter().rev().take(5) {
            content.push_str(&format!(
                "{} {} {}ms - {}\n",
                self.get_status_icon(&activity.status),
                activity.endpoint,
                activity.latency_ms,
                activity.timestamp
            ));
        }

        content
    }
}

impl<'a> Widget for NetworkActivityWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = if self.show_details {
            self.get_detailed_view()
        } else {
            self.get_summary()
        };

        let paragraph = Paragraph::new(content)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .wrap(Wrap { trim: true })
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("🌐 Network Activity")
    }

    fn border_style(&self) -> Style {
        let color = if self
            .activities
            .iter()
            .any(|a| matches!(a.status, NetworkStatus::InProgress))
        {
            Color::Yellow
        } else if self
            .activities
            .iter()
            .any(|a| matches!(a.status, NetworkStatus::Failed))
        {
            Color::Red
        } else {
            Color::Green
        };

        Style::default().fg(color)
    }
}
