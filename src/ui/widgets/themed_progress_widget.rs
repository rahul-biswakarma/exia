use super::eva_theme::Theme;
use super::Widget;
use crate::models::CostAnalytics;
use crate::storage::Statistics;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};

pub struct ThemedProgressWidget<'a> {
    // Renamed struct
    pub stats: &'a Statistics,
    pub cost_analytics: Option<&'a CostAnalytics>,
    pub show_detailed: bool,
    pub theme: &'a dyn Theme, // Theme is now mandatory
}

impl<'a> ThemedProgressWidget<'a> {
    // Renamed struct
    pub fn new(stats: &'a Statistics, theme: &'a dyn Theme) -> Self {
        // Added theme to constructor
        Self {
            stats,
            cost_analytics: None,
            show_detailed: false,
            theme, // Theme initialized
        }
    }

    // with_theme method removed

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
        let is_ok = success_rate >= 70.0; // Assuming 70 as OK threshold
        let is_warning = success_rate >= 40.0 && success_rate < 70.0; // Assuming 40-70 as Warning
        (is_ok, is_warning)
    }

    fn get_themed_sync_symbol(&self, rate: f64) -> String {
        // Return type changed to String
        let indicators = self.theme.symbols().status_indicators();
        match rate {
            r if r >= 80.0 => indicators.sync_high.clone(),
            r if r >= 60.0 => indicators.sync_medium.clone(),
            r if r >= 40.0 => indicators.sync_low.clone(),
            _ => indicators.sync_critical.clone(),
        }
        // Fallback removed
    }

    fn get_themed_progress_bar(&self, progress: f64, width: usize) -> String {
        let progress_symbols = self.theme.symbols().progress_symbols();
        let filled = (progress * width as f64).round() as usize; // .round() added for potentially better visual
        let empty_symbol = progress_symbols.last().unwrap_or(&" ").to_string(); // .to_string()
        let fill_symbol = progress_symbols.first().unwrap_or(&"█").to_string(); // .to_string()

        format!(
            "{}{}",
            fill_symbol.repeat(filled),
            empty_symbol.repeat(width - filled)
        )
        // Fallback removed
    }

    // Private helper for hex display, retained as it doesn't rely on theme-specific details.
    fn hex_display(&self, content: &str) -> String {
        // Simplified: In a real scenario, this might involve more complex formatting.
        // For now, just returns a styled string, but styling should come from theme.
        // This function itself should not apply styles.
        format!("[HEX: {}]", content.to_uppercase())
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
        let header_symbol = self
            .theme
            .symbols()
            .geometric_shapes()
            .get(0)
            .unwrap_or(&"")
            .to_string();
        let header_text = format!(
            "{} CENTRAL DOGMA - ALGORITHM LEARNING SYSTEM",
            header_symbol // EvaSymbols replaced
        );
        let header = Paragraph::new(header_text)
            .style(self.theme.styles().text_highlight()) // EvaStyles replaced
            .block(self.theme.borders().header_block("LEARNING ANALYTICS")); // EvaBorders replaced
        f.render_widget(header, chunks[0]);

        // Sync Rate Display
        let sync_rate = self.get_sync_rate();
        let sync_symbol = self.get_themed_sync_symbol(sync_rate);
        let progress_bar = self.get_themed_progress_bar(sync_rate / 100.0, 20);
        let operational_symbol = self.theme.symbols().operational(); // EvaSymbols removed

        let sync_text = format!(
            "{} LEARNING EFFICIENCY RATE: {:.1}%\n{} {}",
            operational_symbol, sync_rate, sync_symbol, progress_bar
        );

        let sync_style = if sync_rate >= 80.0 {
            // Assuming 80 as high threshold
            self.theme.styles().text_success()
        } else if sync_rate >= 60.0 {
            // Assuming 60 as medium threshold
            self.theme.styles().text_warning()
        } else {
            self.theme.styles().text_error()
        }; // EvaStyles replaced

        let sync_widget = Paragraph::new(sync_text)
            .style(sync_style)
            .block(self.theme.borders().default_block()); // EvaBorders replaced (using default_block)
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
        let diamond_symbol = self
            .theme
            .symbols()
            .geometric_shapes()
            .get(1)
            .unwrap_or(&"")
            .to_string(); // EvaSymbols replaced

        let problems_attempted_str = format!(
            "{} {}: {}",
            self.theme.symbols().status_indicators().info, // Example, pick a suitable symbol
            "PROBLEMS ATTEMPTED",
            self.stats.total_questions
        );
        let solutions_accepted_str = format!(
            "{} {}: {}",
            if self.stats.accepted_solutions > 0 {
                self.theme.symbols().status_indicators().operational
            } else {
                self.theme.symbols().status_indicators().error
            },
            "SOLUTIONS ACCEPTED",
            self.stats.accepted_solutions
        );
        let success_rate_str = format!(
            "{} {}: {:.1}%",
            if self.stats.success_rate >= 70.0 {
                self.theme.symbols().status_indicators().operational
            } else {
                self.theme.symbols().status_indicators().warning
            },
            "SUCCESS RATE",
            self.stats.success_rate
        );
        let degraded_symbol = &self.theme.symbols().status_indicators().degraded;
        let current_streak_str = format!(
            "{} {}: {}",
            if self.stats.current_streak > 0 {
                "🔥"
            } else {
                degraded_symbol
            }, // Example: fire emoji or theme symbol
            "CURRENT STREAK",
            self.stats.current_streak
        );
        let avg_exec_time_str = format!(
            "{} {}: {:.1} {}",
            self.theme.symbols().status_indicators().neutral, // Example
            "AVG EXECUTION TIME",
            self.stats.avg_execution_time,
            "MS"
        );

        let learning_stats = format!(
            "{} LEARNING STATISTICS\n\n{}\n{}\n{}\n{}\n{}",
            diamond_symbol,
            problems_attempted_str,
            solutions_accepted_str,
            success_rate_str,
            current_streak_str,
            avg_exec_time_str
        ); // EvaFormat replaced

        let (is_ok, is_warning) = self.get_operational_status();
        let learning_border = if !is_ok && !is_warning {
            self.theme.borders().error_block()
        } else if is_warning {
            self.theme.borders().warning_block()
        } else {
            self.theme.borders().operational_block() // Assuming operational_block for success
        }; // EvaBorders replaced

        let learning_widget = Paragraph::new(learning_stats)
            .style(self.theme.styles().text_primary()) // EvaStyles replaced
            .block(learning_border.title("LEARNING UNIT STATUS")); // EvaFormat replaced
        f.render_widget(learning_widget, chunks[0]);

        // Right panel - System status
        let triangle_symbol = self
            .theme
            .symbols()
            .geometric_shapes()
            .get(2)
            .unwrap_or(&"")
            .to_string(); // EvaSymbols replaced
        let hexagon_symbol = self
            .theme
            .symbols()
            .geometric_shapes()
            .get(0)
            .unwrap_or(&"")
            .to_string(); // Re-using hexagon

        let system_stats = if let Some(cost) = self.cost_analytics {
            let total_cost_str = format!("{}: ${:.4} {}", "TOTAL COST", cost.total_cost_usd, "USD");
            let tokens_used_str = format!("{}: {}", "TOKENS USED", cost.tokens_used);
            let api_requests_str = format!("{}: {}", "API REQUESTS", cost.requests_count);
            let field_status_str = self.hex_display("FIELD STABLE"); // Kept hex_display for now

            format!(
                "{} SYSTEM RESOURCES\n\n{}\n{}\n{}\n\n{} AT FIELD STATUS\n{}",
                triangle_symbol,
                total_cost_str,
                tokens_used_str,
                api_requests_str,
                hexagon_symbol,
                field_status_str
            )
        } else {
            let resources_str = format!("{}: {}", "RESOURCES", "LOADING");
            let field_status_str = self.hex_display("FIELD UNKNOWN");
            format!(
                "{} SYSTEM RESOURCES\n\n{}\n\n{} AT FIELD STATUS\n{}",
                triangle_symbol,
                resources_str, // EvaFormat replaced
                hexagon_symbol,
                field_status_str
            )
        }; // EvaFormat replaced

        let system_widget = Paragraph::new(system_stats)
            .style(self.theme.styles().text_secondary()) // EvaStyles replaced
            .block(self.theme.borders().default_block().title("NERV SYSTEMS")); // EvaBorders and EvaFormat replaced
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
        let square_symbol = self
            .theme
            .symbols()
            .geometric_shapes()
            .get(3)
            .unwrap_or(&"")
            .to_string(); // EvaSymbols replaced
        let mut topic_text = format!("{} ALGORITHM CATEGORIES\n\n", square_symbol);
        for (topic, count) in self.stats.topic_distribution.iter().take(5) {
            topic_text.push_str(&format!("{} {}: {}\n", "→", topic, count)); // Using "→" as a simple list item indicator
        }

        let topics_widget = Paragraph::new(topic_text)
            .style(self.theme.styles().text_secondary()) // EvaStyles replaced
            .block(
                self.theme
                    .borders()
                    .default_block()
                    .title("ALGORITHM TYPES"),
            ); // EvaBorders and EvaFormat replaced
        f.render_widget(topics_widget, detail_chunks[0]);

        // Difficulty distribution
        let warning_symbol = self.theme.symbols().status_indicators().warning.clone(); // EvaSymbols replaced
        let mut difficulty_text = format!("{} COMPLEXITY LEVELS\n\n", warning_symbol);
        for (difficulty, count) in self.stats.difficulty_distribution.iter().take(3) {
            difficulty_text.push_str(&format!("{} {}: {}\n", "→", difficulty, count));
        }

        let difficulty_widget = Paragraph::new(difficulty_text)
            .style(self.theme.styles().text_secondary()) // EvaStyles replaced
            .block(
                self.theme
                    .borders()
                    .default_block()
                    .title("DIFFICULTY LEVELS"),
            ); // EvaBorders and EvaFormat replaced
        f.render_widget(difficulty_widget, detail_chunks[1]);
    }
}

impl<'a> Widget for ThemedProgressWidget<'a> {
    // Renamed struct
    fn render(&self, f: &mut Frame, area: Rect) {
        if self.show_detailed {
            self.render_detailed_view(f, area);
        } else {
            self.render_main_display(f, area);
        }
    }

    fn title(&self) -> Option<&str> {
        Some("MAGI SYSTEM") // Title can remain generic or be themed if needed
    }

    fn border_style(&self) -> Style {
        let (is_ok, is_warning) = self.get_operational_status();
        if !is_ok && !is_warning {
            Style::default().fg(self.theme.colors().error()) // EvaColors replaced
        } else if is_warning {
            Style::default().fg(self.theme.colors().warning()) // EvaColors replaced
        } else {
            Style::default().fg(self.theme.colors().success()) // EvaColors replaced
        }
    }
}
