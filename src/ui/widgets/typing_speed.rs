use super::Widget;
use crate::ui::TypingMetrics;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct TypingSpeedWidget<'a> {
    pub metrics: &'a TypingMetrics,
}

impl<'a> TypingSpeedWidget<'a> {
    pub fn new(metrics: &'a TypingMetrics) -> Self {
        Self { metrics }
    }

    fn get_wpm_color(&self) -> Color {
        match self.metrics.current_wpm as u32 {
            0..=20 => Color::Red,
            21..=40 => Color::Yellow,
            41..=60 => Color::Green,
            61..=80 => Color::Cyan,
            _ => Color::Magenta,
        }
    }

    fn get_performance_indicator(&self) -> &'static str {
        match self.metrics.current_wpm as u32 {
            0..=20 => "🐌",
            21..=40 => "🚶",
            41..=60 => "🏃",
            61..=80 => "🚀",
            _ => "⚡",
        }
    }
}

impl<'a> Widget for TypingSpeedWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = format!(
            "{} Current: {:.1} WPM\n📊 Average: {:.1} WPM\n⌨️  Characters: {}\n⏱️  Time: {:.1}s",
            self.get_performance_indicator(),
            self.metrics.current_wpm,
            self.metrics.average_wpm,
            self.metrics.total_characters,
            self.metrics.total_time_ms as f64 / 1000.0
        );

        let paragraph = Paragraph::new(content)
            .style(
                Style::default()
                    .fg(self.get_wpm_color())
                    .add_modifier(Modifier::BOLD),
            )
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("⌨️ Typing Speed")
    }

    fn border_style(&self) -> Style {
        Style::default().fg(self.get_wpm_color())
    }
}
