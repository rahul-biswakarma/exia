use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    /// The current progress value, between 0 and max
    #[props(default = 0.0)]
    value: f64,

    /// The maximum value. Defaults to 100
    #[props(default = 100.0)]
    max: f64,

    /// CSS class names to apply
    #[props(default)]
    class: Option<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let percentage = (props.value / props.max) * 100.0;

    let mut classes = vec!["progress"];

    if let Some(class) = &props.class {
        classes.push(class);
    }

    let final_class = classes.join(" ");

    rsx! {
        div {
            class: final_class,
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": props.max,
            "aria-valuenow": props.value,
            "data-state": if props.value > 0.0 { "loading" } else { "indeterminate" },
            "data-value": props.value,
            "data-max": props.max,
            ..props.attributes,

            div {
                class: "progress-bar",
                style: format!("width: {}%;", percentage)
            }
        }
    }
}

/// The indicator that represents the progress visually
#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        div { ..props.attributes }
    }
}
