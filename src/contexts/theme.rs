use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeVariant {
    NeonEvangelion,
    Gundam,
    Terminal,
    ModernUI,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub text_secondary: String,
    pub border: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeSpacing {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeTypography {
    pub font_family: String,
    pub font_family_mono: String,
    pub font_size_xs: String,
    pub font_size_sm: String,
    pub font_size_md: String,
    pub font_size_lg: String,
    pub font_size_xl: String,
    pub font_weight_normal: String,
    pub font_weight_medium: String,
    pub font_weight_bold: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeBorders {
    pub radius_none: String,
    pub radius_sm: String,
    pub radius_md: String,
    pub radius_lg: String,
    pub radius_full: String,
    pub width_thin: String,
    pub width_medium: String,
    pub width_thick: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeEffects {
    pub shadow_sm: String,
    pub shadow_md: String,
    pub shadow_lg: String,
    pub glow_primary: String,
    pub glow_accent: String,
    pub blur_sm: String,
    pub blur_md: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeAnimations {
    pub duration_fast: String,
    pub duration_medium: String,
    pub duration_slow: String,
    pub easing_default: String,
    pub easing_bounce: String,
    pub easing_smooth: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeDecorative {
    pub corner_decorations: bool,
    pub glow_effects: bool,
    pub scan_lines: bool,
    pub noise_overlay: bool,
    pub hexagonal_elements: bool,
    pub terminal_cursor: bool,
    pub floating_particles: bool,
    pub angular_borders: bool,
    pub matrix_rain: bool,
    pub holographic_effects: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoaderStyles {
    pub primary_type: String, // "spinner", "dots", "bars", "hexagon", "matrix", "minimal"
    pub button_loader: String, // "inline-spinner", "dots", "pulse", "slide"
    pub page_loader: String,  // "full-screen", "overlay", "minimal"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub variant: ThemeVariant,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
    pub borders: ThemeBorders,
    pub effects: ThemeEffects,
    pub animations: ThemeAnimations,
    pub decorative: ThemeDecorative,
    pub loaders: LoaderStyles,
}

impl Theme {
    pub fn neon_evangelion() -> Self {
        Self {
            name: "Neon Evangelion".to_string(),
            variant: ThemeVariant::NeonEvangelion,
            colors: ThemeColors {
                primary: "#ff3366".to_string(),        // Electric red
                secondary: "#00ffcc".to_string(),      // Neon cyan
                accent: "#ffcc00".to_string(),         // Electric yellow
                background: "#0a0a0f".to_string(),     // Deep void
                surface: "#1a1a2e".to_string(),        // Dark purple
                text: "#ffffff".to_string(),           // Pure white
                text_secondary: "#cccccc".to_string(), // Light gray
                border: "#ff3366".to_string(),         // Electric red border
                success: "#00ff66".to_string(),        // Neon green
                warning: "#ffcc00".to_string(),        // Electric yellow
                error: "#ff3366".to_string(),          // Electric red
                info: "#00ffcc".to_string(),           // Neon cyan
            },
            spacing: Self::default_spacing(),
            typography: ThemeTypography {
                font_family: "'JetBrains Mono', 'Fira Code', 'Courier New', monospace".to_string(),
                font_family_mono: "'JetBrains Mono', 'Fira Code', monospace".to_string(),
                font_size_xs: "0.75rem".to_string(),
                font_size_sm: "0.875rem".to_string(),
                font_size_md: "1rem".to_string(),
                font_size_lg: "1.125rem".to_string(),
                font_size_xl: "1.25rem".to_string(),
                font_weight_normal: "400".to_string(),
                font_weight_medium: "500".to_string(),
                font_weight_bold: "700".to_string(),
            },
            borders: Self::default_borders(),
            effects: ThemeEffects {
                shadow_sm: "0 1px 2px rgba(255, 51, 102, 0.3)".to_string(),
                shadow_md: "0 4px 8px rgba(255, 51, 102, 0.4)".to_string(),
                shadow_lg: "0 8px 16px rgba(255, 51, 102, 0.5)".to_string(),
                glow_primary: "0 0 20px rgba(255, 51, 102, 0.8), 0 0 40px rgba(255, 51, 102, 0.4)"
                    .to_string(),
                glow_accent: "0 0 20px rgba(255, 204, 0, 0.8), 0 0 40px rgba(255, 204, 0, 0.4)"
                    .to_string(),
                blur_sm: "blur(4px)".to_string(),
                blur_md: "blur(8px)".to_string(),
            },
            animations: Self::default_animations(),
            decorative: ThemeDecorative {
                corner_decorations: true,
                glow_effects: true,
                scan_lines: true,
                noise_overlay: true,
                hexagonal_elements: false,
                terminal_cursor: true,
                floating_particles: true,
                angular_borders: false,
                matrix_rain: false,
                holographic_effects: true,
            },
            loaders: LoaderStyles {
                primary_type: "hexagon".to_string(),
                button_loader: "pulse".to_string(),
                page_loader: "full-screen".to_string(),
            },
        }
    }

    pub fn gundam() -> Self {
        Self {
            name: "Gundam Mecha".to_string(),
            variant: ThemeVariant::Gundam,
            colors: ThemeColors {
                primary: "#ff6b35".to_string(),        // Mecha orange
                secondary: "#004e89".to_string(),      // Deep blue
                accent: "#ffd23f".to_string(),         // Bright yellow
                background: "#0d1117".to_string(),     // Dark steel
                surface: "#21262d".to_string(),        // Steel gray
                text: "#f0f6fc".to_string(),           // Light steel
                text_secondary: "#8b949e".to_string(), // Medium gray
                border: "#ff6b35".to_string(),         // Mecha orange
                success: "#26d0ce".to_string(),        // Cyan success
                warning: "#ffd23f".to_string(),        // Bright yellow
                error: "#ff4757".to_string(),          // Red alert
                info: "#5352ed".to_string(),           // Blue info
            },
            spacing: Self::default_spacing(),
            typography: ThemeTypography {
                font_family: "'Orbitron', 'Exo 2', 'Roboto', sans-serif".to_string(),
                font_family_mono: "'JetBrains Mono', 'Fira Code', monospace".to_string(),
                font_size_xs: "0.75rem".to_string(),
                font_size_sm: "0.875rem".to_string(),
                font_size_md: "1rem".to_string(),
                font_size_lg: "1.125rem".to_string(),
                font_size_xl: "1.25rem".to_string(),
                font_weight_normal: "400".to_string(),
                font_weight_medium: "600".to_string(),
                font_weight_bold: "700".to_string(),
            },
            borders: ThemeBorders {
                radius_none: "0".to_string(),
                radius_sm: "0".to_string(), // Angular design
                radius_md: "0".to_string(),
                radius_lg: "0".to_string(),
                radius_full: "0".to_string(),
                width_thin: "1px".to_string(),
                width_medium: "2px".to_string(),
                width_thick: "4px".to_string(), // Thick mecha borders
            },
            effects: ThemeEffects {
                shadow_sm: "0 2px 4px rgba(255, 107, 53, 0.2)".to_string(),
                shadow_md: "0 4px 8px rgba(255, 107, 53, 0.3)".to_string(),
                shadow_lg: "0 8px 16px rgba(255, 107, 53, 0.4)".to_string(),
                glow_primary: "0 0 15px rgba(255, 107, 53, 0.6), 0 0 30px rgba(255, 107, 53, 0.3)"
                    .to_string(),
                glow_accent: "0 0 15px rgba(255, 210, 63, 0.6), 0 0 30px rgba(255, 210, 63, 0.3)"
                    .to_string(),
                blur_sm: "blur(2px)".to_string(),
                blur_md: "blur(4px)".to_string(),
            },
            animations: Self::default_animations(),
            decorative: ThemeDecorative {
                corner_decorations: true,
                glow_effects: true,
                scan_lines: false,
                noise_overlay: false,
                hexagonal_elements: true,
                terminal_cursor: false,
                floating_particles: false,
                angular_borders: true,
                matrix_rain: false,
                holographic_effects: false,
            },
            loaders: LoaderStyles {
                primary_type: "hexagon".to_string(),
                button_loader: "slide".to_string(),
                page_loader: "overlay".to_string(),
            },
        }
    }

    pub fn terminal() -> Self {
        Self {
            name: "Terminal Hacker".to_string(),
            variant: ThemeVariant::Terminal,
            colors: ThemeColors {
                primary: "#00ff41".to_string(),        // Matrix green
                secondary: "#008f11".to_string(),      // Dark green
                accent: "#ffffff".to_string(),         // Pure white
                background: "#000000".to_string(),     // Pure black
                surface: "#0d1117".to_string(),        // Dark terminal
                text: "#00ff41".to_string(),           // Matrix green text
                text_secondary: "#00cc33".to_string(), // Dimmer green
                border: "#00ff41".to_string(),         // Matrix green
                success: "#00ff41".to_string(),        // Matrix green
                warning: "#ffff00".to_string(),        // Terminal yellow
                error: "#ff0000".to_string(),          // Terminal red
                info: "#00ffff".to_string(),           // Terminal cyan
            },
            spacing: Self::default_spacing(),
            typography: ThemeTypography {
                font_family: "'Courier New', 'Monaco', 'Inconsolata', monospace".to_string(),
                font_family_mono: "'Courier New', 'Monaco', monospace".to_string(),
                font_size_xs: "0.75rem".to_string(),
                font_size_sm: "0.875rem".to_string(),
                font_size_md: "1rem".to_string(),
                font_size_lg: "1.125rem".to_string(),
                font_size_xl: "1.25rem".to_string(),
                font_weight_normal: "400".to_string(),
                font_weight_medium: "400".to_string(), // Monospace consistency
                font_weight_bold: "700".to_string(),
            },
            borders: ThemeBorders {
                radius_none: "0".to_string(),
                radius_sm: "0".to_string(), // Terminal sharp edges
                radius_md: "0".to_string(),
                radius_lg: "0".to_string(),
                radius_full: "0".to_string(),
                width_thin: "1px".to_string(),
                width_medium: "1px".to_string(), // Consistent terminal borders
                width_thick: "2px".to_string(),
            },
            effects: ThemeEffects {
                shadow_sm: "0 1px 2px rgba(0, 255, 65, 0.3)".to_string(),
                shadow_md: "0 2px 4px rgba(0, 255, 65, 0.4)".to_string(),
                shadow_lg: "0 4px 8px rgba(0, 255, 65, 0.5)".to_string(),
                glow_primary: "0 0 10px rgba(0, 255, 65, 0.8), 0 0 20px rgba(0, 255, 65, 0.4)"
                    .to_string(),
                glow_accent: "0 0 10px rgba(255, 255, 255, 0.8), 0 0 20px rgba(255, 255, 255, 0.4)"
                    .to_string(),
                blur_sm: "blur(1px)".to_string(),
                blur_md: "blur(2px)".to_string(),
            },
            animations: Self::default_animations(),
            decorative: ThemeDecorative {
                corner_decorations: false,
                glow_effects: true,
                scan_lines: false,
                noise_overlay: false,
                hexagonal_elements: false,
                terminal_cursor: true,
                floating_particles: false,
                angular_borders: false,
                matrix_rain: true,
                holographic_effects: false,
            },
            loaders: LoaderStyles {
                primary_type: "dots".to_string(),
                button_loader: "dots".to_string(),
                page_loader: "minimal".to_string(),
            },
        }
    }

    pub fn modern_ui() -> Self {
        Self {
            name: "Modern UI".to_string(),
            variant: ThemeVariant::ModernUI,
            colors: ThemeColors {
                primary: "#3b82f6".to_string(),        // Blue 500
                secondary: "#6366f1".to_string(),      // Indigo 500
                accent: "#8b5cf6".to_string(),         // Violet 500
                background: "#ffffff".to_string(),     // Pure white
                surface: "#f8fafc".to_string(),        // Slate 50
                text: "#0f172a".to_string(),           // Slate 900
                text_secondary: "#64748b".to_string(), // Slate 500
                border: "#e2e8f0".to_string(),         // Slate 200
                success: "#10b981".to_string(),        // Emerald 500
                warning: "#f59e0b".to_string(),        // Amber 500
                error: "#ef4444".to_string(),          // Red 500
                info: "#06b6d4".to_string(),           // Cyan 500
            },
            spacing: Self::default_spacing(),
            typography: ThemeTypography {
                font_family: "'Inter', 'SF Pro Display', 'Segoe UI', system-ui, sans-serif"
                    .to_string(),
                font_family_mono: "'JetBrains Mono', 'SF Mono', 'Consolas', monospace".to_string(),
                font_size_xs: "0.75rem".to_string(),
                font_size_sm: "0.875rem".to_string(),
                font_size_md: "1rem".to_string(),
                font_size_lg: "1.125rem".to_string(),
                font_size_xl: "1.25rem".to_string(),
                font_weight_normal: "400".to_string(),
                font_weight_medium: "500".to_string(),
                font_weight_bold: "600".to_string(),
            },
            borders: ThemeBorders {
                radius_none: "0".to_string(),
                radius_sm: "0.25rem".to_string(),
                radius_md: "0.5rem".to_string(),
                radius_lg: "0.75rem".to_string(),
                radius_full: "9999px".to_string(),
                width_thin: "1px".to_string(),
                width_medium: "1px".to_string(),
                width_thick: "2px".to_string(),
            },
            effects: ThemeEffects {
                shadow_sm: "0 1px 2px 0 rgba(0, 0, 0, 0.05)".to_string(),
                shadow_md: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)"
                    .to_string(),
                shadow_lg:
                    "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)"
                        .to_string(),
                glow_primary: "0 0 0 3px rgba(59, 130, 246, 0.1)".to_string(),
                glow_accent: "0 0 0 3px rgba(139, 92, 246, 0.1)".to_string(),
                blur_sm: "blur(4px)".to_string(),
                blur_md: "blur(8px)".to_string(),
            },
            animations: ThemeAnimations {
                duration_fast: "150ms".to_string(),
                duration_medium: "200ms".to_string(),
                duration_slow: "300ms".to_string(),
                easing_default: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
                easing_bounce: "cubic-bezier(0.34, 1.56, 0.64, 1)".to_string(),
                easing_smooth: "cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string(),
            },
            decorative: ThemeDecorative {
                corner_decorations: false,
                glow_effects: false,
                scan_lines: false,
                noise_overlay: false,
                hexagonal_elements: false,
                terminal_cursor: false,
                floating_particles: false,
                angular_borders: false,
                matrix_rain: false,
                holographic_effects: false,
            },
            loaders: LoaderStyles {
                primary_type: "spinner".to_string(),
                button_loader: "inline-spinner".to_string(),
                page_loader: "minimal".to_string(),
            },
        }
    }

    fn default_spacing() -> ThemeSpacing {
        ThemeSpacing {
            xs: "0.25rem".to_string(),
            sm: "0.5rem".to_string(),
            md: "1rem".to_string(),
            lg: "1.5rem".to_string(),
            xl: "2rem".to_string(),
            xxl: "3rem".to_string(),
        }
    }

    fn default_borders() -> ThemeBorders {
        ThemeBorders {
            radius_none: "0".to_string(),
            radius_sm: "0.125rem".to_string(),
            radius_md: "0.25rem".to_string(),
            radius_lg: "0.5rem".to_string(),
            radius_full: "9999px".to_string(),
            width_thin: "1px".to_string(),
            width_medium: "2px".to_string(),
            width_thick: "3px".to_string(),
        }
    }

    fn default_animations() -> ThemeAnimations {
        ThemeAnimations {
            duration_fast: "150ms".to_string(),
            duration_medium: "300ms".to_string(),
            duration_slow: "500ms".to_string(),
            easing_default: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
            easing_bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            easing_smooth: "cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string(),
        }
    }

    pub fn to_css_variables(&self) -> String {
        format!(
            r#"
            --color-primary: {};
            --color-secondary: {};
            --color-accent: {};
            --color-background: {};
            --color-surface: {};
            --color-text: {};
            --color-text-secondary: {};
            --color-border: {};
            --color-success: {};
            --color-warning: {};
            --color-error: {};
            --color-info: {};

            --spacing-xs: {};
            --spacing-sm: {};
            --spacing-md: {};
            --spacing-lg: {};
            --spacing-xl: {};
            --spacing-xxl: {};

            --font-family: {};
            --font-family-mono: {};
            --font-size-xs: {};
            --font-size-sm: {};
            --font-size-md: {};
            --font-size-lg: {};
            --font-size-xl: {};
            --font-weight-normal: {};
            --font-weight-medium: {};
            --font-weight-bold: {};

            --border-radius-none: {};
            --border-radius-sm: {};
            --border-radius-md: {};
            --border-radius-lg: {};
            --border-radius-full: {};
            --border-width-thin: {};
            --border-width-medium: {};
            --border-width-thick: {};

            --shadow-sm: {};
            --shadow-md: {};
            --shadow-lg: {};
            --glow-primary: {};
            --glow-accent: {};
            --blur-sm: {};
            --blur-md: {};

            --duration-fast: {};
            --duration-medium: {};
            --duration-slow: {};
            --easing-default: {};
            --easing-bounce: {};
            --easing-smooth: {};

            --loader-primary: "{}";
            --loader-button: "{}";
            --loader-page: "{}";
            "#,
            self.colors.primary,
            self.colors.secondary,
            self.colors.accent,
            self.colors.background,
            self.colors.surface,
            self.colors.text,
            self.colors.text_secondary,
            self.colors.border,
            self.colors.success,
            self.colors.warning,
            self.colors.error,
            self.colors.info,
            self.spacing.xs,
            self.spacing.sm,
            self.spacing.md,
            self.spacing.lg,
            self.spacing.xl,
            self.spacing.xxl,
            self.typography.font_family,
            self.typography.font_family_mono,
            self.typography.font_size_xs,
            self.typography.font_size_sm,
            self.typography.font_size_md,
            self.typography.font_size_lg,
            self.typography.font_size_xl,
            self.typography.font_weight_normal,
            self.typography.font_weight_medium,
            self.typography.font_weight_bold,
            self.borders.radius_none,
            self.borders.radius_sm,
            self.borders.radius_md,
            self.borders.radius_lg,
            self.borders.radius_full,
            self.borders.width_thin,
            self.borders.width_medium,
            self.borders.width_thick,
            self.effects.shadow_sm,
            self.effects.shadow_md,
            self.effects.shadow_lg,
            self.effects.glow_primary,
            self.effects.glow_accent,
            self.effects.blur_sm,
            self.effects.blur_md,
            self.animations.duration_fast,
            self.animations.duration_medium,
            self.animations.duration_slow,
            self.animations.easing_default,
            self.animations.easing_bounce,
            self.animations.easing_smooth,
            self.loaders.primary_type,
            self.loaders.button_loader,
            self.loaders.page_loader,
        )
    }

    pub fn get_theme_data_attribute(&self) -> String {
        match self.variant {
            ThemeVariant::NeonEvangelion => "neon-evangelion".to_string(),
            ThemeVariant::Gundam => "gundam".to_string(),
            ThemeVariant::Terminal => "terminal".to_string(),
            ThemeVariant::ModernUI => "modern-ui".to_string(),
        }
    }
}

pub static CURRENT_THEME: GlobalSignal<Theme> = GlobalSignal::new(|| Theme::modern_ui());

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = CURRENT_THEME.read();

    rsx! {
        style {
            ":root {{ {theme.to_css_variables()} }}"
        }
        div {
            class: format!("{}-theme", theme.get_theme_data_attribute()),
            "data-theme": theme.get_theme_data_attribute(),
            "data-decorations": if theme.decorative.corner_decorations { "true" } else { "false" },
            "data-glow": if theme.decorative.glow_effects { "true" } else { "false" },
            "data-scan-lines": if theme.decorative.scan_lines { "true" } else { "false" },
            "data-matrix": if theme.decorative.matrix_rain { "true" } else { "false" },
            {children}
        }
    }
}

pub fn use_theme() -> Theme {
    CURRENT_THEME.read().clone()
}

pub fn switch_theme(theme_variant: ThemeVariant) {
    let new_theme = match theme_variant {
        ThemeVariant::NeonEvangelion => Theme::neon_evangelion(),
        ThemeVariant::Gundam => Theme::gundam(),
        ThemeVariant::Terminal => Theme::terminal(),
        ThemeVariant::ModernUI => Theme::modern_ui(),
    };
    *CURRENT_THEME.write() = new_theme;
}

#[component]
pub fn ThemeSwitcher() -> Element {
    let current_theme = use_theme();

    rsx! {
        div { class: "theme-switcher",
            label { "Theme: " }
            select {
                value: match current_theme.variant {
                    ThemeVariant::NeonEvangelion => "neon-evangelion",
                    ThemeVariant::Gundam => "gundam",
                    ThemeVariant::Terminal => "terminal",
                    ThemeVariant::ModernUI => "modern-ui",
                },
                onchange: move |event| {
                    let variant = match event.data.value().as_str() {
                        "gundam" => ThemeVariant::Gundam,
                        "terminal" => ThemeVariant::Terminal,
                        "modern-ui" => ThemeVariant::ModernUI,
                        _ => ThemeVariant::NeonEvangelion,
                    };
                    switch_theme(variant);
                },

                option { value: "neon-evangelion", "Neon Evangelion" }
                option { value: "gundam", "Gundam Mecha" }
                option { value: "terminal", "Terminal Hacker" }
                option { value: "modern-ui", "Modern UI" }
            }
        }
    }
}

// Backwards compatibility
pub type NeonEvangelionTheme = Theme;

#[component]
pub fn NeonThemeProvider(children: Element) -> Element {
    rsx! { ThemeProvider { {children} } }
}

pub fn use_neon_theme() -> Theme {
    use_theme()
}
