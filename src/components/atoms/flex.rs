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

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default = FlexDirection::Row)]
    direction: FlexDirection,

    #[props(default = FlexJustifyContent::Start)]
    justify_content: FlexJustifyContent,

    #[props(default = FlexAlignItems::Start)]
    align_items: FlexAlignItems,

    #[props(default = "".to_string())]
    style: String,
}

pub fn Flex(props: FlexProps) -> Element {
    let flex_style = format!(
        "display: flex; flex-direction: {}; justify-content: {}; align-items: {}; {}",
        props.direction, props.justify_content, props.align_items, props.style
    );

    let filtered_attributes: Vec<Attribute> = props
        .attributes
        .into_iter()
        .filter(|attr| attr.name != "style")
        .collect();

    rsx!(
        div { style: "{flex_style}", ..filtered_attributes, {props.children} }
    )
}
