use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LabelProps {
    html_for: ReadOnlySignal<String>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Label(props: LabelProps) -> Element {
    rsx! {
        label { r#for: props.html_for, ..props.attributes, {props.children} }
    }
}
