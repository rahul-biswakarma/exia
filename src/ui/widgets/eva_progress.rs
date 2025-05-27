use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Widget};
use crate::models::CostAnalytics;
use crate::storage::Statistics;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

pub struct EvaProgressWidget<'a> {
    pub stats: &'a Statistics,
    pub cost_analytics: Option<&'a CostAnalytics>,
    pub show_detailed: bool,
}

impl<'a> EvaProgressWidget<'a> {
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

    fn get_sync_rate(&self) -> f64 {
        self.stats.success_rate
    }

    fn get_operational_status(&self) -> (bool, bool) {
        let success_rate = self.stats.success_rate;
        let is_ok = success_rate >= 70.0;
        let is_warning = success_rate >= 40.0 && success_rate < 70.0;
        (is_ok, is_warning)
    }

    fn render_main_display(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(4), // Sync rate
                Constraint::Min(0),    // Stats
            ])
            .split(area);

        // Header
        let header_text = format!(
            "{} CENTRAL DOGMA - ALGORITHM LEARNING SYSTEM",
            EvaSymbols::HEXAGON
        );
        let header = Paragraph::new(header_text)
            .style(EvaStyles::text_highlight())
            .block(EvaBorders::header("LEARNING ANALYTICS"));
        f.render_widget(header, chunks[0]);

        // Sync Rate Display
        let sync_rate = self.get_sync_rate();
        let sync_symbol = EvaSymbols::sync_rate_symbol(sync_rate);
        let sync_text = format!(
            "{} LEARNING EFFICIENCY RATE: {:.1}%\n{} {}",
            EvaSymbols::OPERATIONAL,
            sync_rate,
            sync_symbol,
            EvaFormat::progress_bar(sync_rate / 100.0, 20)
        );

        let sync_style = if sync_rate >= 80.0 {
            EvaStyles::sync_rate()
        } else if sync_rate >= 60.0 {
            EvaStyles::text_warning()
        } else {
            EvaStyles::text_critical()
        };

        let sync_widget = Paragraph::new(sync_text)
            .style(sync_style)
            .block(EvaBorders::operational());
        f.render_widget(sync_widget, chunks[1]);

        // Main stats
        self.render_stats_panel(f, chunks[2]);
    }

    fn render_stats_panel(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Left panel
                Constraint::Percentage(50), // Right panel
            ])
            .split(area);

        // Left panel - Learning stats
        let learning_stats = format!(
            "{} LEARNING STATISTICS\n\n{}\n{}\n{}\n{}\n{}",
            EvaSymbols::DIAMOND,
            EvaFormat::status("PROBLEMS ATTEMPTED", &self.stats.total_questions.to_string(), true),
            EvaFormat::status(
                "SOLUTIONS ACCEPTED",
                &self.stats.accepted_solutions.to_string(),
                true
            ),
            EvaFormat::status(
                "SUCCESS RATE",
                &format!("{:.1}%", self.stats.success_rate),
                self.stats.success_rate >= 70.0
            ),
            EvaFormat::status(
                "CURRENT STREAK",
                &self.stats.current_streak.to_string(),
                self.stats.current_streak > 0
            ),
            EvaFormat::readout(
                "AVG EXECUTION TIME",
                &format!("{:.1}", self.stats.avg_execution_time),
                "MS"
            )
        );

        let (is_ok, is_warning) = self.get_operational_status();
        let learning_border = if !is_ok && !is_warning {
            EvaBorders::critical()
        } else if is_warning {
            EvaBorders::warning()
        } else {
            EvaBorders::operational()
        };

        let learning_widget = Paragraph::new(learning_stats)
            .style(EvaStyles::text_primary())
            .block(learning_border.title(EvaFormat::title("LEARNING UNIT STATUS")));
        f.render_widget(learning_widget, chunks[0]);

        // Right panel - System status
        let system_stats = if let Some(cost) = self.cost_analytics {
            format!(
                "{} SYSTEM RESOURCES\n\n{}\n{}\n{}\n\n{} AT FIELD STATUS\n{}",
                EvaSymbols::TRIANGLE,
                EvaFormat::readout("TOTAL COST", &format!("${:.4}", cost.total_cost_usd), "USD"),
                EvaFormat::readout("TOKENS USED", &cost.tokens_used.to_string(), ""),
                EvaFormat::readout("API REQUESTS", &cost.requests_count.to_string(), ""),
                EvaSymbols::HEXAGON,
                EvaFormat::hex_display("FIELD STABLE")
            )
        } else {
            format!(
                "{} SYSTEM RESOURCES\n\n{}\n\n{} AT FIELD STATUS\n{}",
                EvaSymbols::TRIANGLE,
                EvaFormat::status("RESOURCES", "LOADING", false),
                EvaSymbols::HEXAGON,
                EvaFormat::hex_display("FIELD UNKNOWN")
            )
        };

        let system_widget = Paragraph::new(system_stats)
            .style(EvaStyles::text_secondary())
            .block(EvaBorders::panel().title(EvaFormat::title("NERV SYSTEMS")));
        f.render_widget(system_widget, chunks[1]);
    }

    fn render_detailed_view(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Main display
                Constraint::Min(0),    // Detailed breakdown
            ])
            .split(area);

        self.render_main_display(f, chunks[0]);

        // Detailed breakdown
        let detail_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Topics
                Constraint::Percentage(50), // Difficulties
            ])
            .split(chunks[1]);

        // Topic distribution
        let mut topic_text = format!("{} ALGORITHM CATEGORIES\n\n", EvaSymbols::SQUARE);
        for (topic, count) in self.stats.topic_distribution.iter().take(5) {
            topic_text.push_str(&format!(
                "{} {}: {}\n",
                EvaSymbols::ARROW_RIGHT,
                topic,
                count
            ));
        }

        let topics_widget = Paragraph::new(topic_text)
            .style(EvaStyles::text_secondary())
            .block(EvaBorders::panel().title(EvaFormat::title("ALGORITHM TYPES")));
        f.render_widget(topics_widget, detail_chunks[0]);

        // Difficulty distribution
        let mut difficulty_text = format!("{} COMPLEXITY LEVELS\n\n", EvaSymbols::WARNING);
        for (difficulty, count) in self.stats.difficulty_distribution.iter().take(3) {
            difficulty_text.push_str(&format!(
                "{} {}: {}\n",
                EvaSymbols::ARROW_RIGHT,
                difficulty,
                count
            ));
        }

        let difficulty_widget = Paragraph::new(difficulty_text)
            .style(EvaStyles::text_secondary())
            .block(EvaBorders::panel().title(EvaFormat::title("DIFFICULTY LEVELS")));
        f.render_widget(difficulty_widget, detail_chunks[1]);
    }
}

impl<'a> Widget for EvaProgressWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        if self.show_detailed {
            self.render_detailed_view(f, area);
        } else {
            self.render_main_display(f, area);
        }
    }

    fn title(&self) -> Option<&str> {
        Some("MAGI SYSTEM")
    }

    fn border_style(&self) -> Style {
        let (is_ok, is_warning) = self.get_operational_status();
        if !is_ok && !is_warning {
            Style::default().fg(EvaColors::STATUS_CRITICAL)
        } else if is_warning {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        }
    }
}
