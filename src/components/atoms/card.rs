use crate::contexts::theme::{use_theme, ThemeVariant};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    /// Enable theme-specific decorations
    #[props(default = false)]
    with_decorations: bool,

    /// Enable theme-specific glow effects
    #[props(default = false)]
    with_glow: bool,

    /// Enable hover animations
    #[props(default = true)]
    hoverable: bool,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let theme = use_theme();
    let mut classes = vec!["card"];

    // Add theme-specific decoration classes
    if props.with_decorations && theme.decorative.corner_decorations {
        classes.push("decorated");

        match theme.variant {
            ThemeVariant::NeonEvangelion => classes.push("neon-decorated"),
        }
    }

    // Add glow effects if enabled in theme
    if props.with_glow && theme.decorative.glow_effects {
        classes.push("glow");
    }

    // Add hover effects
    if props.hoverable {
        classes.push("hoverable");
    }

    // Add theme-specific shape classes - removed since only NeonEvangelion doesn't use hexagonal elements

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            "data-theme": theme.get_theme_data_attribute(),
            "data-decorations": if props.with_decorations { "true" } else { "false" },
            "data-glow": if props.with_glow { "true" } else { "false" },
            ..props.attributes,
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div {
            class: "card-header",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    rsx! {
        div {
            class: "card-content",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            class: "card-footer",
            {props.children}
        }
    }
}
