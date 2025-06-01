use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SeparatorProps {

    #[props(default = true)]
    horizontal: bool,



    #[props(default = false)]
    decorative: bool,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Separator(props: SeparatorProps) -> Element {
    let orientation = match props.horizontal {
        true => "horizontal",
        false => "vertical",
    };

    rsx! {
        div {
            role: if !props.decorative { "separator" } else { "none" },
            aria_orientation: if !props.decorative { orientation },
            "data-orientation": orientation,
            ..props.attributes,
        }
    }
}
