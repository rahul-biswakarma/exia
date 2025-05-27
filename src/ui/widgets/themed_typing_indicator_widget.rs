use crate::ui::theme::Theme; // Eva* imports removed
use crate::ui::TypingMetrics;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph}, // Added Block for get_border_style return type
    Frame
};

pub struct ThemedTypingIndicatorWidget<'a> { // Renamed struct
    pub metrics: &'a TypingMetrics,
    pub theme: &'a dyn Theme, // Theme is now mandatory
}

impl<'a> ThemedTypingIndicatorWidget<'a> { // Renamed struct
    pub fn new(metrics: &'a TypingMetrics, theme: &'a dyn Theme) -> Self { // Added theme to constructor
        Self {
            metrics,
            theme, // Theme initialized
        }
    }

    // with_theme method removed

    fn get_coder_performance_level(&self) -> (&'static str, Style) {
        let styles = self.theme.styles();
        match self.metrics.current_wpm as u32 {
            0..=20 => ("NOVICE CODER", styles.text_error()),
            21..=40 => ("JUNIOR DEVELOPER", styles.text_warning()),
            41..=60 => ("SOFTWARE ENGINEER", styles.text_success()),
            61..=80 => ("SENIOR DEVELOPER", styles.text_highlight()), // Assuming sync_rate maps to text_highlight
            _ => ("ALGORITHM ARCHITECT", styles.text_info()),      // Assuming at_field maps to text_info
        }
        // EvaStyles removed
    }

    fn get_coding_efficiency(&self) -> f64 {
        // Convert WPM to a coding efficiency percentage (0-100%)
        (self.metrics.current_wpm / 100.0 * 100.0).min(100.0)
    }

    fn get_interface_status(&self) -> &'static str {
        let efficiency = self.get_coding_efficiency();
        match efficiency as u32 {
            0..=30 => "INITIALIZING",
            31..=50 => "CONNECTING",
            51..=70 => "SYNCHRONIZED",
            71..=90 => "OPTIMIZED",
            _ => "PEAK PERFORMANCE",
        }
    }
    
    // Helper for readout formatting, using theme styles
    fn format_readout(&self, label: &str, value: &str, unit: &str) -> String {
        // This is a simplified version of EvaFormat::readout
        // Styling will be applied at the Paragraph level or via Spans if needed
        format!("{}: {} {}", label, value, unit)
    }


    fn format_display(&self) -> String {
        let (coder_level, _) = self.get_coder_performance_level();
        let coding_efficiency = self.get_coding_efficiency();
        let interface_status = self.get_interface_status();
        let efficiency_symbol = self.get_themed_efficiency_symbol(coding_efficiency);

        let hexagon_symbol = self.theme.symbols().geometric_shapes().get(0).cloned().unwrap_or_default();
        let diamond_symbol = self.theme.symbols().geometric_shapes().get(1).cloned().unwrap_or_default();
        // EvaSymbols removed

        format!(
            "{} CODING INTERFACE STATUS\n\n{}\n{}\n{}\n{}\n\n{} DEVELOPER LEVEL: {}\n{} INTERFACE: {}",
            hexagon_symbol,
            self.format_readout("CURRENT WPM", &format!("{:.1}", self.metrics.current_wpm), ""),
            self.format_readout("AVERAGE WPM", &format!("{:.1}", self.metrics.average_wpm), ""),
            self.format_readout("TOTAL CHARS", &self.metrics.total_characters.to_string(), ""),
            self.format_readout("SESSION TIME", &format!("{:.1}", self.metrics.total_time_ms as f64 / 1000.0), "SEC"),
            diamond_symbol,
            coder_level,
            efficiency_symbol,
            interface_status
        )
        // EvaFormat removed
    }

    fn get_themed_efficiency_symbol(&self, efficiency: f64) -> String { // Return type String
        let indicators = self.theme.symbols().status_indicators();
        match efficiency {
            e if e >= 80.0 => indicators.sync_high.clone(),
            e if e >= 60.0 => indicators.sync_medium.clone(),
            e if e >= 40.0 => indicators.sync_low.clone(),
            _ => indicators.sync_critical.clone(),
        }
        // EvaSymbols removed
    }

    fn get_border_style(&self) -> Block<'static> { // Return type changed to Block
        let efficiency = self.get_coding_efficiency();
        if efficiency >= 70.0 {
            self.theme.borders().operational_block() // Assuming operational_block for success
        } else if efficiency >= 40.0 {
            self.theme.borders().warning_block()
        } else {
            self.theme.borders().error_block()
        }
        // EvaBorders removed
    }
}

impl<'a> Widget for ThemedTypingIndicatorWidget<'a> { // Renamed struct
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.format_display();
        let (_, coder_style) = self.get_coder_performance_level();

        let paragraph = Paragraph::new(content).style(coder_style).block(
            self.get_border_style()
                .title(self.theme.formats().title("CODING TERMINAL INTERFACE")), // EvaFormat replaced
        );

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("CODING ACTIVITY") // Title can be generic or themed
    }

    fn border_style(&self) -> Style {
        let efficiency = self.get_coding_efficiency();
        if efficiency >= 70.0 {
            Style::default().fg(self.theme.colors().success()) // EvaColors replaced
        } else if efficiency >= 40.0 {
            Style::default().fg(self.theme.colors().warning()) // EvaColors replaced
        } else {
            Style::default().fg(self.theme.colors().error()) // EvaColors replaced
        }
    }
}
