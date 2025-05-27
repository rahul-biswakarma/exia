use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme, Widget};
use crate::storage::Statistics;
use ratatui::{layout::Rect, style::Style, widgets::Paragraph, Frame};

pub struct LearningUnitStatusWidget<'a> {
    pub stats: &'a Statistics,
    pub theme: Option<&'a dyn Theme>,
}

impl<'a> LearningUnitStatusWidget<'a> {
    pub fn new(stats: &'a Statistics) -> Self {
        Self { stats, theme: None }
    }

    pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    fn get_operational_status(&self) -> (bool, bool) {
        let success_rate = self.stats.success_rate;
        let is_ok = success_rate >= 70.0;
        let is_warning = success_rate >= 40.0 && success_rate < 70.0;
        (is_ok, is_warning)
    }
}

impl<'a> Widget for LearningUnitStatusWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let learning_stats = format!(
            "{} LEARNING STATISTICS\n\n{}\n{}\n{}\n{}\n{}",
            EvaSymbols::DIAMOND,
            EvaFormat::status(
                "PROBLEMS ATTEMPTED",
                &self.stats.total_questions.to_string(),
                true
            ),
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
            EvaBorders::error()
        } else if is_warning {
            EvaBorders::warning()
        } else {
            EvaBorders::operational()
        };

        let learning_widget = Paragraph::new(learning_stats)
            .style(EvaStyles::text_primary())
            .block(learning_border.title(EvaFormat::title("LEARNING UNIT STATUS")));

        f.render_widget(learning_widget, area);
    }

    fn title(&self) -> Option<&str> {
        Some("LEARNING UNIT STATUS")
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
