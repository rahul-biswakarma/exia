use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme, Widget};
use crate::storage::Statistics;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame
};

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

    fn get_success_rate_color(&self) -> Color {
        match self.stats.success_rate {
            rate if rate >= 90.0 => EvaColors::STATUS_NORMAL,
            rate if rate >= 70.0 => Color::Cyan,
            rate if rate >= 50.0 => EvaColors::STATUS_WARNING,
            _ => EvaColors::STATUS_CRITICAL,
        }
    }

    fn get_streak_color(&self) -> Color {
        match self.stats.current_streak {
            streak if streak >= 10 => EvaColors::STATUS_NORMAL,
            streak if streak >= 5 => Color::Cyan,
            streak if streak >= 1 => Color::Yellow,
            _ => Color::Gray,
        }
    }

    fn get_problems_color(&self) -> Color {
        match self.stats.total_questions {
            count if count >= 50 => EvaColors::STATUS_NORMAL,
            count if count >= 20 => Color::Cyan,
            count if count >= 10 => Color::Yellow,
            _ => Color::White,
        }
    }

    fn get_solutions_color(&self) -> Color {
        match self.stats.accepted_solutions {
            count if count >= 30 => EvaColors::STATUS_NORMAL,
            count if count >= 15 => Color::Cyan,
            count if count >= 5 => Color::Yellow,
            _ => Color::White,
        }
    }

    fn get_execution_time_color(&self) -> Color {
        match self.stats.avg_execution_time {
            time if time <= 100.0 => EvaColors::STATUS_NORMAL,
            time if time <= 500.0 => Color::Cyan,
            time if time <= 1000.0 => Color::Yellow,
            _ => EvaColors::STATUS_WARNING,
        }
    }
}

impl<'a> Widget for LearningUnitStatusWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        // Create colorful lines with spans
        let header_line = Line::from(vec![
            Span::styled(
                format!("{} LEARNING STATISTICS", EvaSymbols::DIAMOND),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]);

        let problems_line = Line::from(vec![
            Span::styled("PROBLEMS ATTEMPTED: ", Style::default().fg(Color::Gray)),
            Span::styled(
                self.stats.total_questions.to_string(),
                Style::default()
                    .fg(self.get_problems_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let solutions_line = Line::from(vec![
            Span::styled("SOLUTIONS ACCEPTED: ", Style::default().fg(Color::Gray)),
            Span::styled(
                self.stats.accepted_solutions.to_string(),
                Style::default()
                    .fg(self.get_solutions_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let success_rate_line = Line::from(vec![
            Span::styled("SUCCESS RATE: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}%", self.stats.success_rate),
                Style::default()
                    .fg(self.get_success_rate_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let streak_line = Line::from(vec![
            Span::styled("CURRENT STREAK: ", Style::default().fg(Color::Gray)),
            Span::styled(
                self.stats.current_streak.to_string(),
                Style::default()
                    .fg(self.get_streak_color())
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let execution_time_line = Line::from(vec![
            Span::styled("AVG EXECUTION TIME: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}", self.stats.avg_execution_time),
                Style::default()
                    .fg(self.get_execution_time_color())
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" MS", Style::default().fg(Color::Gray)),
        ]);

        // Combine all lines
        let learning_stats = vec![
            header_line,
            Line::from(""), // Empty line
            problems_line,
            solutions_line,
            success_rate_line,
            streak_line,
            execution_time_line,
        ];

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
