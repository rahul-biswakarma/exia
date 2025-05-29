use crate::components::synapse::schema_parser::{
    Atom, Layout, LayoutElement, RootSchema, UiElement,
};
use dioxus::prelude::*;

pub fn render_ui_from_schema(schema_json: &str) -> Element {
    match serde_json::from_str::<RootSchema>(schema_json) {
        Ok(root_schema) => {
            rsx! {
                for element in root_schema.ui_elements {
                    {render_ui_element(element)}
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing schema JSON: {:?}", e);
            rsx! {
                div { "Error parsing schema" }
            }
        }
    }
}

pub fn render_ui_element(element: UiElement) -> Element {
    match element {
        UiElement::Atom(atom) => render_atom(atom),
        UiElement::Layout(layout) => render_layout(layout),
    }
}

pub fn render_atom(atom: Atom) -> Element {
    match atom.type_name.as_str() {
        "label" => {
            if let Some(text_value_json) = atom.properties.get("text") {
                if let Some(text_str) = text_value_json.as_str() {
                    rsx! {
                        label { "{text_str}" }
                    }
                } else {
                    eprintln!("Warning: Atom 'label' has 'text' property but it's not a string. Atom: {:?}", atom);
                    rsx! {
                        div {}
                    }
                }
            } else {
                eprintln!(
                    "Warning: Atom 'label' is missing 'text' property. Atom: {:?}",
                    atom
                );
                rsx! {
                    div {}
                }
            }
        }
        "portal" => {
            rsx! {
                div {}
            }
        }
        "separator" => {
            rsx! {
                hr {}
            }
        }
        "checkbox" | "switch" => {
            let class_name = if atom.type_name == "switch" {
                "switch"
            } else {
                ""
            };
            let checked = atom
                .properties
                .get("checked")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let disabled = atom
                .properties
                .get("disabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let value = atom
                .properties
                .get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let id = atom
                .properties
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            rsx! {
                input {
                    r#type: "checkbox",
                    class: if !class_name.is_empty() { class_name } else { "" },
                    checked,
                    disabled,
                    value,
                    id,
                }
            }
        }
        "avatar" => {
            let src = atom
                .properties
                .get("src")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let alt = atom
                .properties
                .get("alt")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let size = atom
                .properties
                .get("size")
                .and_then(|v| v.as_u64())
                .unwrap_or(32);

            if src.is_empty() {
                eprintln!(
                    "Warning: Atom 'avatar' is missing 'src' property. Atom: {:?}",
                    atom
                );
            }

            rsx! {
                img {
                    src,
                    alt,
                    width: "{size}px",
                    height: "{size}px",
                }
            }
        }
        "progress" => {
            let value = atom.properties.get("value").and_then(|v| v.as_f64());
            let max = atom.properties.get("max").and_then(|v| v.as_f64());

            rsx! {
                progress {
                    value: value.map(|v| v.to_string()).unwrap_or_default(),
                    max: max.map(|v| v.to_string()).unwrap_or_default(),
                }
            }
        }
        "slider" => {
            let value = atom.properties.get("value").and_then(|v| v.as_f64());
            let min = atom.properties.get("min").and_then(|v| v.as_f64());
            let max = atom.properties.get("max").and_then(|v| v.as_f64());
            let step = atom.properties.get("step").and_then(|v| v.as_f64());
            let disabled = atom
                .properties
                .get("disabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let id = atom
                .properties
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            rsx! {
                input {
                    r#type: "range",
                    value: value.map(|v| v.to_string()).unwrap_or_default(),
                    min: min.map(|v| v.to_string()).unwrap_or_default(),
                    max: max.map(|v| v.to_string()).unwrap_or_default(),
                    step: step.map(|v| v.to_string()).unwrap_or_default(),
                    disabled,
                    id,
                }
            }
        }
        unknown_type => {
            eprintln!(
                "Warning: Unknown atom type encountered: {}. Atom: {:?}",
                unknown_type, atom
            );
            rsx! {
                div {}
            }
        }
    }
}

pub fn render_layout(layout: Layout) -> Element {
    match layout.layout_type.as_str() {
        "grid" => {
            let mut container_style_parts: Vec<String> = vec![
                "display: grid;".to_string(),
                format!("grid-template-rows: repeat({}, 1fr);", layout.rows),
                format!("grid-template-columns: repeat({}, 1fr);", layout.cols),
            ];

            if let Some(gap) = layout.gap {
                container_style_parts.push(format!("gap: {}px;", gap));
            }
            if let Some(padding) = layout.padding {
                container_style_parts.push(format!("padding: {}px;", padding));
            }
            let container_style_str = container_style_parts.join(" ");

            rsx! {
                div { style: "{container_style_str}",
                    for element_config in layout.elements {
                        {render_grid_cell(element_config)}
                    }
                }
            }
        }
        unsupported_layout => {
            eprintln!("Warning: Unsupported layout type: {}", unsupported_layout);
            rsx! {
                div {}
            }
        }
    }
}

pub fn render_grid_cell(element_config: LayoutElement) -> Element {
    let cell_style_str = format!(
        "grid-row-start: {}; grid-column-start: {}; grid-row-end: span {}; grid-column-end: span {};",
        element_config.row,
        element_config.col,
        element_config.row_span,
        element_config.col_span
    );

    rsx! {
        div {
            style: "{cell_style_str}",
            key: "{element_config.row}-{element_config.col}",
            {render_atom(element_config.atom)}
        }
    }
}
