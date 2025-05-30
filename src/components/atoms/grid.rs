use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    #[props(default)]
    children: Element,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(default)]
    column: Option<String>,

    #[props(default)]
    row: Option<String>,
}

pub fn Grid(props: GridProps) -> Element {
    let mut style_parts = vec!["display: grid".to_string()];

    if let Some(column) = props.column {
        style_parts.push(format!("grid-template-columns: {}", column));
    }

    if let Some(row) = props.row {
        style_parts.push(format!("grid-template-rows: {}", row));
    }

    if let Some(style_attr) = props.attributes.iter().find(|attr| attr.name == "style") {
        style_parts.push(style_attr.value.to_string());
    }

    let style = style_parts.join("; ") + ";";

    let filtered_attributes: Vec<Attribute> = props
        .attributes
        .into_iter()
        .filter(|attr| attr.name != "style")
        .collect();

    rsx! {
		div { style: "{style}", ..filtered_attributes, {props.children} }
	}
}
