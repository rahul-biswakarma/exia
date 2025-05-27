use std::cmp;

/// A proper text editor that handles cursor positioning and text modification
#[derive(Debug, Clone)]
pub struct TextEditor {
    content: String,
    cursor_line: usize,
    cursor_col: usize,
    scroll_offset: usize,
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            content: String::new(),
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }
}

impl TextEditor {
    pub fn new(content: String) -> Self {
        Self {
            content,
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_line, self.cursor_col)
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    pub fn line_count(&self) -> usize {
        self.content.lines().count().max(1)
    }

    pub fn char_count(&self) -> usize {
        self.content.len()
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.normalize_cursor();
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0;
    }

    // Cursor movement methods
    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.normalize_cursor_col();
            self.update_scroll();
        }
    }

    pub fn move_cursor_down(&mut self) {
        let lines = self.lines();
        if self.cursor_line < lines.len().saturating_sub(1) {
            self.cursor_line += 1;
            self.normalize_cursor_col();
            self.update_scroll();
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let lines = self.lines();
            if self.cursor_line < lines.len() {
                self.cursor_col = lines[self.cursor_line].len();
            }
            self.update_scroll();
        }
    }

    pub fn move_cursor_right(&mut self) {
        let lines = self.lines();
        if self.cursor_line < lines.len() {
            let line_len = lines[self.cursor_line].len();
            if self.cursor_col < line_len {
                self.cursor_col += 1;
            } else if self.cursor_line < lines.len() - 1 {
                self.cursor_line += 1;
                self.cursor_col = 0;
                self.update_scroll();
            }
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    pub fn move_to_line_end(&mut self) {
        let lines = self.lines();
        if self.cursor_line < lines.len() {
            self.cursor_col = lines[self.cursor_line].len();
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.cursor_line = self.cursor_line.saturating_sub(page_size);
        self.normalize_cursor_col();
        if self.scroll_offset > page_size {
            self.scroll_offset -= page_size;
        } else {
            self.scroll_offset = 0;
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        let lines = self.lines();
        self.cursor_line = (self.cursor_line + page_size).min(lines.len().saturating_sub(1));
        self.normalize_cursor_col();
        self.scroll_offset += page_size;
    }

    // Text modification methods
    pub fn insert_char(&mut self, ch: char) {
        let cursor_pos = self.get_cursor_byte_position();
        self.content.insert(cursor_pos, ch);

        if ch == '\n' {
            self.cursor_line += 1;
            self.cursor_col = 0;
        } else {
            self.cursor_col += 1;
        }

        self.update_scroll();
    }

    pub fn insert_str(&mut self, s: &str) {
        for ch in s.chars() {
            self.insert_char(ch);
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_col > 0 {
            // Delete character before cursor
            self.cursor_col -= 1;
            let cursor_pos = self.get_cursor_byte_position();
            if cursor_pos < self.content.len() {
                self.content.remove(cursor_pos);
            }
        } else if self.cursor_line > 0 {
            // Delete newline, merge with previous line
            let line_len = {
                let lines = self.lines();
                if self.cursor_line > 0 && self.cursor_line - 1 < lines.len() {
                    lines[self.cursor_line - 1].len()
                } else {
                    0
                }
            };

            self.cursor_line -= 1;
            self.cursor_col = line_len;
            let cursor_pos = self.get_cursor_byte_position();
            if cursor_pos < self.content.len() {
                self.content.remove(cursor_pos);
            }
        }
        self.update_scroll();
    }

    pub fn delete_forward(&mut self) {
        let cursor_pos = self.get_cursor_byte_position();
        if cursor_pos < self.content.len() {
            self.content.remove(cursor_pos);
        }
    }

    // Helper methods
    fn get_cursor_byte_position(&self) -> usize {
        let lines = self.lines();
        let mut pos = 0;

        // Add bytes for all lines before cursor line
        for i in 0..self.cursor_line.min(lines.len()) {
            pos += lines[i].len() + 1; // +1 for newline
        }

        // Add bytes for characters before cursor in current line
        if self.cursor_line < lines.len() {
            let line = lines[self.cursor_line];
            let chars: Vec<char> = line.chars().collect();
            for i in 0..self.cursor_col.min(chars.len()) {
                pos += chars[i].len_utf8();
            }
        }

        pos.min(self.content.len())
    }

    fn normalize_cursor(&mut self) {
        let lines = self.lines();
        self.cursor_line = self.cursor_line.min(lines.len().saturating_sub(1));
        self.normalize_cursor_col();
        self.update_scroll();
    }

    fn normalize_cursor_col(&mut self) {
        let lines = self.lines();
        if self.cursor_line < lines.len() {
            self.cursor_col = self.cursor_col.min(lines[self.cursor_line].len());
        }
    }

    fn update_scroll(&mut self) {
        const VISIBLE_LINES: usize = 20;

        if self.cursor_line >= self.scroll_offset + VISIBLE_LINES {
            self.scroll_offset = self.cursor_line.saturating_sub(VISIBLE_LINES - 1);
        } else if self.cursor_line < self.scroll_offset {
            self.scroll_offset = self.cursor_line;
        }
    }
}
