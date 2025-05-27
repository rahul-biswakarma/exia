use super::{EvaBorders, EvaColors, EvaFormat, EvaStyles, EvaSymbols, Theme, Widget};
use crate::models::CostAnalytics;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Axis, Chart, Dataset, GraphType, Paragraph},
    Frame,
};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub ram_total: f64,
    pub timestamp: std::time::Instant,
}

pub struct SystemMonitorWidget<'a> {
    pub cost_analytics: Option<&'a CostAnalytics>,
    pub theme: Option<&'a dyn Theme>,
    pub cpu_history: &'a VecDeque<(f64, f64)>, // (time, cpu_usage)
    pub ram_history: &'a VecDeque<(f64, f64)>, // (time, ram_usage)
    pub current_metrics: Option<&'a SystemMetrics>,
}

impl<'a> SystemMonitorWidget<'a> {
    pub fn new(
        cost_analytics: Option<&'a CostAnalytics>,
        cpu_history: &'a VecDeque<(f64, f64)>,
        ram_history: &'a VecDeque<(f64, f64)>,
        current_metrics: Option<&'a SystemMetrics>,
    ) -> Self {
        Self {
            cost_analytics,
            theme: None,
            cpu_history,
            ram_history,
            current_metrics,
        }
    }

    pub fn with_theme(mut self, theme: &'a dyn Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    fn render_system_resources(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Cost info
                Constraint::Min(0),    // System graphs
            ])
            .split(area);

        // Cost information
        let system_stats = if let Some(cost) = self.cost_analytics {
            format!(
                "{} SYSTEM RESOURCES\n\n{}\n{}\n{}",
                EvaSymbols::TRIANGLE,
                EvaFormat::readout("TOTAL COST", &format!("${:.4}", cost.total_cost_usd), "USD"),
                EvaFormat::readout("TOKENS USED", &cost.tokens_used.to_string(), ""),
                EvaFormat::readout("API REQUESTS", &cost.requests_count.to_string(), ""),
            )
        } else {
            format!(
                "{} SYSTEM RESOURCES\n\n{}",
                EvaSymbols::TRIANGLE,
                EvaFormat::status("RESOURCES", "LOADING", false),
            )
        };

        let cost_widget = Paragraph::new(system_stats)
            .style(EvaStyles::text_secondary())
            .block(EvaBorders::panel().title(EvaFormat::title("COST ANALYTICS")));
        f.render_widget(cost_widget, chunks[0]);

        // System performance graphs
        self.render_performance_graphs(f, chunks[1]);
    }

    fn render_performance_graphs(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // CPU graph
                Constraint::Percentage(50), // RAM graph
            ])
            .split(area);

        // CPU Usage Graph
        self.render_cpu_graph(f, chunks[0]);

        // RAM Usage Graph
        self.render_ram_graph(f, chunks[1]);
    }

    fn render_cpu_graph(&self, f: &mut Frame, area: Rect) {
        let cpu_data: Vec<(f64, f64)> = self.cpu_history.iter().cloned().collect();

        let current_cpu = self.current_metrics.map(|m| m.cpu_usage).unwrap_or(0.0);

        let cpu_color = if current_cpu > 80.0 {
            EvaColors::STATUS_CRITICAL
        } else if current_cpu > 60.0 {
            EvaColors::STATUS_WARNING
        } else {
            EvaColors::STATUS_NORMAL
        };

        let datasets = vec![Dataset::default()
            .name("CPU")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(cpu_color))
            .data(&cpu_data)];

        let x_bounds = if cpu_data.is_empty() {
            [0.0, 60.0]
        } else {
            let min_x = cpu_data
                .iter()
                .map(|(x, _)| *x)
                .fold(f64::INFINITY, f64::min);
            let max_x = cpu_data
                .iter()
                .map(|(x, _)| *x)
                .fold(f64::NEG_INFINITY, f64::max);
            [min_x, max_x.max(min_x + 1.0)]
        };

        // Dynamic Y-axis bounds for better visualization
        let y_bounds = if cpu_data.is_empty() {
            [0.0, 100.0]
        } else {
            let min_y = cpu_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::INFINITY, f64::min);
            let max_y = cpu_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::NEG_INFINITY, f64::max);

            // Add padding around the data for better visualization
            let range = (max_y - min_y).max(15.0); // Minimum 15% range for CPU
            let padding = range * 0.3; // 30% padding for CPU (more dynamic)
            let lower_bound = (min_y - padding).max(0.0);
            let upper_bound = (max_y + padding).min(100.0);

            [lower_bound, upper_bound]
        };

        let chart = Chart::new(datasets)
            .block(
                EvaBorders::operational()
                    .title(EvaFormat::title(&format!("CPU USAGE: {:.1}%", current_cpu))),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(x_bounds),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(y_bounds),
            );

        f.render_widget(chart, area);
    }

    fn render_ram_graph(&self, f: &mut Frame, area: Rect) {
        let ram_data: Vec<(f64, f64)> = self.ram_history.iter().cloned().collect();

        let current_ram = self
            .current_metrics
            .map(|m| (m.ram_usage / m.ram_total) * 100.0)
            .unwrap_or(0.0);

        let ram_color = if current_ram > 80.0 {
            EvaColors::STATUS_CRITICAL
        } else if current_ram > 60.0 {
            EvaColors::STATUS_WARNING
        } else {
            Color::Cyan
        };

        let datasets = vec![Dataset::default()
            .name("RAM")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(ram_color))
            .data(&ram_data)];

        let x_bounds = if ram_data.is_empty() {
            [0.0, 60.0]
        } else {
            let min_x = ram_data
                .iter()
                .map(|(x, _)| *x)
                .fold(f64::INFINITY, f64::min);
            let max_x = ram_data
                .iter()
                .map(|(x, _)| *x)
                .fold(f64::NEG_INFINITY, f64::max);
            [min_x, max_x.max(min_x + 1.0)]
        };

        // Dynamic Y-axis bounds for better visualization
        let y_bounds = if ram_data.is_empty() {
            [0.0, 100.0]
        } else {
            let min_y = ram_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::INFINITY, f64::min);
            let max_y = ram_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::NEG_INFINITY, f64::max);

            // Add padding around the data for better visualization
            let range = (max_y - min_y).max(10.0); // Minimum 10% range
            let padding = range * 0.2; // 20% padding
            let lower_bound = (min_y - padding).max(0.0);
            let upper_bound = (max_y + padding).min(100.0);

            [lower_bound, upper_bound]
        };

        let chart = Chart::new(datasets)
            .block(
                EvaBorders::operational()
                    .title(EvaFormat::title(&format!("RAM USAGE: {:.1}%", current_ram))),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(x_bounds),
            )
            .y_axis(
                Axis::default()
                    .title("Usage %")
                    .style(Style::default().fg(Color::Gray))
                    .bounds(y_bounds),
            );

        f.render_widget(chart, area);
    }
}

impl<'a> Widget for SystemMonitorWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        self.render_system_resources(f, area);
    }

    fn title(&self) -> Option<&str> {
        Some("SYSTEM MONITOR")
    }

    fn border_style(&self) -> Style {
        let current_cpu = self.current_metrics.map(|m| m.cpu_usage).unwrap_or(0.0);
        let current_ram = self
            .current_metrics
            .map(|m| (m.ram_usage / m.ram_total) * 100.0)
            .unwrap_or(0.0);

        if current_cpu > 80.0 || current_ram > 80.0 {
            Style::default().fg(EvaColors::STATUS_CRITICAL)
        } else if current_cpu > 60.0 || current_ram > 60.0 {
            Style::default().fg(EvaColors::STATUS_WARNING)
        } else {
            Style::default().fg(EvaColors::STATUS_NORMAL)
        }
    }
}
