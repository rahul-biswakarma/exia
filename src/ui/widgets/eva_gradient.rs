use super::{EvaColors, EvaFormat, Widget};
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};

pub struct EvaGradientWidget;

impl EvaGradientWidget {
    pub fn new() -> Self {
        Self
    }

    /// Render gradient background by drawing multiple horizontal lines
    /// with different background colors to simulate a gradient effect
    fn render_gradient_lines(&self, f: &mut Frame, area: Rect) {
        for y in 0..area.height {
            let line_area = Rect {
                x: area.x,
                y: area.y + y,
                width: area.width,
                height: 1,
            };

            let bg_color = EvaFormat::gradient_bg(y, area.height);
            let gradient_line = Paragraph::new(" ".repeat(area.width as usize))
                .style(Style::default().bg(bg_color))
                .block(Block::default());

            f.render_widget(gradient_line, line_area);
        }
    }
}

impl Widget for EvaGradientWidget {
    fn render(&self, f: &mut Frame, area: Rect) {
        self.render_gradient_lines(f, area);
    }

    fn title(&self) -> Option<&str> {
        None
    }

    fn border_style(&self) -> Style {
        Style::default()
    }
}
