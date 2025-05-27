use super::syntax_highlighter::RustSyntaxHighlighter;
use super::Widget;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use tui_input::Input;

pub struct CodeEditorWidget<'a> {
    pub input: &'a Input,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    pub show_line_numbers: bool,
    pub language: CodeLanguage,
    pub highlighter: RustSyntaxHighlighter,
}

#[derive(Debug, Clone)]
pub enum CodeLanguage {
    Rust,
    Python,
    JavaScript,
    Other,
}

impl<'a> CodeEditorWidget<'a> {
    pub fn new(input: &'a Input) -> Self {
        Self {
            input,
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
            show_line_numbers: true,
            language: CodeLanguage::Rust,
            highlighter: RustSyntaxHighlighter::default(),
        }
    }

    pub fn with_cursor(mut self, line: usize, col: usize) -> Self {
        self.cursor_line = line;
        self.cursor_col = col;
        self
    }

    pub fn with_scroll(mut self, offset: usize) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn with_language(mut self, language: CodeLanguage) -> Self {
        self.language = language;
        self
    }

    fn get_syntax_highlighted_lines(&self) -> Vec<Line> {
        let code = self.input.value();
        let lines: Vec<&str> = code.lines().collect();

        lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let mut spans = Vec::new();

                // Add line number
                if self.show_line_numbers {
                    let line_num_style = if i == self.cursor_line {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    spans.push(Span::styled(format!("{:3} │ ", i + 1), line_num_style));
                }

                // Add syntax highlighted content
                let highlighted_line = self.highlighter.highlight_line(line);
                spans.extend(highlighted_line.spans);

                // Add cursor indicator if this is the cursor line
                if i == self.cursor_line {
                    // Insert cursor at the correct position
                    self.insert_cursor_in_spans(&mut spans, self.cursor_col);
                }

                Line::from(spans)
            })
            .collect()
    }

    fn insert_cursor_in_spans(&self, spans: &mut Vec<Span>, cursor_col: usize) {
        let line_num_offset = if self.show_line_numbers { 5 } else { 0 }; // "123 │ " = 5 chars
        let target_col = cursor_col + line_num_offset;

        let mut current_col = 0;
        let mut span_index = 0;

        while span_index < spans.len() {
            let span = &spans[span_index];
            let span_len = span.content.chars().count();

            if current_col + span_len > target_col {
                // Cursor is within this span
                let offset_in_span = target_col - current_col;
                let span_content = span.content.as_ref();
                let chars: Vec<char> = span_content.chars().collect();

                if offset_in_span < chars.len() {
                    // Split the span and insert cursor
                    let before: String = chars[..offset_in_span].iter().collect();
                    let after: String = chars[offset_in_span..].iter().collect();

                    let original_style = span.style;
                    spans.remove(span_index);

                    if !before.is_empty() {
                        spans.insert(span_index, Span::styled(before, original_style));
                        span_index += 1;
                    }

                    // Insert cursor
                    spans.insert(
                        span_index,
                        Span::styled("█", Style::default().fg(Color::White).bg(Color::Blue)),
                    );
                    span_index += 1;

                    if !after.is_empty() {
                        spans.insert(span_index, Span::styled(after, original_style));
                    }
                } else {
                    // Cursor is at the end of this span
                    spans.insert(
                        span_index + 1,
                        Span::styled("█", Style::default().fg(Color::White).bg(Color::Blue)),
                    );
                }
                break;
            }

            current_col += span_len;
            span_index += 1;
        }

        // If we didn't find the position, add cursor at the end
        if span_index >= spans.len() {
            spans.push(Span::styled(
                "█",
                Style::default().fg(Color::White).bg(Color::Blue),
            ));
        }
    }

    fn get_editor_info(&self) -> String {
        let lines: Vec<&str> = self.input.value().lines().collect();
        let total_lines = lines.len().max(1);
        let total_chars = self.input.value().len();

        format!(
            "Lines: {} | Chars: {} | Pos: {}:{} | Lang: {:?}",
            total_lines,
            total_chars,
            self.cursor_line + 1,
            self.cursor_col + 1,
            self.language
        )
    }
}

impl<'a> Widget for CodeEditorWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let lines = self.get_syntax_highlighted_lines();

        let block = Block::default()
            .title(format!("🦀 Code Editor - {}", self.get_editor_info()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        let paragraph = Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset as u16, 0))
            .block(block);

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        Some("Code Editor")
    }

    fn border_style(&self) -> Style {
        Style::default().fg(Color::Green)
    }
}

/// Enhanced code editor state management
#[derive(Debug, Clone)]
pub struct CodeEditorState {
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    pub selection_start: Option<(usize, usize)>,
    pub selection_end: Option<(usize, usize)>,
}

impl Default for CodeEditorState {
    fn default() -> Self {
        Self {
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
            selection_start: None,
            selection_end: None,
        }
    }
}

impl CodeEditorState {
    pub fn move_cursor_up(&mut self, lines: &[&str]) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            if self.cursor_line < lines.len() {
                self.cursor_col = self.cursor_col.min(lines[self.cursor_line].len());
            }
        }
    }

    pub fn move_cursor_down(&mut self, lines: &[&str]) {
        if self.cursor_line < lines.len().saturating_sub(1) {
            self.cursor_line += 1;
            if self.cursor_line < lines.len() {
                self.cursor_col = self.cursor_col.min(lines[self.cursor_line].len());
            }
        }
    }

    pub fn move_cursor_left(&mut self, lines: &[&str]) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            if self.cursor_line < lines.len() {
                self.cursor_col = lines[self.cursor_line].len();
            }
        }
    }

    pub fn move_cursor_right(&mut self, lines: &[&str]) {
        if self.cursor_line < lines.len() {
            let line_len = lines[self.cursor_line].len();
            if self.cursor_col < line_len {
                self.cursor_col += 1;
            } else if self.cursor_line < lines.len() - 1 {
                self.cursor_line += 1;
                self.cursor_col = 0;
            }
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_to_line_end(&mut self, lines: &[&str]) {
        if self.cursor_line < lines.len() {
            self.cursor_col = lines[self.cursor_line].len();
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.cursor_line = self.cursor_line.saturating_sub(page_size);
        if self.scroll_offset > page_size {
            self.scroll_offset -= page_size;
        } else {
            self.scroll_offset = 0;
        }
    }

    pub fn page_down(&mut self, lines: &[&str], page_size: usize) {
        self.cursor_line = (self.cursor_line + page_size).min(lines.len().saturating_sub(1));
        self.scroll_offset += page_size;
    }
}
