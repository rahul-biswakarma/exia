use super::{Theme, Widget};
use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Paragraph},
    Frame,
};

pub struct CornerDecorationWidget<'a> {
    theme: &'a dyn Theme,
    show_decorations: bool,
}

impl<'a> CornerDecorationWidget<'a> {
    pub fn new(theme: &'a dyn Theme) -> Self {
        Self {
            theme,
            show_decorations: true,
        }
    }

    pub fn with_decorations(mut self, show: bool) -> Self {
        self.show_decorations = show;
        self
    }

    fn render_corner_decoration(&self, f: &mut Frame, area: Rect, position: CornerPosition) {
        if !self.show_decorations || area.width < 3 || area.height < 3 {
            return;
        }

        let decoration = self.theme.symbols().corner_decoration();
        let shapes = self.theme.symbols().geometric_shapes();

        let (x, y, content) = match position {
            CornerPosition::TopLeft => (area.x, area.y, format!("{}", decoration)),
            CornerPosition::TopRight => (area.x + area.width - 1, area.y, format!("{}", shapes[0])),
            CornerPosition::BottomLeft => {
                (area.x, area.y + area.height - 1, format!("{}", shapes[1]))
            }
            CornerPosition::BottomRight => (
                area.x + area.width - 1,
                area.y + area.height - 1,
                format!("{}", shapes[2]),
            ),
        };

        let decoration_area = Rect {
            x,
            y,
            width: 1,
            height: 1,
        };

        let decoration_widget = Paragraph::new(content)
            .style(self.theme.styles().text_highlight())
            .alignment(Alignment::Center);

        f.render_widget(decoration_widget, decoration_area);
    }

    pub fn render_all_corners(&self, f: &mut Frame, area: Rect) {
        self.render_corner_decoration(f, area, CornerPosition::TopLeft);
        self.render_corner_decoration(f, area, CornerPosition::TopRight);
        self.render_corner_decoration(f, area, CornerPosition::BottomLeft);
        self.render_corner_decoration(f, area, CornerPosition::BottomRight);
    }
}

#[derive(Debug, Clone, Copy)]
enum CornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl<'a> Widget for CornerDecorationWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        self.render_all_corners(f, area);
    }

    fn title(&self) -> Option<&str> {
        None
    }

    fn border_style(&self) -> Style {
        self.theme.styles().text_primary()
    }

    fn has_borders(&self) -> bool {
        false
    }
}

/// Enhanced block widget with corner decorations
pub struct DecoratedBlock<'a> {
    theme: &'a dyn Theme,
    title: Option<String>,
    show_corners: bool,
}

impl<'a> DecoratedBlock<'a> {
    pub fn new(theme: &'a dyn Theme) -> Self {
        Self {
            theme,
            title: None,
            show_corners: true,
        }
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_corners(mut self, show: bool) -> Self {
        self.show_corners = show;
        self
    }

    pub fn render_with_content<F>(&self, f: &mut Frame, area: Rect, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect),
    {
        // Render the main block
        let block = if let Some(ref title) = self.title {
            self.theme.borders().header_block(title)
        } else {
            self.theme.borders().default_block()
        };

        let inner_area = block.inner(area);
        f.render_widget(block, area);

        // Render content inside
        render_content(f, inner_area);

        // Render corner decorations
        if self.show_corners {
            let corner_widget = CornerDecorationWidget::new(self.theme);
            corner_widget.render_all_corners(f, area);
        }
    }
}

impl<'a> Widget for DecoratedBlock<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        self.render_with_content(f, area, |_, _| {});
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn border_style(&self) -> Style {
        self.theme.styles().text_primary()
    }
}
