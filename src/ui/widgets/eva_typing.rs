use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Widget};
use crate::ui::TypingMetrics;
use ratatui::{layout::Rect, style::Style, widgets::Paragraph, Frame};

pub struct EvaTypingWidget<'a> {
    pub metrics: &'a TypingMetrics,
}

impl<'a> EvaTypingWidget<'a> {
    pub fn new(metrics: &'a TypingMetrics) -> Self {
        Self { metrics }
    }

    fn get_coder_performance_level(&self) -> (&'static str, Style) {
        match self.metrics.current_wpm as u32 {
            0..=20 => ("NOVICE CODER", EvaStyles::text_error()),
            21..=40 => ("JUNIOR DEVELOPER", EvaStyles::text_warning()),
            41..=60 => ("SOFTWARE ENGINEER", EvaStyles::text_success()),
            61..=80 => ("SENIOR DEVELOPER", EvaStyles::sync_rate()),
            _ => ("ALGORITHM ARCHITECT", EvaStyles::at_field()),
        }
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

    fn format_display(&self) -> String {
        let (coder_level, _) = self.get_coder_performance_level();
        let coding_efficiency = self.get_coding_efficiency();
        let interface_status = self.get_interface_status();
        let efficiency_symbol = EvaSymbols::sync_rate_symbol(coding_efficiency);

        format!(
            "{} CODING INTERFACE STATUS\n\n{}\n{}\n{}\n{}\n\n{} DEVELOPER LEVEL: {}\n{} INTERFACE: {}",
            EvaSymbols::HEXAGON,
            EvaFormat::readout("CURRENT WPM", &format!("{:.1}", self.metrics.current_wpm), ""),
            EvaFormat::readout("AVERAGE WPM", &format!("{:.1}", self.metrics.average_wpm), ""),
            EvaFormat::readout("TOTAL CHARS", &self.metrics.total_characters.to_string(), ""),
            EvaFormat::readout("SESSION TIME", &format!("{:.1}", self.metrics.total_time_ms as f64 / 1000.0), "SEC"),
            EvaSymbols::DIAMOND,
            coder_level,
            efficiency_symbol,
            interface_status
        )
    }

    fn get_border_style(&self) -> ratatui::widgets::Block<'static> {
        let efficiency = self.get_coding_efficiency();
        if efficiency >= 70.0 {
            EvaBorders::operational()
        } else if efficiency >= 40.0 {
            EvaBorders::warning()
        } else {
            EvaBorders::error()
        }
    }
}

impl<'a> Widget for EvaTypingWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.format_display();
        let (_, coder_style) = self.get_coder_performance_level();

        let paragraph = Paragraph::new(content).style(coder_style).block(
            self.get_border_style()
                .title(EvaFormat::title("CODING TERMINAL INTERFACE")),
        );

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("CODING TERMINAL")
    }

    fn border_style(&self) -> Style {
        let efficiency = self.get_coding_efficiency();
        if efficiency >= 70.0 {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        } else if efficiency >= 40.0 {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_CRITICAL)
        }
    }
}
