use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeonEvangelionTheme {
    pub name: String,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub typography: ThemeTypography,
    pub borders: ThemeBorders,
    pub effects: ThemeEffects,
    pub animations: ThemeAnimations,
    pub decorative: ThemeDecorative,
}

impl NeonEvangelionTheme {
    pub fn new() -> Self {
        Self {
            name: "Neon Evangelion".to_string(),
            colors: ThemeColors {
                primary: "#ff6b35".to_string(),        // Terminal orange
                secondary: "#00ff66".to_string(),      // Neon green
                accent: "#ffcc00".to_string(),         // Electric yellow
                background: "#0a0a0f".to_string(),     // Deep void
                surface: "#1a1a2e".to_string(),        // Dark purple
                text: "#ffffff".to_string(),           // Pure white
                text_secondary: "#cccccc".to_string(), // Light gray
                border: "#ff6b35".to_string(),         // Terminal orange border
                success: "#00ff66".to_string(),        // Neon green
                warning: "#ffcc00".to_string(),        // Electric yellow
                error: "#ff3366".to_string(),          // Electric red
                info: "#00ffcc".to_string(),           // Neon cyan
            },
            spacing: ThemeSpacing {
                xs: "0.25rem".to_string(),
                sm: "0.5rem".to_string(),
                md: "1rem".to_string(),
                lg: "1.5rem".to_string(),
                xl: "2rem".to_string(),
                xxl: "3rem".to_string(),
            },
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
            borders: ThemeBorders {
                radius_none: "0".to_string(),
                radius_sm: "0.125rem".to_string(),
                radius_md: "0.25rem".to_string(),
                radius_lg: "0.5rem".to_string(),
                radius_full: "9999px".to_string(),
                width_thin: "1px".to_string(),
                width_medium: "2px".to_string(),
                width_thick: "3px".to_string(),
            },
            effects: ThemeEffects {
                shadow_sm: "0 1px 2px rgba(255, 107, 53, 0.3)".to_string(),
                shadow_md: "0 4px 8px rgba(255, 107, 53, 0.4)".to_string(),
                shadow_lg: "0 8px 16px rgba(255, 107, 53, 0.5)".to_string(),
                glow_primary: "0 0 20px rgba(255, 107, 53, 0.8), 0 0 40px rgba(255, 107, 53, 0.4)"
                    .to_string(),
                glow_accent: "0 0 20px rgba(255, 204, 0, 0.8), 0 0 40px rgba(255, 204, 0, 0.4)"
                    .to_string(),
                blur_sm: "blur(4px)".to_string(),
                blur_md: "blur(8px)".to_string(),
            },
            animations: ThemeAnimations {
                duration_fast: "150ms".to_string(),
                duration_medium: "300ms".to_string(),
                duration_slow: "500ms".to_string(),
                easing_default: "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
                easing_bounce: "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
                easing_smooth: "cubic-bezier(0.25, 0.46, 0.45, 0.94)".to_string(),
            },
            decorative: ThemeDecorative {
                corner_decorations: true,
                glow_effects: true,
                scan_lines: true,
                noise_overlay: true,
                hexagonal_elements: false,
                terminal_cursor: true,
                floating_particles: true,
            },
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
        )
    }
}

impl Default for NeonEvangelionTheme {
    fn default() -> Self {
        Self::new()
    }
}

pub static NEON_THEME: GlobalSignal<NeonEvangelionTheme> =
    GlobalSignal::new(|| NeonEvangelionTheme::new());

#[component]
pub fn NeonThemeProvider(children: Element) -> Element {
    let theme = NEON_THEME.read();

    rsx! {
        style {
            ":root {{ {theme.to_css_variables()} }}"
        }
        div {
            class: "neon-evangelion-theme",
            "data-theme": "neonevangelion",
            {children}
        }
    }
}

// Hook for accessing the neon theme
pub fn use_neon_theme() -> NeonEvangelionTheme {
    NEON_THEME.read().clone()
}
