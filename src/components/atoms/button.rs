use crate::contexts::theme::{use_theme, ThemeVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Button variant style
    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,

    /// Button size
    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,

    /// Enable theme-specific glow effects
    #[props(default = false)]
    with_glow: bool,

    /// Enable theme-specific decorations
    #[props(default = false)]
    with_decorations: bool,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    loading: ReadOnlySignal<bool>,

    #[props(default)]
    onclick: Callback<Event<MouseData>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Success,
    Warning,
    Error,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let theme = use_theme();
    let mut classes = vec!["btn"];

    // Add variant classes
    match props.variant {
        ButtonVariant::Primary => classes.push("btn-primary"),
        ButtonVariant::Secondary => classes.push("btn-secondary"),
        ButtonVariant::Outline => classes.push("btn-outline"),
        ButtonVariant::Ghost => classes.push("btn-ghost"),
        ButtonVariant::Success => classes.push("btn-success"),
        ButtonVariant::Warning => classes.push("btn-warning"),
        ButtonVariant::Error => classes.push("btn-error"),
    }

    // Add size classes
    match props.size {
        ButtonSize::Small => classes.push("btn-sm"),
        ButtonSize::Medium => {} // Default size
        ButtonSize::Large => classes.push("btn-lg"),
    }

    // Add theme-specific classes
    if props.with_glow && theme.decorative.glow_effects {
        classes.push("btn-glow");
    }

    if props.with_decorations {
        match theme.variant {
            ThemeVariant::NeonEvangelion => classes.push("neon-glow"),
        }
    }

    // Add state classes
    if let Some(class) = &props.class {
        classes.push(class);
    }

    if *props.disabled.read() {
        classes.push("disabled");
    }

    if *props.loading.read() {
        classes.push("btn-loading");
    }

    let final_class = classes.join(" ");

    // Get theme-specific loader type
    let loader_type = if *props.loading.read() {
        Some(theme.loaders.button_loader.clone())
    } else {
        None
    };

    rsx! {
        button {
            class: final_class,
            disabled: *props.disabled.read() || *props.loading.read(),
            onclick: props.onclick,
            "data-loader": loader_type.as_deref().unwrap_or(""),
            "data-theme": theme.get_theme_data_attribute(),
            ..props.attributes,

            if !*props.loading.read() {
                {props.children}
            } else {
                // Show loading content based on theme
                match theme.variant {
                    ThemeVariant::NeonEvangelion => rsx! {
                        span { class: "neon-text", "PROCESSING..." }
                    },
                }
            }
        }
    }
}
