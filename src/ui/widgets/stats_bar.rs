use super::Widget;
use crate::ui::TypingMetrics;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct StatsBarWidget<'a> {
    pub success_count: usize,
    pub error_count: usize,
    pub api_calls_count: usize,
    pub network_activity_count: usize,
    pub typing_metrics: &'a TypingMetrics,
    pub compact: bool,
}

impl<'a> StatsBarWidget<'a> {
    pub fn new(
        success_count: usize,
        error_count: usize,
        api_calls_count: usize,
        network_activity_count: usize,
        typing_metrics: &'a TypingMetrics,
    ) -> Self {
        Self {
            success_count,
            error_count,
            api_calls_count,
            network_activity_count,
            typing_metrics,
            compact: false,
        }
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    fn get_success_rate(&self) -> f64 {
        let total = self.success_count + self.error_count;
        if total == 0 {
            0.0
        } else {
            (self.success_count as f64 / total as f64) * 100.0
        }
    }

    fn get_success_rate_color(&self) -> Color {
        match self.get_success_rate() as u32 {
            0..=30 => Color::Red,
            31..=60 => Color::Yellow,
            61..=80 => Color::Green,
            81..=95 => Color::Cyan,
            _ => Color::Magenta,
        }
    }

    fn get_typing_speed_color(&self) -> Color {
        match self.typing_metrics.current_wpm as u32 {
            0..=20 => Color::Red,
            21..=40 => Color::Yellow,
            41..=60 => Color::Green,
            61..=80 => Color::Cyan,
            _ => Color::Magenta,
        }
    }

    fn get_compact_stats(&self) -> String {
        format!(
            "✅ {} | ❌ {} | 📊 {} | 🌐 {} | ⌨️ {:.0} WPM",
            self.success_count,
            self.error_count,
            self.api_calls_count,
            self.network_activity_count,
            self.typing_metrics.current_wpm
        )
    }

    fn get_detailed_stats(&self) -> String {
        format!(
            "✅ Success: {} | ❌ Errors: {} | 📈 Rate: {:.1}%\n📊 API Calls: {} | 🌐 Network: {} | ⌨️ WPM: {:.1} | 📝 Chars: {}",
            self.success_count,
            self.error_count,
            self.get_success_rate(),
            self.api_calls_count,
            self.network_activity_count,
            self.typing_metrics.current_wpm,
            self.typing_metrics.total_characters
        )
    }
}

impl<'a> Widget for StatsBarWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = if self.compact {
            self.get_compact_stats()
        } else {
            self.get_detailed_stats()
        };

        let paragraph = Paragraph::new(content)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        if self.compact {
            None
        } else {
            Some("📊 Session Stats")
        }
    }

    fn border_style(&self) -> Style {
        Style::default().fg(Color::Cyan)
    }

    fn has_borders(&self) -> bool {
        !self.compact
    }
}
