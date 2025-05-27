use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

/// EVA-style color palette inspired by Neon Genesis Evangelion
pub struct EvaColors;

impl EvaColors {
    // Primary EVA colors
    pub const ORANGE: Color = Color::Rgb(255, 102, 0); // EVA Unit-01 orange
    pub const RED: Color = Color::Rgb(220, 20, 20); // Alert red
    pub const AMBER: Color = Color::Rgb(255, 191, 0); // Warning amber
    pub const GREEN: Color = Color::Rgb(0, 255, 127); // Success/operational green

    // Background and UI colors
    pub const DARK_BG: Color = Color::Rgb(20, 20, 30); // Dark background
    pub const PANEL_BG: Color = Color::Rgb(40, 40, 50); // Panel background
    pub const BORDER: Color = Color::Rgb(255, 102, 0); // Default border orange

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(255, 255, 255); // White text
    pub const TEXT_SECONDARY: Color = Color::Rgb(200, 200, 200); // Gray text
    pub const TEXT_HIGHLIGHT: Color = Color::Rgb(255, 255, 0); // Yellow highlight

    // Status colors
    pub const STATUS_NORMAL: Color = Color::Rgb(0, 255, 127); // Green
    pub const STATUS_WARNING: Color = Color::Rgb(255, 191, 0); // Amber
    pub const STATUS_CRITICAL: Color = Color::Rgb(220, 20, 20); // Red
    pub const STATUS_UNKNOWN: Color = Color::Rgb(128, 128, 128); // Gray

    // Special effects
    pub const SYNC_RATE: Color = Color::Rgb(0, 255, 255); // Cyan for sync rates
    pub const AT_FIELD: Color = Color::Rgb(255, 0, 255); // Magenta for AT Field

    // Gradient background colors (dark to darker)
    pub const GRADIENT_TOP: Color = Color::Rgb(30, 30, 45); // Slightly lighter dark
    pub const GRADIENT_MID: Color = Color::Rgb(20, 20, 35); // Medium dark
    pub const GRADIENT_BOTTOM: Color = Color::Rgb(10, 10, 25); // Darkest
}

/// EVA-style border configurations
pub struct EvaBorders;

impl EvaBorders {
    /// Standard EVA panel border
    pub fn panel() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(EvaColors::BORDER))
    }

    /// Critical alert border with flashing effect
    pub fn critical() -> Block<'static> {
        let flash = Self::get_flash_state();
        let color = if flash {
            EvaColors::STATUS_CRITICAL
        } else {
            EvaColors::BORDER
        };

        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
    }

    /// Warning border
    pub fn warning() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(EvaColors::STATUS_WARNING))
    }

    /// Success/operational border
    pub fn operational() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvaColors::STATUS_NORMAL))
    }

    /// Header border with title styling
    pub fn header(title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(
                Style::default()
                    .fg(EvaColors::ORANGE)
                    .add_modifier(Modifier::BOLD),
            )
            .title(format!("[ {} ]", title))
            .title_style(
                Style::default()
                    .fg(EvaColors::TEXT_HIGHLIGHT)
                    .add_modifier(Modifier::BOLD),
            )
    }

    /// Get flashing state for critical alerts
    fn get_flash_state() -> bool {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        (time / 500) % 2 == 0
    }
}

/// EVA-style text styles
pub struct EvaStyles;

impl EvaStyles {
    /// Primary text style
    pub fn text_primary() -> Style {
        Style::default().fg(EvaColors::TEXT_PRIMARY)
    }

    /// Secondary text style
    pub fn text_secondary() -> Style {
        Style::default().fg(EvaColors::TEXT_SECONDARY)
    }

    /// Highlighted text
    pub fn text_highlight() -> Style {
        Style::default()
            .fg(EvaColors::TEXT_HIGHLIGHT)
            .add_modifier(Modifier::BOLD)
    }

    /// Critical alert text
    pub fn text_critical() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_CRITICAL)
            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
    }

    /// Warning text
    pub fn text_warning() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_WARNING)
            .add_modifier(Modifier::BOLD)
    }

    /// Success text
    pub fn text_success() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_NORMAL)
            .add_modifier(Modifier::BOLD)
    }

    /// Selected item style
    pub fn selected() -> Style {
        Style::default()
            .fg(EvaColors::TEXT_PRIMARY)
            .bg(EvaColors::ORANGE)
            .add_modifier(Modifier::BOLD)
    }

    /// Sync rate display style
    pub fn sync_rate() -> Style {
        Style::default()
            .fg(EvaColors::SYNC_RATE)
            .add_modifier(Modifier::BOLD)
    }

    /// AT Field style
    pub fn at_field() -> Style {
        Style::default()
            .fg(EvaColors::AT_FIELD)
            .add_modifier(Modifier::BOLD)
    }
}

/// EVA-style UI symbols and indicators
pub struct EvaSymbols;

impl EvaSymbols {
    // Status indicators
    pub const OPERATIONAL: &'static str = "◉";
    pub const WARNING: &'static str = "⚠";
    pub const CRITICAL: &'static str = "⚡";
    pub const OFFLINE: &'static str = "◯";

    // Progress indicators
    pub const SYNC_FULL: &'static str = "████";
    pub const SYNC_HIGH: &'static str = "███░";
    pub const SYNC_MED: &'static str = "██░░";
    pub const SYNC_LOW: &'static str = "█░░░";
    pub const SYNC_NONE: &'static str = "░░░░";

    // Directional indicators
    pub const ARROW_UP: &'static str = "▲";
    pub const ARROW_DOWN: &'static str = "▼";
    pub const ARROW_LEFT: &'static str = "◀";
    pub const ARROW_RIGHT: &'static str = "▶";

    // Special symbols
    pub const HEXAGON: &'static str = "⬢";
    pub const DIAMOND: &'static str = "◆";
    pub const TRIANGLE: &'static str = "▲";
    pub const SQUARE: &'static str = "■";

    // Animation frames for loading
    pub const LOADING_FRAMES: &'static [&'static str] = &["◐", "◓", "◑", "◒"];

    /// Get current loading animation frame
    pub fn loading_frame() -> &'static str {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let frame = (time / 200) % Self::LOADING_FRAMES.len() as u128;
        Self::LOADING_FRAMES[frame as usize]
    }

    /// Get sync rate symbol based on percentage
    pub fn sync_rate_symbol(rate: f64) -> &'static str {
        match rate as u32 {
            80..=100 => Self::SYNC_FULL,
            60..=79 => Self::SYNC_HIGH,
            40..=59 => Self::SYNC_MED,
            20..=39 => Self::SYNC_LOW,
            _ => Self::SYNC_NONE,
        }
    }

    /// Get status symbol based on condition
    pub fn status_symbol(is_ok: bool, is_warning: bool) -> &'static str {
        if is_ok {
            Self::OPERATIONAL
        } else if is_warning {
            Self::WARNING
        } else {
            Self::CRITICAL
        }
    }
}

/// EVA-style formatting utilities
pub struct EvaFormat;

impl EvaFormat {
    /// Format a title in EVA style
    pub fn title(text: &str) -> String {
        format!("[ {} ]", text.to_uppercase())
    }

    /// Format a status display
    pub fn status(label: &str, value: &str, is_ok: bool) -> String {
        let symbol = if is_ok {
            EvaSymbols::OPERATIONAL
        } else {
            EvaSymbols::CRITICAL
        };
        format!("{} {}: {}", symbol, label.to_uppercase(), value)
    }

    /// Format a sync rate display
    pub fn sync_rate(rate: f64) -> String {
        let symbol = EvaSymbols::sync_rate_symbol(rate);
        format!("{} SYNC RATE: {:.1}%", symbol, rate)
    }

    /// Format a technical readout
    pub fn readout(label: &str, value: &str, unit: &str) -> String {
        format!("{}: {} {}", label.to_uppercase(), value, unit)
    }

    /// Format a hexagonal display element
    pub fn hex_display(content: &str) -> String {
        format!(
            "{} {} {}",
            EvaSymbols::HEXAGON,
            content,
            EvaSymbols::HEXAGON
        )
    }

    /// Format timestamp in EVA style
    pub fn timestamp() -> String {
        let now = chrono::Utc::now();
        format!("[{}]", now.format("%H:%M:%S"))
    }

    /// Format a progress bar in EVA style
    pub fn progress_bar(progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        let empty = width - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    /// Get gradient background color based on vertical position
    pub fn gradient_bg(y_position: u16, total_height: u16) -> Color {
        let ratio = y_position as f64 / total_height as f64;

        if ratio < 0.33 {
            // Top third - lighter
            EvaColors::GRADIENT_TOP
        } else if ratio < 0.66 {
            // Middle third - medium
            EvaColors::GRADIENT_MID
        } else {
            // Bottom third - darkest
            EvaColors::GRADIENT_BOTTOM
        }
    }
}
