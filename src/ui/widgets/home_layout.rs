use super::{
    LLMInfoWidget, LearningEfficiencyWidget, LearningUnitStatusWidget, SystemMonitorWidget, Widget,
};
use crate::models::CostAnalytics;
use crate::storage::Statistics;
use crate::ui::widgets::{LLMStreamInfo, SystemMetrics};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use std::collections::VecDeque;

pub struct HomeLayoutWidget<'a> {
    pub stats: &'a Statistics,
    pub cost_analytics: Option<&'a CostAnalytics>,
    pub llm_stream_info: Option<&'a LLMStreamInfo>,
    pub system_metrics: Option<&'a SystemMetrics>,
    pub cpu_history: &'a VecDeque<(f64, f64)>,
    pub ram_history: &'a VecDeque<(f64, f64)>,
    pub exia_operations_widget: Option<Box<dyn Widget + 'a>>,
    pub animation_frame: usize,
}

impl<'a> HomeLayoutWidget<'a> {
    pub fn new(
        stats: &'a Statistics,
        cpu_history: &'a VecDeque<(f64, f64)>,
        ram_history: &'a VecDeque<(f64, f64)>,
    ) -> Self {
        Self {
            stats,
            cost_analytics: None,
            llm_stream_info: None,
            system_metrics: None,
            cpu_history,
            ram_history,
            exia_operations_widget: None,
            animation_frame: 0,
        }
    }

    pub fn with_cost_analytics(mut self, cost_analytics: Option<&'a CostAnalytics>) -> Self {
        self.cost_analytics = cost_analytics;
        self
    }

    pub fn with_llm_stream_info(mut self, llm_stream_info: Option<&'a LLMStreamInfo>) -> Self {
        self.llm_stream_info = llm_stream_info;
        self
    }

    pub fn with_system_metrics(mut self, system_metrics: Option<&'a SystemMetrics>) -> Self {
        self.system_metrics = system_metrics;
        self
    }

    pub fn with_exia_operations(mut self, widget: Box<dyn Widget + 'a>) -> Self {
        self.exia_operations_widget = Some(widget);
        self
    }

    pub fn with_animation_frame(mut self, frame: usize) -> Self {
        self.animation_frame = frame;
        self
    }
}

impl<'a> Widget for HomeLayoutWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        // Main layout: 50% for Exia operations, 50% for learning widgets
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Exia operations
                Constraint::Percentage(50), // Learning widgets
            ])
            .split(area);

        // Render Exia operations on the left
        if let Some(ref exia_widget) = self.exia_operations_widget {
            exia_widget.render(f, main_chunks[0]);
        } else {
            // Render a placeholder if no Exia operations widget is provided
            use crate::ui::widgets::{EvaBorders, EvaFormat, EvaStyles, EvaSymbols};
            use ratatui::widgets::Paragraph;

            let placeholder_text = format!(
                "{} EXIA OPERATIONS\n\nSYSTEM STANDBY\nAWAITING COMMANDS...",
                EvaSymbols::HEXAGON
            );
            let placeholder = Paragraph::new(placeholder_text)
                .style(EvaStyles::text_secondary())
                .block(EvaBorders::panel().title(EvaFormat::title("EXIA OPERATIONS")));
            f.render_widget(placeholder, main_chunks[0]);
        }

        // Right side layout for learning widgets
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),  // Learning efficiency widget
                Constraint::Length(12), // Learning unit status + system monitor
                Constraint::Min(0),     // LLM info widget
            ])
            .split(main_chunks[1]);

        // Learning efficiency widget (top right)
        let learning_efficiency = LearningEfficiencyWidget::new(self.stats);
        learning_efficiency.render(f, right_chunks[0]);

        // Middle section: Learning unit status (25%) + System monitor (25%)
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Learning unit status
                Constraint::Percentage(50), // System monitor
            ])
            .split(right_chunks[1]);

        // Learning unit status widget
        let learning_unit_status = LearningUnitStatusWidget::new(self.stats);
        learning_unit_status.render(f, middle_chunks[0]);

        // System monitor widget
        let system_monitor = SystemMonitorWidget::new(
            self.cost_analytics,
            self.cpu_history,
            self.ram_history,
            self.system_metrics,
        );
        system_monitor.render(f, middle_chunks[1]);

        // LLM info widget (bottom right)
        let llm_info =
            LLMInfoWidget::new(self.llm_stream_info).with_animation_frame(self.animation_frame);
        llm_info.render(f, right_chunks[2]);
    }

    fn title(&self) -> Option<&str> {
        Some("EXIA COMMAND CENTER")
    }
}
