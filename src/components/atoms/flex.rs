use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

impl std::fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexDirection::Row => write!(f, "row"),
            FlexDirection::Column => write!(f, "column"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FlexJustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl std::fmt::Display for FlexJustifyContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexJustifyContent::Start => write!(f, "flex-start"),
            FlexJustifyContent::Center => write!(f, "center"),
            FlexJustifyContent::End => write!(f, "flex-end"),
            FlexJustifyContent::SpaceBetween => write!(f, "space-between"),
            FlexJustifyContent::SpaceAround => write!(f, "space-around"),
            FlexJustifyContent::SpaceEvenly => write!(f, "space-evenly"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FlexAlignItems {
    Start,
    Center,
    End,
}

impl std::fmt::Display for FlexAlignItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlexAlignItems::Start => write!(f, "flex-start"),
            FlexAlignItems::Center => write!(f, "center"),
            FlexAlignItems::End => write!(f, "flex-end"),
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct FlexProps {
    #[props(default)]
    children: Element,

    #[props(default)]
    style: Option<String>,

    #[props(default)]
    class: Option<String>,

    #[props(default = FlexDirection::Row)]
    direction: FlexDirection,

    #[props(default = FlexJustifyContent::Start)]
    justify_content: FlexJustifyContent,

    #[props(default = FlexAlignItems::Start)]
    align_items: FlexAlignItems,
}

pub fn Flex(props: FlexProps) -> Element {
    let mut style_parts = vec![
        "display: flex".to_string(),
        format!("flex-direction: {}", props.direction),
        format!("justify-content: {}", props.justify_content),
        format!("align-items: {}", props.align_items),
    ];

    if let Some(style) = props.style {
        style_parts.push(style);
    }

    let style = style_parts.join("; ") + ";";

    rsx!(
        div { style: "{style}", class: "{props.class}", {props.children} }
    )
}
