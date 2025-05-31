use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
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

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut classes = vec!["btn"];

    if let Some(class) = &props.class {
        classes.push(class);
    }

    if *props.disabled.read() {
        classes.push("disabled");
    }

    if *props.loading.read() {
        classes.push("loading");
    }

    let final_class = classes.join(" ");

    rsx! {
        button {
            class: final_class,
            disabled: *props.disabled.read(),
            onclick: props.onclick,
            ..props.attributes,

            if !*props.loading.read() {
                {props.children}
            }
        }
    }
}
