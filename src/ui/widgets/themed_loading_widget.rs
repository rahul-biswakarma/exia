use crate::ui::theme::Theme; // Adjusted import
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Paragraph}, // Added Block
    Frame,
}; // EvaFormat, EvaSymbols, etc. removed

pub struct ThemedLoadingWidget<'a> {
    pub message: String,
    pub is_loading: bool,
    pub show_progress: bool,
    pub progress: f64, // 0.0 to 1.0
    pub operation_type: LoadingOperationType,
    pub theme: &'a dyn Theme, // Theme is now mandatory
}

#[derive(Debug, Clone)]
pub enum LoadingOperationType {
    SystemBoot,
    AngelDetection,
    EvaActivation,
    SyncTest,
    DataAnalysis,
    MagiCalculation,
    AtFieldGeneration,
    // Added from original task description, ensure these are covered in ThemeSymbols::loading_messages
    DeviceDiscovery,
    DataBackup,
    SoftwareUpdate,
    SecurityScan,
    NetworkCheck,
    LogUpload,
    ConfigApply,
    Custom(String),
}

impl<'a> ThemedLoadingWidget<'a> {
    pub fn new(message: String, is_loading: bool, theme: &'a dyn Theme) -> Self {
        Self {
            message,
            is_loading,
            show_progress: false,
            progress: 0.0,
            operation_type: LoadingOperationType::SystemBoot, // Default operation type
            theme,
        }
    }

    pub fn with_progress(mut self, progress: f64) -> Self {
        self.show_progress = true;
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn with_operation_type(mut self, operation_type: LoadingOperationType) -> Self {
        self.operation_type = operation_type;
        self
    }

    fn get_operation_prefix(&self) -> String {
        let messages = self.theme.symbols().loading_messages();
        match self.operation_type {
            LoadingOperationType::SystemBoot => messages.system_boot.clone(),
            LoadingOperationType::AngelDetection => messages.angel_detection.clone(),
            LoadingOperationType::EvaActivation => messages.eva_activation.clone(),
            LoadingOperationType::SyncTest => messages.sync_test.clone(),
            LoadingOperationType::DataAnalysis => messages.data_analysis.clone(),
            LoadingOperationType::MagiCalculation => messages.magi_calculation.clone(),
            LoadingOperationType::AtFieldGeneration => messages.at_field_generation.clone(),
            LoadingOperationType::DeviceDiscovery => messages.device_discovery.clone(),
            LoadingOperationType::DataBackup => messages.data_backup.clone(),
            LoadingOperationType::SoftwareUpdate => messages.software_update.clone(),
            LoadingOperationType::SecurityScan => messages.security_scan.clone(),
            LoadingOperationType::NetworkCheck => messages.network_check.clone(),
            LoadingOperationType::LogUpload => messages.log_upload.clone(),
            LoadingOperationType::ConfigApply => messages.config_apply.clone(),
            LoadingOperationType::Custom(ref msg) => msg.clone(),
        }
    }

    fn get_loading_animation(&self) -> String {
        if !self.is_loading {
            let operational_symbol = self.theme.symbols().operational();
            return format!("{} {}", operational_symbol, "Complete");
        }

        let loading_frames = self.theme.symbols().loading_frames();
        let frame = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            / 500) // Keep the slower animation
            % loading_frames.len() as u128;

        let animation_char = loading_frames[frame as usize];
        format!("{} {}", animation_char, self.get_operation_prefix())
    }

    fn get_progress_display(&self) -> String {
        if !self.show_progress {
            return String::new();
        }
        // Uses self.create_themed_progress_bar which already uses the theme.
        let progress_bar = self.create_themed_progress_bar(self.progress, 20, self.theme);
        let percentage = self.progress * 100.0;

        format!("\n\nProgress: {:.1}%\n[{}]", percentage, progress_bar)
    }

    // This method already correctly uses the theme via its `theme` argument
    fn create_themed_progress_bar(&self, progress: f64, width: usize, theme: &dyn Theme) -> String {
        let progress_symbols = theme.symbols().progress_symbols();
        let filled = (progress * width as f64) as usize;
        let empty_symbol = progress_symbols.last().unwrap_or(&" "); // Default if empty
        let fill_symbol = progress_symbols.first().unwrap_or(&"█"); // Default if empty

        format!(
            "{}{}",
            fill_symbol.repeat(filled),
            empty_symbol.repeat(width - filled)
        )
    }

    fn get_status_readouts(&self) -> String {
        // Timestamp removed as per requirements
        let status = if self.is_loading {
            "In Progress"
        } else {
            "Complete"
        };
        format!("\n\nStatus: {}", status)
    }

    fn format_display(&self) -> String {
        let animation = self.get_loading_animation();
        let progress = self.get_progress_display();
        let status = self.get_status_readouts();

        format!("{}\n\n{}{}{}", animation, self.message, progress, status)
    }

    fn get_border_style(&self) -> Block<'static> { // Return type changed to Block
        if self.is_loading {
            self.theme.borders().warning_block()
        } else {
            self.theme.borders().success_block() // Assuming success_block for operational state
        }
    }

    fn get_text_style(&self) -> Style {
        if self.is_loading {
            self.theme.styles().text_warning()
        } else {
            self.theme.styles().text_success()
        }
    }
}

// Implementing crate::ui::widgets::Widget trait
impl<'a> crate::ui::widgets::Widget for ThemedLoadingWidget<'a> {
    fn render(&self, f: &mut Frame, area: Rect) {
        let content = self.format_display();
        let paragraph = Paragraph::new(content)
            .style(self.get_text_style())
            .block(self.get_border_style().title(self.title().unwrap_or("Operations"))); // Added title to block

        f.render_widget(paragraph, area);
    }

    fn title(&self) -> Option<&str> {
        // Title can be dynamic based on operation or state if needed
        Some("System Operations") 
    }

    fn border_style(&self) -> Style {
        if self.is_loading {
            Style::default().fg(self.theme.colors().warning())
        } else {
            Style::default().fg(self.theme.colors().success())
        }
    }
}
