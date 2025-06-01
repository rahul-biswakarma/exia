use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {

    #[props(default = 0.0)]
    value: f64,


    #[props(default = 100.0)]
    max: f64,


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
