use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

/// Theme trait for different UI themes
pub trait Theme {
    fn name(&self) -> &'static str;
    fn colors(&self) -> &dyn ThemeColors;
    fn borders(&self) -> &dyn ThemeBorders;
    fn styles(&self) -> &dyn ThemeStyles;
    fn symbols(&self) -> &dyn ThemeSymbols;
    fn formats(&self) -> &dyn ThemeFormats;
}

/// Color palette trait
pub trait ThemeColors {
    fn primary(&self) -> Color;
    fn secondary(&self) -> Color;
    fn accent(&self) -> Color;
    fn success(&self) -> Color;
    fn warning(&self) -> Color;
    fn error(&self) -> Color;
    fn background(&self) -> Color;
    fn surface(&self) -> Color;
    fn text_primary(&self) -> Color;
    fn text_secondary(&self) -> Color;
    fn text_highlight(&self) -> Color;
    fn border_default(&self) -> Color;
    fn border_focus(&self) -> Color;
}

/// Border configurations trait
pub trait ThemeBorders {
    fn default_block(&self) -> Block<'static>;
    fn focused_block(&self) -> Block<'static>;
    fn header_block(&self, title: &str) -> Block;
    fn warning_block(&self) -> Block<'static>;
    fn error_block(&self) -> Block<'static>;
    fn success_block(&self) -> Block<'static>;
    fn operational_block(&self) -> Block<'static>;
}

/// Text styles trait
pub trait ThemeStyles {
    fn text_primary(&self) -> Style;
    fn text_secondary(&self) -> Style;
    fn text_highlight(&self) -> Style;
    fn text_success(&self) -> Style;
    fn text_warning(&self) -> Style;
    fn text_error(&self) -> Style;
    fn selected(&self) -> Style;
}

/// Symbol sets trait
pub trait ThemeSymbols {
    fn operational(&self) -> &'static str;
    fn warning(&self) -> &'static str;
    fn error(&self) -> &'static str;
    fn offline(&self) -> &'static str;
    fn loading_frames(&self) -> &'static [&'static str];
    fn corner_decoration(&self) -> &'static str;
    fn geometric_shapes(&self) -> &'static [&'static str];
    fn progress_symbols(&self) -> &'static [&'static str];
    fn status_indicators(&self) -> StatusIndicators;
    fn loading_messages(&self) -> LoadingMessages;
}

/// Format utilities trait
pub trait ThemeFormats {
    fn title(&self, text: &str) -> String;
    fn status(&self, label: &str, value: &str, is_ok: bool) -> String;
    fn readout(&self, label: &str, value: &str, unit: &str) -> String;
    fn hex_display(&self, content: &str) -> String;
    fn progress_bar(&self, progress: f64, width: usize) -> String;
}

/// Status indicator symbols for different themes
#[derive(Debug, Clone)]
pub struct StatusIndicators {
    pub sync_high: String,
    pub sync_medium: String,
    pub sync_low: String,
    pub sync_critical: String,
    pub connection_active: String,
    pub connection_weak: String,
    pub connection_lost: String,
    pub operational: String,
    pub warning: String,
    pub error: String,
    pub info: String,
    pub neutral: String,
    pub degraded: String,
}

/// Loading messages for different themes
#[derive(Debug, Clone)]
pub struct LoadingMessages {
    pub system_boot: String,
    pub angel_detection: String,
    pub eva_activation: String,
    pub sync_test: String,
    pub data_analysis: String,
    pub magi_calculation: String,
    pub at_field_generation: String,
    pub device_discovery: String,
    pub data_backup: String,
    pub software_update: String,
    pub security_scan: String,
    pub network_check: String,
    pub log_upload: String,
    pub config_apply: String,
}

// ============================================================================
// EVANGELION THEME
// ============================================================================

pub struct EvangelionTheme;

impl Theme for EvangelionTheme {
    fn name(&self) -> &'static str {
        "Evangelion"
    }
    fn colors(&self) -> &dyn ThemeColors {
        &EvangelionColors
    }
    fn borders(&self) -> &dyn ThemeBorders {
        &EvangelionBorders
    }
    fn styles(&self) -> &dyn ThemeStyles {
        &EvangelionStyles
    }
    fn symbols(&self) -> &dyn ThemeSymbols {
        &EvangelionSymbols
    }
    fn formats(&self) -> &dyn ThemeFormats {
        &EvangelionFormats
    }
}

struct EvangelionColors;

impl ThemeColors for EvangelionColors {
    fn primary(&self) -> Color {
        Color::Rgb(255, 102, 0)
    } // EVA orange
    fn secondary(&self) -> Color {
        Color::Rgb(220, 20, 20)
    } // Alert red
    fn accent(&self) -> Color {
        Color::Rgb(255, 191, 0)
    } // Warning amber
    fn success(&self) -> Color {
        Color::Rgb(0, 255, 127)
    } // Operational green
    fn warning(&self) -> Color {
        Color::Rgb(255, 191, 0)
    } // Amber
    fn error(&self) -> Color {
        Color::Rgb(220, 20, 20)
    } // Red
    fn background(&self) -> Color {
        Color::Rgb(20, 20, 30)
    } // Dark background
    fn surface(&self) -> Color {
        Color::Rgb(40, 40, 50)
    } // Panel background
    fn text_primary(&self) -> Color {
        Color::Rgb(255, 255, 255)
    } // White
    fn text_secondary(&self) -> Color {
        Color::Rgb(200, 200, 200)
    } // Gray
    fn text_highlight(&self) -> Color {
        Color::Rgb(255, 255, 0)
    } // Yellow
    fn border_default(&self) -> Color {
        Color::Rgb(255, 102, 0)
    } // Orange
    fn border_focus(&self) -> Color {
        Color::Rgb(0, 255, 127)
    } // Green
}

struct EvangelionBorders;

impl ThemeBorders for EvangelionBorders {
    fn default_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvangelionColors.border_default()))
    }

    fn focused_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(
                Style::default()
                    .fg(EvangelionColors.border_focus())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn header_block(&self, title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(
                Style::default()
                    .fg(EvangelionColors.primary())
                    .add_modifier(Modifier::BOLD),
            )
            .title(format!("[ {} ]", title))
            .title_style(
                Style::default()
                    .fg(EvangelionColors.text_highlight())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn warning_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(EvangelionColors.warning()))
    }

    fn error_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(
                Style::default()
                    .fg(EvangelionColors.error())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn success_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvangelionColors.success()))
    }

    fn operational_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvangelionColors.primary()))
    }
}

struct EvangelionStyles;

impl ThemeStyles for EvangelionStyles {
    fn text_primary(&self) -> Style {
        Style::default().fg(EvangelionColors.text_primary())
    }
    fn text_secondary(&self) -> Style {
        Style::default().fg(EvangelionColors.text_secondary())
    }
    fn text_highlight(&self) -> Style {
        Style::default()
            .fg(EvangelionColors.text_highlight())
            .add_modifier(Modifier::BOLD)
    }
    fn text_success(&self) -> Style {
        Style::default()
            .fg(EvangelionColors.success())
            .add_modifier(Modifier::BOLD)
    }
    fn text_warning(&self) -> Style {
        Style::default()
            .fg(EvangelionColors.warning())
            .add_modifier(Modifier::BOLD)
    }
    fn text_error(&self) -> Style {
        Style::default()
            .fg(EvangelionColors.error())
            .add_modifier(Modifier::BOLD)
    }
    fn selected(&self) -> Style {
        Style::default()
            .fg(EvangelionColors.text_primary())
            .bg(EvangelionColors.primary())
            .add_modifier(Modifier::BOLD)
    }
}

struct EvangelionSymbols;

impl ThemeSymbols for EvangelionSymbols {
    fn operational(&self) -> &'static str {
        "◉"
    }
    fn warning(&self) -> &'static str {
        "⚠"
    }
    fn error(&self) -> &'static str {
        "⚡"
    }
    fn offline(&self) -> &'static str {
        "◯"
    }
    fn loading_frames(&self) -> &'static [&'static str] {
        &["◐", "◓", "◑", "◒"]
    }
    fn corner_decoration(&self) -> &'static str {
        "■"
    }
    fn geometric_shapes(&self) -> &'static [&'static str] {
        &["◆", "▲", "●", "■", "⬢"]
    }
    fn progress_symbols(&self) -> &'static [&'static str] {
        &["█", "▉", "▊", "▋", "▌", "▍", "▎", "▏", "░"]
    }
    fn status_indicators(&self) -> StatusIndicators {
        StatusIndicators {
            sync_high: "████".to_string(),
            sync_medium: "███░".to_string(),
            sync_low: "██░░".to_string(),
            sync_critical: "█░░░".to_string(),
            connection_active: "◉◉◉".to_string(),
            connection_weak: "◉◉◯".to_string(),
            connection_lost: "◯◯◯".to_string(),
            operational: "◉".to_string(),
            warning: "⚠".to_string(),
            error: "⚡".to_string(),
            info: "ℹ".to_string(),
            neutral: "●".to_string(),
            degraded: "◯".to_string(),
        }
    }
    fn loading_messages(&self) -> LoadingMessages {
        LoadingMessages {
            system_boot: "NERV SYSTEM INITIALIZATION".to_string(),
            angel_detection: "PATTERN BLUE DETECTED".to_string(),
            eva_activation: "EVA UNIT ACTIVATION SEQUENCE".to_string(),
            sync_test: "PILOT SYNCHRONIZATION TEST".to_string(),
            data_analysis: "MAGI SYSTEM ANALYSIS".to_string(),
            magi_calculation: "MAGI SUPERCOMPUTER CALCULATION".to_string(),
            at_field_generation: "AT FIELD GENERATION".to_string(),
            device_discovery: "DEVICE DISCOVERY PROTOCOL".to_string(),
            data_backup: "DATA BACKUP SEQUENCE".to_string(),
            software_update: "SOFTWARE UPDATE PROTOCOL".to_string(),
            security_scan: "SECURITY SCAN INITIATED".to_string(),
            network_check: "NETWORK DIAGNOSTICS".to_string(),
            log_upload: "LOG UPLOAD SEQUENCE".to_string(),
            config_apply: "CONFIGURATION APPLY".to_string(),
        }
    }
}

struct EvangelionFormats;

impl ThemeFormats for EvangelionFormats {
    fn title(&self, text: &str) -> String {
        format!("[ {} ]", text.to_uppercase())
    }

    fn status(&self, label: &str, value: &str, is_ok: bool) -> String {
        let symbol = if is_ok { "◉" } else { "⚠" };
        format!("{} {}: {}", symbol, label, value)
    }

    fn readout(&self, label: &str, value: &str, unit: &str) -> String {
        format!("{}: {} {}", label.to_uppercase(), value, unit)
    }

    fn hex_display(&self, content: &str) -> String {
        content
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn progress_bar(&self, progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        "█".repeat(filled) + &"░".repeat(width - filled)
    }
}

// ============================================================================
// GUNDAM 00 THEME
// ============================================================================

pub struct Gundam00Theme;

impl Theme for Gundam00Theme {
    fn name(&self) -> &'static str {
        "Gundam 00"
    }
    fn colors(&self) -> &dyn ThemeColors {
        &Gundam00Colors
    }
    fn borders(&self) -> &dyn ThemeBorders {
        &Gundam00Borders
    }
    fn styles(&self) -> &dyn ThemeStyles {
        &Gundam00Styles
    }
    fn symbols(&self) -> &dyn ThemeSymbols {
        &Gundam00Symbols
    }
    fn formats(&self) -> &dyn ThemeFormats {
        &Gundam00Formats
    }
}

struct Gundam00Colors;

impl ThemeColors for Gundam00Colors {
    fn primary(&self) -> Color {
        Color::Rgb(0, 150, 255)
    } // Celestial Being blue
    fn secondary(&self) -> Color {
        Color::Rgb(255, 255, 255)
    } // White
    fn accent(&self) -> Color {
        Color::Rgb(0, 255, 200)
    } // Cyan accent
    fn success(&self) -> Color {
        Color::Rgb(0, 255, 100)
    } // Green
    fn warning(&self) -> Color {
        Color::Rgb(255, 200, 0)
    } // Gold
    fn error(&self) -> Color {
        Color::Rgb(255, 50, 50)
    } // Red
    fn background(&self) -> Color {
        Color::Rgb(10, 15, 25)
    } // Deep space blue
    fn surface(&self) -> Color {
        Color::Rgb(25, 35, 50)
    } // Panel blue
    fn text_primary(&self) -> Color {
        Color::Rgb(255, 255, 255)
    } // White
    fn text_secondary(&self) -> Color {
        Color::Rgb(180, 200, 220)
    } // Light blue
    fn text_highlight(&self) -> Color {
        Color::Rgb(0, 255, 200)
    } // Cyan
    fn border_default(&self) -> Color {
        Color::Rgb(0, 150, 255)
    } // Blue
    fn border_focus(&self) -> Color {
        Color::Rgb(0, 255, 200)
    } // Cyan
}

struct Gundam00Borders;

impl ThemeBorders for Gundam00Borders {
    fn default_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Gundam00Colors.border_default()))
    }

    fn focused_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(
                Style::default()
                    .fg(Gundam00Colors.border_focus())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn header_block(&self, title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(
                Style::default()
                    .fg(Gundam00Colors.primary())
                    .add_modifier(Modifier::BOLD),
            )
            .title(format!("◤ {} ◥", title))
            .title_style(
                Style::default()
                    .fg(Gundam00Colors.text_highlight())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn warning_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Gundam00Colors.warning()))
    }

    fn error_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(
                Style::default()
                    .fg(Gundam00Colors.error())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn success_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Gundam00Colors.success()))
    }

    fn operational_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(Gundam00Colors.primary()))
    }
}

struct Gundam00Styles;

impl ThemeStyles for Gundam00Styles {
    fn text_primary(&self) -> Style {
        Style::default().fg(Gundam00Colors.text_primary())
    }
    fn text_secondary(&self) -> Style {
        Style::default().fg(Gundam00Colors.text_secondary())
    }
    fn text_highlight(&self) -> Style {
        Style::default()
            .fg(Gundam00Colors.text_highlight())
            .add_modifier(Modifier::BOLD)
    }
    fn text_success(&self) -> Style {
        Style::default()
            .fg(Gundam00Colors.success())
            .add_modifier(Modifier::BOLD)
    }
    fn text_warning(&self) -> Style {
        Style::default()
            .fg(Gundam00Colors.warning())
            .add_modifier(Modifier::BOLD)
    }
    fn text_error(&self) -> Style {
        Style::default()
            .fg(Gundam00Colors.error())
            .add_modifier(Modifier::BOLD)
    }
    fn selected(&self) -> Style {
        Style::default()
            .fg(Gundam00Colors.background())
            .bg(Gundam00Colors.primary())
            .add_modifier(Modifier::BOLD)
    }
}

struct Gundam00Symbols;

impl ThemeSymbols for Gundam00Symbols {
    fn operational(&self) -> &'static str {
        "●"
    }
    fn warning(&self) -> &'static str {
        "▲"
    }
    fn error(&self) -> &'static str {
        "✕"
    }
    fn offline(&self) -> &'static str {
        "○"
    }
    fn loading_frames(&self) -> &'static [&'static str] {
        &["◢", "◣", "◤", "◥"]
    }
    fn corner_decoration(&self) -> &'static str {
        "◢"
    }
    fn geometric_shapes(&self) -> &'static [&'static str] {
        &["◢", "◣", "◤", "◥", "◆", "▼"]
    }
    fn progress_symbols(&self) -> &'static [&'static str] {
        &["▰", "▱", "▰", "▱", "▰", "▱", "▰", "▱", "▱"]
    }
    fn status_indicators(&self) -> StatusIndicators {
        StatusIndicators {
            sync_high: "▰▰▰▰".to_string(),
            sync_medium: "▰▰▰▱".to_string(),
            sync_low: "▰▰▱▱".to_string(),
            sync_critical: "▰▱▱▱".to_string(),
            connection_active: "●●●".to_string(),
            connection_weak: "●●○".to_string(),
            connection_lost: "○○○".to_string(),
            operational: "●".to_string(),
            warning: "▲".to_string(),
            error: "✕".to_string(),
            info: "ℹ".to_string(),
            neutral: "○".to_string(),
            degraded: "◯".to_string(),
        }
    }
    fn loading_messages(&self) -> LoadingMessages {
        LoadingMessages {
            system_boot: "CELESTIAL BEING SYSTEM ONLINE".to_string(),
            angel_detection: "TARGET ACQUISITION IN PROGRESS".to_string(),
            eva_activation: "GUNDAM ACTIVATION SEQUENCE".to_string(),
            sync_test: "QUANTUM BRAINWAVE SYNC".to_string(),
            data_analysis: "VEDA SYSTEM ANALYSIS".to_string(),
            magi_calculation: "QUANTUM COMPUTATION".to_string(),
            at_field_generation: "GN FIELD DEPLOYMENT".to_string(),
            device_discovery: "DEVICE SCAN PROTOCOL".to_string(),
            data_backup: "DATA ARCHIVE SEQUENCE".to_string(),
            software_update: "SYSTEM UPDATE PROTOCOL".to_string(),
            security_scan: "SECURITY ANALYSIS".to_string(),
            network_check: "NETWORK DIAGNOSTICS".to_string(),
            log_upload: "LOG TRANSMISSION".to_string(),
            config_apply: "CONFIGURATION UPDATE".to_string(),
        }
    }
}

struct Gundam00Formats;

impl ThemeFormats for Gundam00Formats {
    fn title(&self, text: &str) -> String {
        format!("◤ {} ◥", text.to_uppercase())
    }

    fn status(&self, label: &str, value: &str, is_ok: bool) -> String {
        let symbol = if is_ok { "●" } else { "▲" };
        format!("{} {}: {}", symbol, label, value)
    }

    fn readout(&self, label: &str, value: &str, unit: &str) -> String {
        format!("{}: {} {}", label.to_uppercase(), value, unit)
    }

    fn hex_display(&self, content: &str) -> String {
        content
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn progress_bar(&self, progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        "▰".repeat(filled) + &"▱".repeat(width - filled)
    }
}

// ============================================================================
// CLEAN TERMINAL THEME
// ============================================================================

pub struct CleanTerminalTheme;

impl Theme for CleanTerminalTheme {
    fn name(&self) -> &'static str {
        "Clean Terminal"
    }
    fn colors(&self) -> &dyn ThemeColors {
        &CleanTerminalColors
    }
    fn borders(&self) -> &dyn ThemeBorders {
        &CleanTerminalBorders
    }
    fn styles(&self) -> &dyn ThemeStyles {
        &CleanTerminalStyles
    }
    fn symbols(&self) -> &dyn ThemeSymbols {
        &CleanTerminalSymbols
    }
    fn formats(&self) -> &dyn ThemeFormats {
        &CleanTerminalFormats
    }
}

struct CleanTerminalColors;

impl ThemeColors for CleanTerminalColors {
    fn primary(&self) -> Color {
        Color::Rgb(100, 150, 255)
    } // Soft blue
    fn secondary(&self) -> Color {
        Color::Rgb(120, 120, 120)
    } // Gray
    fn accent(&self) -> Color {
        Color::Rgb(150, 100, 255)
    } // Purple
    fn success(&self) -> Color {
        Color::Rgb(100, 200, 100)
    } // Green
    fn warning(&self) -> Color {
        Color::Rgb(255, 180, 100)
    } // Orange
    fn error(&self) -> Color {
        Color::Rgb(255, 100, 100)
    } // Red
    fn background(&self) -> Color {
        Color::Rgb(25, 25, 25)
    } // Dark gray
    fn surface(&self) -> Color {
        Color::Rgb(35, 35, 35)
    } // Lighter gray
    fn text_primary(&self) -> Color {
        Color::Rgb(240, 240, 240)
    } // Light gray
    fn text_secondary(&self) -> Color {
        Color::Rgb(180, 180, 180)
    } // Medium gray
    fn text_highlight(&self) -> Color {
        Color::Rgb(255, 255, 255)
    } // White
    fn border_default(&self) -> Color {
        Color::Rgb(100, 100, 100)
    } // Gray
    fn border_focus(&self) -> Color {
        Color::Rgb(100, 150, 255)
    } // Blue
}

struct CleanTerminalBorders;

impl ThemeBorders for CleanTerminalBorders {
    fn default_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.border_default()))
    }

    fn focused_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(
                Style::default()
                    .fg(CleanTerminalColors.border_focus())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn header_block(&self, title: &str) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.primary()))
            .title(format!(" {} ", title))
            .title_style(
                Style::default()
                    .fg(CleanTerminalColors.text_highlight())
                    .add_modifier(Modifier::BOLD),
            )
    }

    fn warning_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.warning()))
    }

    fn error_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.error()))
    }

    fn success_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.success()))
    }

    fn operational_block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .border_style(Style::default().fg(CleanTerminalColors.primary()))
    }
}

struct CleanTerminalStyles;

impl ThemeStyles for CleanTerminalStyles {
    fn text_primary(&self) -> Style {
        Style::default().fg(CleanTerminalColors.text_primary())
    }
    fn text_secondary(&self) -> Style {
        Style::default().fg(CleanTerminalColors.text_secondary())
    }
    fn text_highlight(&self) -> Style {
        Style::default()
            .fg(CleanTerminalColors.text_highlight())
            .add_modifier(Modifier::BOLD)
    }
    fn text_success(&self) -> Style {
        Style::default().fg(CleanTerminalColors.success())
    }
    fn text_warning(&self) -> Style {
        Style::default().fg(CleanTerminalColors.warning())
    }
    fn text_error(&self) -> Style {
        Style::default().fg(CleanTerminalColors.error())
    }
    fn selected(&self) -> Style {
        Style::default()
            .fg(CleanTerminalColors.text_primary())
            .bg(CleanTerminalColors.primary())
    }
}

struct CleanTerminalSymbols;

impl ThemeSymbols for CleanTerminalSymbols {
    fn operational(&self) -> &'static str {
        "●"
    }
    fn warning(&self) -> &'static str {
        "!"
    }
    fn error(&self) -> &'static str {
        "✗"
    }
    fn offline(&self) -> &'static str {
        "○"
    }
    fn loading_frames(&self) -> &'static [&'static str] {
        &["|", "/", "-", "\\"]
    }
    fn corner_decoration(&self) -> &'static str {
        "+"
    }
    fn geometric_shapes(&self) -> &'static [&'static str] {
        &["+", "-", "|", "·", "•"]
    }
    fn progress_symbols(&self) -> &'static [&'static str] {
        &["=", "-", "=", "-", "=", "-", "=", "-", " "]
    }
    fn status_indicators(&self) -> StatusIndicators {
        StatusIndicators {
            sync_high: "====".to_string(),
            sync_medium: "===-".to_string(),
            sync_low: "==--".to_string(),
            sync_critical: "=---".to_string(),
            connection_active: "●●●".to_string(),
            connection_weak: "●●○".to_string(),
            connection_lost: "○○○".to_string(),
            operational: "●".to_string(),
            warning: "!".to_string(),
            error: "✗".to_string(),
            info: "i".to_string(),
            neutral: "○".to_string(),
            degraded: "·".to_string(),
        }
    }
    fn loading_messages(&self) -> LoadingMessages {
        LoadingMessages {
            system_boot: "System Initialization".to_string(),
            angel_detection: "Pattern Recognition".to_string(),
            eva_activation: "Application Startup".to_string(),
            sync_test: "Connection Test".to_string(),
            data_analysis: "Data Processing".to_string(),
            magi_calculation: "Computing".to_string(),
            at_field_generation: "Loading Resources".to_string(),
            device_discovery: "Device Discovery".to_string(),
            data_backup: "Data Backup".to_string(),
            software_update: "Software Update".to_string(),
            security_scan: "Security Scan".to_string(),
            network_check: "Network Check".to_string(),
            log_upload: "Log Upload".to_string(),
            config_apply: "Config Apply".to_string(),
        }
    }
}

struct CleanTerminalFormats;

impl ThemeFormats for CleanTerminalFormats {
    fn title(&self, text: &str) -> String {
        format!(" {} ", text)
    }

    fn status(&self, label: &str, value: &str, is_ok: bool) -> String {
        let symbol = if is_ok { "●" } else { "!" };
        format!("{} {}: {}", symbol, label, value)
    }

    fn readout(&self, label: &str, value: &str, unit: &str) -> String {
        format!("{}: {} {}", label, value, unit)
    }

    fn hex_display(&self, content: &str) -> String {
        content
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn progress_bar(&self, progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        "=".repeat(filled) + &"-".repeat(width - filled)
    }
}

// ============================================================================
// THEME MANAGER
// ============================================================================

pub struct ThemeManager {
    current_theme: Box<dyn Theme>,
    available_themes: Vec<Box<dyn Theme>>,
}

impl std::fmt::Debug for ThemeManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeManager")
            .field("current_theme", &self.current_theme.name())
            .field("available_themes_count", &self.available_themes.len())
            .finish()
    }
}

impl Clone for ThemeManager {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes: Vec<Box<dyn Theme>> = vec![
            Box::new(EvangelionTheme),
            Box::new(Gundam00Theme),
            Box::new(CleanTerminalTheme),
        ];

        Self {
            current_theme: Box::new(EvangelionTheme), // Default theme
            available_themes: themes,
        }
    }

    pub fn current_theme(&self) -> &dyn Theme {
        self.current_theme.as_ref()
    }

    pub fn available_themes(&self) -> Vec<&str> {
        self.available_themes.iter().map(|t| t.name()).collect()
    }

    pub fn switch_theme(&mut self, theme_name: &str) -> bool {
        for theme in &self.available_themes {
            if theme.name() == theme_name {
                self.current_theme = match theme_name {
                    "Evangelion" => Box::new(EvangelionTheme),
                    "Gundam 00" => Box::new(Gundam00Theme),
                    "Clean Terminal" => Box::new(CleanTerminalTheme),
                    _ => continue,
                };
                return true;
            }
        }
        false
    }
}

// ============================================================================
// LEGACY COMPATIBILITY (for existing code)
// ============================================================================

/// Legacy EVA colors for backward compatibility
pub struct EvaColors;

impl EvaColors {
    pub const ORANGE: Color = Color::Rgb(255, 102, 0);
    pub const RED: Color = Color::Rgb(220, 20, 20);
    pub const AMBER: Color = Color::Rgb(255, 191, 0);
    pub const GREEN: Color = Color::Rgb(0, 255, 127);
    pub const DARK_BG: Color = Color::Rgb(20, 20, 30);
    pub const PANEL_BG: Color = Color::Rgb(40, 40, 50);
    pub const BORDER: Color = Color::Rgb(255, 102, 0);
    pub const TEXT_PRIMARY: Color = Color::Rgb(255, 255, 255);
    pub const TEXT_SECONDARY: Color = Color::Rgb(200, 200, 200);
    pub const TEXT_HIGHLIGHT: Color = Color::Rgb(255, 255, 0);
    pub const STATUS_NORMAL: Color = Color::Rgb(0, 255, 127);
    pub const STATUS_WARNING: Color = Color::Rgb(255, 191, 0);
    pub const STATUS_CRITICAL: Color = Color::Rgb(220, 20, 20);
    pub const STATUS_UNKNOWN: Color = Color::Rgb(128, 128, 128);
    pub const SYNC_RATE: Color = Color::Rgb(0, 255, 255);
    pub const AT_FIELD: Color = Color::Rgb(255, 0, 255);
}

/// Legacy border configurations
pub struct EvaBorders;

impl EvaBorders {
    pub fn panel() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvaColors::BORDER))
    }

    pub fn warning() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvaColors::STATUS_WARNING))
    }

    pub fn operational() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(EvaColors::STATUS_NORMAL))
    }

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

    pub fn critical() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(
                Style::default()
                    .fg(EvaColors::STATUS_CRITICAL)
                    .add_modifier(Modifier::BOLD),
            )
    }

    pub fn error() -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(
                Style::default()
                    .fg(EvaColors::STATUS_CRITICAL)
                    .add_modifier(Modifier::BOLD),
            )
    }
}

/// Legacy styles
pub struct EvaStyles;

impl EvaStyles {
    pub fn text_primary() -> Style {
        Style::default().fg(EvaColors::TEXT_PRIMARY)
    }
    pub fn text_secondary() -> Style {
        Style::default().fg(EvaColors::TEXT_SECONDARY)
    }
    pub fn text_highlight() -> Style {
        Style::default()
            .fg(EvaColors::TEXT_HIGHLIGHT)
            .add_modifier(Modifier::BOLD)
    }
    pub fn text_warning() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_WARNING)
            .add_modifier(Modifier::BOLD)
    }
    pub fn text_success() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_NORMAL)
            .add_modifier(Modifier::BOLD)
    }
    pub fn selected() -> Style {
        Style::default()
            .fg(EvaColors::TEXT_PRIMARY)
            .bg(EvaColors::ORANGE)
            .add_modifier(Modifier::BOLD)
    }
    pub fn text_critical() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_CRITICAL)
            .add_modifier(Modifier::BOLD)
    }
    pub fn text_error() -> Style {
        Style::default()
            .fg(EvaColors::STATUS_CRITICAL)
            .add_modifier(Modifier::BOLD)
    }
    pub fn sync_rate() -> Style {
        Style::default()
            .fg(EvaColors::SYNC_RATE)
            .add_modifier(Modifier::BOLD)
    }
    pub fn at_field() -> Style {
        Style::default()
            .fg(EvaColors::AT_FIELD)
            .add_modifier(Modifier::BOLD)
    }
}

/// Legacy symbols
pub struct EvaSymbols;

impl EvaSymbols {
    pub const OPERATIONAL: &'static str = "◉";
    pub const WARNING: &'static str = "⚠";
    pub const CRITICAL: &'static str = "⚡";
    pub const OFFLINE: &'static str = "◯";
    pub const LOADING_FRAMES: &'static [&'static str] = &["◐", "◓", "◑", "◒"];
    pub const HEXAGON: &'static str = "⬢";
    pub const DIAMOND: &'static str = "◆";
    pub const TRIANGLE: &'static str = "▲";
    pub const SQUARE: &'static str = "■";
    pub const ARROW_RIGHT: &'static str = "→";
    pub const CLOCK: &'static str = "⏱";
    pub const SYNC: &'static str = "⟲";

    pub fn loading_frame() -> &'static str {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self::LOADING_FRAMES[(time / 250) as usize % Self::LOADING_FRAMES.len()]
    }

    pub fn sync_rate_symbol(rate: f64) -> &'static str {
        match rate {
            r if r >= 0.8 => "████",
            r if r >= 0.6 => "███░",
            r if r >= 0.4 => "██░░",
            r if r >= 0.2 => "█░░░",
            _ => "░░░░",
        }
    }

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

/// Legacy formatting utilities
pub struct EvaFormat;

impl EvaFormat {
    pub fn title(text: &str) -> String {
        format!("[ {} ]", text.to_uppercase())
    }

    pub fn status(label: &str, value: &str, is_ok: bool) -> String {
        let symbol = if is_ok {
            EvaSymbols::OPERATIONAL
        } else {
            EvaSymbols::WARNING
        };
        format!("{} {}: {}", symbol, label, value)
    }

    pub fn sync_rate(rate: f64) -> String {
        format!(
            "SYNC: {:.1}% {}",
            rate * 100.0,
            EvaSymbols::sync_rate_symbol(rate)
        )
    }

    pub fn readout(label: &str, value: &str, unit: &str) -> String {
        format!("{}: {} {}", label.to_uppercase(), value, unit)
    }

    pub fn hex_display(content: &str) -> String {
        content
            .chars()
            .map(|c| format!("{:02X}", c as u8))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn timestamp() -> String {
        chrono::Utc::now().format("%H:%M:%S").to_string()
    }

    pub fn progress_bar(progress: f64, width: usize) -> String {
        let filled = (progress * width as f64) as usize;
        "█".repeat(filled) + &"░".repeat(width - filled)
    }
}
