use crate::components::themes::context::{use_theme, ThemeVariant};
use dioxus::prelude::*;

use crate::components::atoms::button::{Button, ButtonSize, ButtonVariant};
use crate::components::atoms::card::Card;
use crate::components::atoms::loader::{InlineLoader, Loader, LoaderType, PageLoader};

/// Theme-aware Button wrapper that automatically applies theme settings
#[derive(Props, Clone, PartialEq)]
pub struct ThemedButtonProps {
    /// Button variant style
    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,

    /// Button size
    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,

    /// Override theme glow setting
    #[props(default)]
    glow_override: Option<bool>,

    /// Override theme decorations setting
    #[props(default)]
    decorated_override: Option<bool>,

    /// Loading state text override
    #[props(default)]
    loading_text: Option<String>,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    #[props(default)]
    disabled: ReadOnlySignal<bool>,

    #[props(default)]
    loading: ReadOnlySignal<bool>,

    #[props(default)]
    onclick: Callback<Event<MouseData>>,

    children: Element,
}

#[component]
pub fn ThemedButton(props: ThemedButtonProps) -> Element {
    let theme = use_theme();

    // Determine glow and decoration settings from theme or override
    let glow = props.glow_override.unwrap_or(theme.decorative.glow_effects);
    let decorated = props
        .decorated_override
        .unwrap_or(theme.decorative.corner_decorations);

    // Get theme-specific loading text if not provided
    let loading_text = if props.loading_text.is_some() {
        props.loading_text.clone()
    } else if *props.loading.read() {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.button_loading_text.clone()),
            #[allow(unreachable_patterns)] // Allows for future themes not using this
            _ => None,
        }
    } else {
        None
    };

    rsx! {
        Button {
            variant: props.variant,
            size: props.size,
            glow: glow,
            decorated: decorated,
            loading_text: loading_text,
            class: props.class,
            disabled: props.disabled,
            loading: props.loading,
            onclick: props.onclick,
            {props.children}
        }
    }
}

/// Theme-aware Card wrapper that automatically applies theme settings
#[derive(Props, Clone, PartialEq)]
pub struct ThemedCardProps {
    /// Override theme decorations setting
    #[props(default)]
    decorated_override: Option<bool>,

    /// Override theme glow setting
    #[props(default)]
    glow_override: Option<bool>,

    /// Enable hover animations
    #[props(default = true)]
    hoverable: bool,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    children: Element,
}

#[component]
pub fn ThemedCard(props: ThemedCardProps) -> Element {
    let theme = use_theme();

    // Determine settings from theme or override
    let decorated = props
        .decorated_override
        .unwrap_or(theme.decorative.corner_decorations);
    let glow = props.glow_override.unwrap_or(theme.decorative.glow_effects);

    rsx! {
        Card {
            decorated: decorated,
            glow: glow,
            hoverable: props.hoverable,
            class: props.class,
            {props.children}
        }
    }
}

/// Theme-aware Loader wrapper that automatically selects loader type from theme
#[derive(Props, Clone, PartialEq)]
pub struct ThemedLoaderProps {
    /// Loader usage context
    #[props(default = LoaderContext::Primary)]
    context: LoaderContext,

    /// Override theme loader type
    #[props(default)]
    loader_type_override: Option<LoaderType>,

    /// Size of the loader
    #[props(default = crate::components::atoms::loader::LoaderSize::Medium)]
    size: crate::components::atoms::loader::LoaderSize,

    /// Optional loading text override
    #[props(default)]
    text: Option<String>,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoaderContext {
    Primary,
    Button,
    Page,
}

impl LoaderContext {
    fn get_theme_loader_type(self, theme: &crate::components::themes::context::Theme) -> LoaderType {
        match self {
            LoaderContext::Primary => match theme.loaders.primary_type.as_str() {
                "spinner" => LoaderType::Spinner,
                "dots" => LoaderType::Dots,
                "bars" => LoaderType::Bars,
                "pulse" => LoaderType::Pulse,
                "slide" => LoaderType::Slide,
                "ring" => LoaderType::Ring,
                "wave" => LoaderType::Wave,
                "custom" => LoaderType::Custom,
                _ => LoaderType::Spinner,
            },
            LoaderContext::Button => match theme.loaders.button_loader.as_str() {
                "spinner" => LoaderType::Spinner,
                "dots" => LoaderType::Dots,
                "pulse" => LoaderType::Pulse,
                "slide" => LoaderType::Slide,
                _ => LoaderType::Pulse,
            },
            LoaderContext::Page => match theme.loaders.page_loader.as_str() {
                "spinner" => LoaderType::Spinner,
                "ring" => LoaderType::Ring,
                "custom" => LoaderType::Custom,
                _ => LoaderType::Spinner,
            },
        }
    }
}

#[component]
pub fn ThemedLoader(props: ThemedLoaderProps) -> Element {
    let theme = use_theme();

    // Determine loader type from theme or override
    let loader_type = props
        .loader_type_override
        .unwrap_or_else(|| props.context.get_theme_loader_type(&theme));

    // Get theme-specific loading text if not provided
    let loading_text = if props.text.is_some() {
        props.text.clone()
    } else {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.loader_syncing_text.clone()),
            #[allow(unreachable_patterns)] // Allows for future themes not using this
            _ => None,
        }
    };

    rsx! {
        Loader {
            loader_type: loader_type,
            size: props.size,
            text: loading_text,
            class: props.class,
        }
    }
}

/// Theme-aware PageLoader wrapper
#[derive(Props, Clone, PartialEq)]
pub struct ThemedPageLoaderProps {
    /// Whether the page loader is visible
    #[props(default = true)]
    visible: bool,

    /// Override theme loader type
    #[props(default)]
    loader_type_override: Option<LoaderType>,

    /// Optional custom loading text
    #[props(default)]
    text: Option<String>,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn ThemedPageLoader(props: ThemedPageLoaderProps) -> Element {
    let theme = use_theme();

    // Determine loader type from theme or override
    let loader_type = props
        .loader_type_override
        .unwrap_or_else(|| LoaderContext::Page.get_theme_loader_type(&theme));

    // Get theme-specific loading text if not provided
    let loading_text = if props.text.is_some() {
        props.text.clone()
    } else {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.loader_initializing_text.clone()),
            #[allow(unreachable_patterns)] // Allows for future themes not using this
            _ => None,
        }
    };

    rsx! {
        PageLoader {
            visible: props.visible,
            loader_type: loader_type,
            text: loading_text,
            class: props.class,
        }
    }
}

/// Theme-aware InlineLoader wrapper
#[derive(Props, Clone, PartialEq)]
pub struct ThemedInlineLoaderProps {
    /// Override theme loader type
    #[props(default)]
    loader_type_override: Option<LoaderType>,

    /// Size of the inline loader
    #[props(default = crate::components::atoms::loader::LoaderSize::Small)]
    size: crate::components::atoms::loader::LoaderSize,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn ThemedInlineLoader(props: ThemedInlineLoaderProps) -> Element {
    let theme = use_theme();

    // Determine loader type from theme or override
    let loader_type = props
        .loader_type_override
        .unwrap_or_else(|| LoaderContext::Button.get_theme_loader_type(&theme));

    rsx! {
        InlineLoader {
            loader_type: loader_type,
            size: props.size,
            class: props.class,
        }
    }
}
