use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use std::collections::HashSet;

pub struct RustSyntaxHighlighter {
    keywords: HashSet<&'static str>,
    types: HashSet<&'static str>,
    literals: HashSet<&'static str>,
}

impl Default for RustSyntaxHighlighter {
    fn default() -> Self {
        let mut keywords = HashSet::new();
        keywords.extend([
            "fn", "let", "mut", "if", "else", "for", "while", "loop", "match", "struct", "enum",
            "impl", "trait", "pub", "use", "mod", "crate", "return", "break", "continue", "const",
            "static", "unsafe", "async", "await", "move", "ref", "where", "super", "self", "Self",
            "as", "in", "extern", "type", "dyn", "box", "yield",
        ]);

        let mut types = HashSet::new();
        types.extend([
            "String", "Vec", "Option", "Result", "Some", "None", "Ok", "Err", "i8", "i16", "i32",
            "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64",
            "bool", "char", "str", "&str", "Box", "Rc", "Arc", "RefCell", "Mutex", "HashMap",
            "BTreeMap",
        ]);

        let mut literals = HashSet::new();
        literals.extend(["true", "false", "null"]);

        Self {
            keywords,
            types,
            literals,
        }
    }
}

impl RustSyntaxHighlighter {
    pub fn highlight_line(&self, line: &str) -> Line {
        let mut spans = Vec::new();
        let mut current_pos = 0;
        let chars: Vec<char> = line.chars().collect();

        while current_pos < chars.len() {
            let ch = chars[current_pos];

            match ch {
                // String literals
                '"' => {
                    let (span, new_pos) = self.parse_string_literal(&chars, current_pos);
                    spans.push(span);
                    current_pos = new_pos;
                }
                // Character literals
                '\'' => {
                    let (span, new_pos) = self.parse_char_literal(&chars, current_pos);
                    spans.push(span);
                    current_pos = new_pos;
                }
                // Comments
                '/' if current_pos + 1 < chars.len() && chars[current_pos + 1] == '/' => {
                    let comment_text: String = chars[current_pos..].iter().collect();
                    spans.push(Span::styled(
                        comment_text,
                        Style::default()
                            .fg(Color::Gray)
                            .add_modifier(Modifier::ITALIC),
                    ));
                    break; // Rest of line is comment
                }
                // Numbers
                c if c.is_ascii_digit() => {
                    let (span, new_pos) = self.parse_number(&chars, current_pos);
                    spans.push(span);
                    current_pos = new_pos;
                }
                // Identifiers and keywords
                c if c.is_alphabetic() || c == '_' => {
                    let (span, new_pos) = self.parse_identifier(&chars, current_pos);
                    spans.push(span);
                    current_pos = new_pos;
                }
                // Operators and punctuation
                c if "{}()[]<>,.;:!@#$%^&*+-=|\\?".contains(c) => {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default().fg(Color::Cyan),
                    ));
                    current_pos += 1;
                }
                // Whitespace
                _ => {
                    spans.push(Span::raw(ch.to_string()));
                    current_pos += 1;
                }
            }
        }

        Line::from(spans)
    }

    fn parse_string_literal(&self, chars: &[char], start: usize) -> (Span, usize) {
        let mut pos = start + 1; // Skip opening quote
        let mut content = String::from("\"");

        while pos < chars.len() {
            let ch = chars[pos];
            content.push(ch);

            if ch == '"' && (pos == start + 1 || chars[pos - 1] != '\\') {
                pos += 1;
                break;
            }
            pos += 1;
        }

        (
            Span::styled(content, Style::default().fg(Color::Green)),
            pos,
        )
    }

    fn parse_char_literal(&self, chars: &[char], start: usize) -> (Span, usize) {
        let mut pos = start + 1; // Skip opening quote
        let mut content = String::from("'");

        while pos < chars.len() {
            let ch = chars[pos];
            content.push(ch);

            if ch == '\'' && (pos == start + 1 || chars[pos - 1] != '\\') {
                pos += 1;
                break;
            }
            pos += 1;
        }

        (
            Span::styled(content, Style::default().fg(Color::Green)),
            pos,
        )
    }

    fn parse_number(&self, chars: &[char], start: usize) -> (Span, usize) {
        let mut pos = start;
        let mut content = String::new();

        while pos < chars.len() {
            let ch = chars[pos];
            if ch.is_ascii_digit() || ch == '.' || ch == '_' {
                content.push(ch);
                pos += 1;
            } else {
                break;
            }
        }

        (
            Span::styled(content, Style::default().fg(Color::Magenta)),
            pos,
        )
    }

    fn parse_identifier(&self, chars: &[char], start: usize) -> (Span, usize) {
        let mut pos = start;
        let mut content = String::new();

        while pos < chars.len() {
            let ch = chars[pos];
            if ch.is_alphanumeric() || ch == '_' {
                content.push(ch);
                pos += 1;
            } else {
                break;
            }
        }

        let style = if self.keywords.contains(content.as_str()) {
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        } else if self.types.contains(content.as_str()) {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if self.literals.contains(content.as_str()) {
            Style::default().fg(Color::Red)
        } else if content.chars().next().unwrap_or('a').is_uppercase() {
            // Likely a type or constant
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        (Span::styled(content, style), pos)
    }
}
