use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme, Widget};
use crate::storage::Statistics;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Paragraph,
    Frame,
};

pub struct LearningEfficiencyWidget<'a> {
    pub stats: &'a Statistics,
    pub theme: Option<&'a dyn Theme>,
}

impl<'a> LearningEfficiencyWidget<'a> {
    pub fn new(stats: &'a Statistics) -> Self {
        Self { stats, theme: None }
    }

    pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    fn get_sync_rate(&self) -> f64 {
        self.stats.success_rate
    }

    fn get_themed_sync_symbol(&self, rate: f64) -> &'static str {
        if let Some(theme) = self.theme {
            let indicators = theme.symbols().status_indicators();
            match rate {
                r if r >= 80.0 => indicators.sync_high,
                r if r >= 60.0 => indicators.sync_medium,
                r if r >= 40.0 => indicators.sync_low,
                _ => indicators.sync_critical,
            }
        } else {
            EvaSymbols::sync_rate_symbol(rate)
        }
    }

    fn get_themed_progress_bar(&self, progress: f64, width: usize) -> String {
        if let Some(theme) = self.theme {
            let progress_symbols = theme.symbols().progress_symbols();
            let filled = (progress * width as f64) as usize;
            let empty_symbol = progress_symbols.last().unwrap_or(&" ");
            let fill_symbol = progress_symbols.first().unwrap_or(&"█");

            format!(
                "{}{}",
                fill_symbol.repeat(filled),
                empty_symbol.repeat(width - filled)
            )
        } else {
            EvaFormat::progress_bar(progress, width)
        }
    }
}

impl<'a> Widget for LearningEfficiencyWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Sync rate display
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
        let sync_symbol = self.get_themed_sync_symbol(sync_rate);
        let progress_bar = self.get_themed_progress_bar(sync_rate / 100.0, 20);
        let operational_symbol = if let Some(theme) = self.theme {
            theme.symbols().operational()
        } else {
            EvaSymbols::OPERATIONAL
        };

        let sync_text = format!(
            "{} LEARNING EFFICIENCY RATE: {:.1}%\n{} {}",
            operational_symbol, sync_rate, sync_symbol, progress_bar
        );

        let sync_style = if sync_rate >= 80.0 {
            EvaStyles::sync_rate()
        } else if sync_rate >= 60.0 {
            EvaStyles::text_warning()
        } else {
            EvaStyles::text_error()
        };

        let sync_widget = Paragraph::new(sync_text)
            .style(sync_style)
            .block(EvaBorders::operational());
        f.render_widget(sync_widget, chunks[1]);
    }

    fn title(&self) -> Option<&str> {
        Some("LEARNING EFFICIENCY")
    }

    fn border_style(&self) -> Style {
        let sync_rate = self.get_sync_rate();
        if sync_rate >= 80.0 {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        } else if sync_rate >= 60.0 {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_CRITICAL)
        }
    }
}
