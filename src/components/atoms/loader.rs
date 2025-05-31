use crate::contexts::theme::{use_theme, ThemeVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LoaderProps {
    /// Type of loader to display
    #[props(default = LoaderType::Primary)]
    loader_type: LoaderType,

    /// Size of the loader
    #[props(default = LoaderSize::Medium)]
    size: LoaderSize,

    /// Optional loading text
    #[props(default)]
    text: Option<String>,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderType {
    Primary, // Uses theme's primary loader style
    Button,  // Uses theme's button loader style
    Page,    // Uses theme's page loader style
    Dots,    // Dot-based loader
    Spinner, // Spinner loader
    Bars,    // Bar-based loader
    Hexagon, // Hexagonal loader (Gundam/Neon themes)
    Matrix,  // Matrix-style loader (Terminal theme)
    Minimal, // Minimal loader (Modern UI theme)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Loader(props: LoaderProps) -> Element {
    let theme = use_theme();
    let mut classes = vec!["loader"];

    // Determine the actual loader style based on theme and type
    let actual_loader_type = match props.loader_type {
        LoaderType::Primary => match theme.loaders.primary_type.as_str() {
            "spinner" => LoaderType::Spinner,
            "dots" => LoaderType::Dots,
            "bars" => LoaderType::Bars,
            "hexagon" => LoaderType::Hexagon,
            "matrix" => LoaderType::Matrix,
            "minimal" => LoaderType::Minimal,
            _ => LoaderType::Spinner,
        },
        LoaderType::Button => match theme.loaders.button_loader.as_str() {
            "inline-spinner" => LoaderType::Spinner,
            "dots" => LoaderType::Dots,
            "pulse" => LoaderType::Minimal,
            "slide" => LoaderType::Bars,
            _ => LoaderType::Spinner,
        },
        LoaderType::Page => match theme.loaders.page_loader.as_str() {
            "full-screen" => LoaderType::Spinner,
            "overlay" => LoaderType::Hexagon,
            "minimal" => LoaderType::Minimal,
            _ => LoaderType::Spinner,
        },
        other => other,
    };

    // Add type-specific class
    match actual_loader_type {
        LoaderType::Spinner => classes.push("loader-spinner"),
        LoaderType::Dots => classes.push("loader-dots"),
        LoaderType::Bars => classes.push("loader-bars"),
        LoaderType::Hexagon => classes.push("loader-hexagon"),
        LoaderType::Matrix => classes.push("loader-matrix"),
        LoaderType::Minimal => classes.push("loader-minimal"),
        _ => classes.push("loader-spinner"),
    }

    // Add size class
    match props.size {
        LoaderSize::Small => classes.push("loader-small"),
        LoaderSize::Medium => classes.push("loader-medium"),
        LoaderSize::Large => classes.push("loader-large"),
    }

    // Add theme-specific class
    match theme.variant {
        ThemeVariant::NeonEvangelion => classes.push("neon-loader"),
    }

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    // Generate loading text based on theme if not provided
    let loading_text = props.text.clone().unwrap_or_else(|| match theme.variant {
        ThemeVariant::NeonEvangelion => "SYNCING...".to_string(),
    });

    rsx! {
        div {
            class: final_class,
            "data-theme": theme.get_theme_data_attribute(),
            "data-loader-type": format!("{:?}", actual_loader_type).to_lowercase(),
            ..props.attributes,

            // Render different loader types
            match actual_loader_type {
                LoaderType::Spinner => rsx! {
                    div { class: "spinner-ring" }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Dots => rsx! {
                    div { class: "dots-container",
                        div { class: "dot" }
                        div { class: "dot" }
                        div { class: "dot" }
                    }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Bars => rsx! {
                    div { class: "bars-container",
                        div { class: "bar" }
                        div { class: "bar" }
                        div { class: "bar" }
                        div { class: "bar" }
                    }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Hexagon => rsx! {
                    div { class: "hexagon-container",
                        div { class: "hexagon outer" }
                        div { class: "hexagon inner" }
                    }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                LoaderType::Matrix => rsx! {
                    div { class: "matrix-container",
                        div { class: "matrix-char", "0" }
                        div { class: "matrix-char", "1" }
                        div { class: "matrix-char", "0" }
                        div { class: "matrix-char", "1" }
                    }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text terminal-cursor", "{loading_text}" }
                    }
                },
                LoaderType::Minimal => rsx! {
                    div { class: "minimal-container",
                        div { class: "minimal-bar" }
                    }
                    if props.text.is_some() || matches!(props.loader_type, LoaderType::Page) {
                        div { class: "loader-text", "{loading_text}" }
                    }
                },
                _ => rsx! {
                    div { class: "spinner-ring" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PageLoaderProps {
    /// Whether the page loader is visible
    #[props(default = true)]
    visible: bool,

    /// Optional custom loading text
    #[props(default)]
    text: Option<String>,
}

#[component]
pub fn PageLoader(props: PageLoaderProps) -> Element {
    let theme = use_theme();

    if !props.visible {
        return rsx! { div { style: "display: none;" } };
    }

    let overlay_class = match theme.variant {
        ThemeVariant::NeonEvangelion => "page-loader-overlay neon-overlay",
    };

    rsx! {
        div {
            class: overlay_class,
            "data-theme": theme.get_theme_data_attribute(),

            Loader {
                loader_type: LoaderType::Page,
                size: LoaderSize::Large,
                text: props.text,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InlineLoaderProps {
    /// Size of the inline loader
    #[props(default = LoaderSize::Small)]
    size: LoaderSize,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn InlineLoader(props: InlineLoaderProps) -> Element {
    rsx! {
        Loader {
            loader_type: LoaderType::Button,
            size: props.size,
            class: props.class,
        }
    }
}
