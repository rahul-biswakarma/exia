use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    /// Button variant style
    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,

    /// Button size
    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,

    /// Enable glow effects (controlled by theme/CSS)
    #[props(default = false)]
    glow: bool,

    /// Enable decorations (controlled by theme/CSS)
    #[props(default = false)]
    decorated: bool,

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

    // Add feature classes (theme-controlled via CSS)
    if props.glow {
        classes.push("btn-glow");
    }

    if props.decorated {
        classes.push("btn-decorated");
    }

    // Add state classes
    if let Some(class) = &props.class {
        classes.push(class);
    }

    if *props.disabled.read() {
        classes.push("btn-disabled");
    }

    if *props.loading.read() {
        classes.push("btn-loading");
    }

    let final_class = classes.join(" ");

    rsx! {
        button {
            class: final_class,
            disabled: *props.disabled.read() || *props.loading.read(),
            onclick: props.onclick,
            ..props.attributes,

            if !*props.loading.read() {
                {props.children}
            } else {
                // Show loading content - theme controls appearance via CSS
                span {
                    class: "btn-loading-content",
                    if let Some(text) = &props.loading_text {
                        "{text}"
                    } else {
                        span { class: "btn-loading-default", "Loading..." }
                    }
                }
            }
        }
    }
}
