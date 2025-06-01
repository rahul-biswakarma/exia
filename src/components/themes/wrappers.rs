use crate::components::themes::context::{use_theme, ThemeVariant};
use dioxus::prelude::*;

use crate::components::atoms::button::{Button, ButtonSize, ButtonVariant};
use crate::components::atoms::card::Card;
use crate::components::atoms::loader::{InlineLoader, Loader, LoaderType, PageLoader};


#[derive(Props, Clone, PartialEq)]
pub struct ThemedButtonProps {

    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,


    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,


    #[props(default)]
    glow_override: Option<bool>,


    #[props(default)]
    decorated_override: Option<bool>,


    #[props(default)]
    loading_text: Option<String>,


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


    let glow = props.glow_override.unwrap_or(theme.decorative.glow_effects);
    let decorated = props
        .decorated_override
        .unwrap_or(theme.decorative.corner_decorations);


    let loading_text = if props.loading_text.is_some() {
        props.loading_text.clone()
    } else if *props.loading.read() {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.button_loading_text.clone()),
            #[allow(unreachable_patterns)]
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


#[derive(Props, Clone, PartialEq)]
pub struct ThemedCardProps {

    #[props(default)]
    decorated_override: Option<bool>,


    #[props(default)]
    glow_override: Option<bool>,


    #[props(default = true)]
    hoverable: bool,


    #[props(default)]
    class: Option<String>,

    children: Element,
}

#[component]
pub fn ThemedCard(props: ThemedCardProps) -> Element {
    let theme = use_theme();


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


#[derive(Props, Clone, PartialEq)]
pub struct ThemedLoaderProps {

    #[props(default = LoaderContext::Primary)]
    context: LoaderContext,


    #[props(default)]
    loader_type_override: Option<LoaderType>,


    #[props(default = crate::components::atoms::loader::LoaderSize::Medium)]
    size: crate::components::atoms::loader::LoaderSize,


    #[props(default)]
    text: Option<String>,


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


    let loader_type = props
        .loader_type_override
        .unwrap_or_else(|| props.context.get_theme_loader_type(&theme));


    let loading_text = if props.text.is_some() {
        props.text.clone()
    } else {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.loader_syncing_text.clone()),
            #[allow(unreachable_patterns)]
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


#[derive(Props, Clone, PartialEq)]
pub struct ThemedPageLoaderProps {

    #[props(default = true)]
    visible: bool,


    #[props(default)]
    loader_type_override: Option<LoaderType>,


    #[props(default)]
    text: Option<String>,


    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn ThemedPageLoader(props: ThemedPageLoaderProps) -> Element {
    let theme = use_theme();


    let loader_type = props
        .loader_type_override
        .unwrap_or_else(|| LoaderContext::Page.get_theme_loader_type(&theme));


    let loading_text = if props.text.is_some() {
        props.text.clone()
    } else {
        match theme.variant {
            ThemeVariant::NeonEvangelion => Some(theme.component_texts.loader_initializing_text.clone()),
            #[allow(unreachable_patterns)]
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


#[derive(Props, Clone, PartialEq)]
pub struct ThemedInlineLoaderProps {

    #[props(default)]
    loader_type_override: Option<LoaderType>,


    #[props(default = crate::components::atoms::loader::LoaderSize::Small)]
    size: crate::components::atoms::loader::LoaderSize,


    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn ThemedInlineLoader(props: ThemedInlineLoaderProps) -> Element {
    let theme = use_theme();


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
