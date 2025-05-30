use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridProps {
    #[props(default)]
    children: Element,

    #[props(default)]
    style: Option<String>,

    #[props(default)]
    class: Option<String>,

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

    if let Some(style) = props.style {
        style_parts.push(style);
    }

    let style = style_parts.join("; ") + ";";

    rsx! {
        div { style: "{style}", class: "{props.class}", {props.children} }
    }
}
