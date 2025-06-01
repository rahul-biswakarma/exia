use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {

    #[props(default = ButtonVariant::Primary)]
    variant: ButtonVariant,


    #[props(default = ButtonSize::Medium)]
    size: ButtonSize,


    #[props(default = false)]
    glow: bool,


    #[props(default = false)]
    decorated: bool,


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


    match props.variant {
        ButtonVariant::Primary => classes.push("btn-primary"),
        ButtonVariant::Secondary => classes.push("btn-secondary"),
        ButtonVariant::Outline => classes.push("btn-outline"),
        ButtonVariant::Ghost => classes.push("btn-ghost"),
        ButtonVariant::Success => classes.push("btn-success"),
        ButtonVariant::Warning => classes.push("btn-warning"),
        ButtonVariant::Error => classes.push("btn-error"),
    }


    match props.size {
        ButtonSize::Small => classes.push("btn-sm"),
        ButtonSize::Medium => {}
        ButtonSize::Large => classes.push("btn-lg"),
    }


    if props.glow {
        classes.push("btn-glow");
    }

    if props.decorated {
        classes.push("btn-decorated");
    }


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
