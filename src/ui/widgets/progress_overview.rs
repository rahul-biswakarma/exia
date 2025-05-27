use super::Widget;
use crate::models::CostAnalytics;
use crate::storage::Statistics;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub struct ProgressOverviewWidget<'a> {
    pub stats: &'a Statistics,
    pub cost_analytics: Option<&'a CostAnalytics>,
    pub show_detailed: bool,
}

impl<'a> ProgressOverviewWidget<'a> {
    pub fn new(stats: &'a Statistics) -> Self {
        Self {
            stats,
            cost_analytics: None,
            show_detailed: false,
        }
    }

    pub fn with_cost_analytics(mut self, cost_analytics: Option<&'a CostAnalytics>) -> Self {
        self.cost_analytics = cost_analytics;
        self
    }

    pub fn with_details(mut self, show_detailed: bool) -> Self {
        self.show_detailed = show_detailed;
        self
    }

    fn get_basic_stats(&self) -> String {
        format!(
            "📝 Questions: {} | ✅ Solved: {} | 📈 Success: {:.1}%\n🔥 Streak: {} | ⏱️ Avg Time: {:.1}ms",
            self.stats.total_questions,
            self.stats.accepted_solutions,
            self.stats.success_rate,
            self.stats.current_streak,
            self.stats.avg_execution_time
        )
    }

    fn get_cost_info(&self) -> String {
        if let Some(cost) = self.cost_analytics {
            format!(
                "\n💰 Total Cost: ${:.4} | 🎯 Tokens: {} | 📊 Requests: {}",
                cost.total_cost_usd, cost.tokens_used, cost.requests_count
            )
        } else {
            "\n💰 Loading cost data...".to_string()
        }
    }

    fn get_detailed_breakdown(&self) -> String {
        let mut content = self.get_basic_stats();
        content.push_str(&self.get_cost_info());

        if self.show_detailed {
            content.push_str("\n\n📊 Topic Breakdown:");
            for (topic, count) in self.stats.topic_distribution.iter().take(3) {
                content.push_str(&format!("\n  • {}: {}", topic, count));
            }

            content.push_str("\n\n🎯 Difficulty Breakdown:");
            for (difficulty, count) in self.stats.difficulty_distribution.iter().take(3) {
                content.push_str(&format!("\n  • {}: {}", difficulty, count));
            }
        }

        content
    }

    fn get_performance_color(&self) -> Color {
        match self.stats.success_rate as u32 {
            0..=30 => Color::Red,
            31..=60 => Color::Yellow,
            61..=80 => Color::Green,
            81..=95 => Color::Cyan,
            _ => Color::Magenta,
        }
    }
}

impl<'a> Widget for ProgressOverviewWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.get_detailed_breakdown();

        let paragraph = Paragraph::new(content)
            .style(
                Style::default()
                    .fg(self.get_performance_color())
                    .add_modifier(Modifier::BOLD),
            )
            .block(self.create_block());

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("📊 Progress Overview")
    }

    fn border_style(&self) -> Style {
        Style::default().fg(self.get_performance_color())
    }
}
