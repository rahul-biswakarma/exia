use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThemeVariant {
    NeonEvangelion,
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
            ThemeVariant::NeonEvangelion => "neonevangelion".to_string(),
        }
    }
}

pub static CURRENT_THEME: GlobalSignal<Theme> = GlobalSignal::new(|| Theme::neon_evangelion());

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let theme = CURRENT_THEME.read();

    // Apply theme to document body as well - this needs to be reactive
    use_effect(use_reactive((&theme.variant,), move |(variant,)| {
        let current_theme = CURRENT_THEME.read();
        let theme_attr = match variant {
            ThemeVariant::NeonEvangelion => "neonevangelion",
        };
        let css_variables = current_theme.to_css_variables();

        #[cfg(target_arch = "wasm32")]
        {
            use dioxus::document::eval;
            let js = eval(&format!(
                r#"
                document.body.setAttribute('data-theme', '{}');
                document.body.className = '{}-theme';

                // Apply CSS variables to root
                const root = document.documentElement;
                const variables = `{}`;
                const tempStyle = document.createElement('style');
                tempStyle.textContent = `:root {{ ${{variables}} }}`;

                // Remove previous theme styles
                const existingThemeStyle = document.getElementById('theme-variables');
                if (existingThemeStyle) {{
                    existingThemeStyle.remove();
                }}

                // Add new theme styles
                tempStyle.id = 'theme-variables';
                document.head.appendChild(tempStyle);
                "#,
                theme_attr,
                theme_attr,
                css_variables.replace('\n', "\\n").replace('"', "\\\"")
            ));
            let _ = js;
        }
    }));

    rsx! {
        div {
            class: format!("{}-theme", theme.get_theme_data_attribute()),
            "data-theme": theme.get_theme_data_attribute(),
            "data-decorations": if theme.decorative.corner_decorations { "true" } else { "false" },
            "data-glow": if theme.decorative.glow_effects { "true" } else { "false" },
            "data-scan-lines": if theme.decorative.scan_lines { "true" } else { "false" },
            "data-matrix": if theme.decorative.matrix_rain { "true" } else { "false" },
            "data-hexagonal": if theme.decorative.hexagonal_elements { "true" } else { "false" },
            "data-terminal-cursor": if theme.decorative.terminal_cursor { "true" } else { "false" },
            "data-floating-particles": if theme.decorative.floating_particles { "true" } else { "false" },
            "data-angular-borders": if theme.decorative.angular_borders { "true" } else { "false" },
            "data-holographic": if theme.decorative.holographic_effects { "true" } else { "false" },
            "data-noise-overlay": if theme.decorative.noise_overlay { "true" } else { "false" },
            {children}
        }
    }
}

pub fn use_theme() -> Theme {
    CURRENT_THEME.read().clone()
}

pub fn switch_theme(theme_variant: ThemeVariant) {
    dioxus::logger::tracing::info!("switch_theme called with variant: {:?}", theme_variant);

    let new_theme = match theme_variant {
        ThemeVariant::NeonEvangelion => Theme::neon_evangelion(),
    };

    dioxus::logger::tracing::info!("Updating CURRENT_THEME to: {}", new_theme.name);
    *CURRENT_THEME.write() = new_theme.clone();

    // Update document body for web platform
    #[cfg(target_arch = "wasm32")]
    {
        use dioxus::document::eval;
        let theme_attr = new_theme.get_theme_data_attribute();
        dioxus::logger::tracing::info!("Setting data-theme attribute to: {}", theme_attr);
        let js = eval(&format!(
            r#"
            console.log('Setting theme attributes:', '{}');
            document.body.setAttribute('data-theme', '{}');
            document.body.className = '{}-theme';
            console.log('Theme attributes set successfully');
            "#,
            theme_attr, theme_attr, theme_attr
        ));
        let _ = js;
    }
}

#[component]
pub fn ThemeSwitcher() -> Element {
    let current_theme = CURRENT_THEME.read();

    rsx! {
        div { class: "theme-switcher",
            label { "Theme: " }
            select {
                class: "select",
                value: match current_theme.variant {
                    ThemeVariant::NeonEvangelion => "neonevangelion",
                },
                onchange: move |event| {
                    let selected_value = event.data.value();
                    dioxus::logger::tracing::info!("Theme switcher changed to: {}", selected_value);

                    let variant = match selected_value.as_str() {
                        "neonevangelion" => ThemeVariant::NeonEvangelion,
                        _ => {
                            dioxus::logger::tracing::warn!("Unknown theme variant: {}", selected_value);
                            ThemeVariant::NeonEvangelion
                        }
                    };

                    dioxus::logger::tracing::info!("Switching to theme variant: {:?}", variant);
                    switch_theme(variant);
                },

                option { value: "neonevangelion", "Neon Evangelion" }
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
