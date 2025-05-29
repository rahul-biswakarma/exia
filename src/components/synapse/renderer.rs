use crate::components::synapse::schema_parser::{
    Atom, Layout, LayoutElement, RootSchema, UiElement,
};
use dioxus::prelude::*;

// Main entry point for rendering UI from a JSON schema string
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
            rsx! { div { "Error parsing schema" } }
        }
    }
}

// Helper function to render a UiElement
fn render_ui_element(element: UiElement) -> Element {
    match element {
        UiElement::Atom(atom) => render_atom(atom),
        UiElement::Layout(layout) => render_layout(layout),
    }
}

// Implementation for render_atom function
fn render_atom(atom: Atom) -> Element {
    match atom.type_name.as_str() {
        "label" => {
            if let Some(text_value_json) = atom.properties.get("text") {
                if let Some(text_str) = text_value_json.as_str() {
                    rsx! { label { "{text_str}" } }
                } else {
                    eprintln!("Warning: Atom 'label' has 'text' property but it's not a string. Atom: {:?}", atom);
                    rsx! { div {} }
                }
            } else {
                eprintln!(
                    "Warning: Atom 'label' is missing 'text' property. Atom: {:?}",
                    atom
                );
                rsx! { div {} }
            }
        }
        "portal" => {
            rsx! { div {} }
        }
        "separator" => {
            rsx! { hr {} }
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
                    checked: checked,
                    disabled: disabled,
                    value: value,
                    id: id,
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
                    src: src,
                    alt: alt,
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
                    disabled: disabled,
                    id: id,
                }
            }
        }
        unknown_type => {
            eprintln!(
                "Warning: Unknown atom type encountered: {}. Atom: {:?}",
                unknown_type, atom
            );
            rsx! { div {} }
        }
    }
}

// Implementation for render_layout function
fn render_layout(layout: Layout) -> Element {
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
                div {
                    style: "{container_style_str}",
                    for element_config in layout.elements {
                        {render_grid_cell(element_config)}
                    }
                }
            }
        }
        unsupported_layout => {
            eprintln!("Warning: Unsupported layout type: {}", unsupported_layout);
            rsx! { div {} }
        }
    }
}

// Helper function to render a grid cell
fn render_grid_cell(element_config: LayoutElement) -> Element {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::synapse::schema_parser::LayoutElement;
    use std::collections::HashMap;

    #[test]
    fn test_render_label_atom_correctly() {
        let atom = Atom {
            type_name: "label".to_string(),
            properties: [(
                "text".to_string(),
                serde_json::Value::String("Hello Dioxus".to_string()),
            )]
            .into(),
        };
        let element = render_atom(atom);
        // Basic test to ensure it doesn't panic
        assert!(format!("{:?}", element).contains("label"));
    }

    #[test]
    fn test_render_portal_atom() {
        let atom = Atom {
            type_name: "portal".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("div"));
    }

    #[test]
    fn test_render_separator_atom() {
        let atom = Atom {
            type_name: "separator".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("hr"));
    }

    #[test]
    fn test_render_unknown_atom_type() {
        let atom = Atom {
            type_name: "unknownAtom".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("div"));
    }

    #[test]
    fn test_render_checkbox_basic() {
        let atom = Atom {
            type_name: "checkbox".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("input"));
    }

    #[test]
    fn test_render_switch_basic() {
        let atom = Atom {
            type_name: "switch".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("input"));
    }

    #[test]
    fn test_render_avatar_basic() {
        let atom = Atom {
            type_name: "avatar".to_string(),
            properties: [(
                "alt".to_string(),
                serde_json::Value::String("User Avatar".to_string()),
            )]
            .into(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("img"));
    }

    #[test]
    fn test_render_progress_basic() {
        let atom = Atom {
            type_name: "progress".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("progress"));
    }

    #[test]
    fn test_render_slider_basic() {
        let atom = Atom {
            type_name: "slider".to_string(),
            properties: HashMap::new(),
        };
        let element = render_atom(atom);
        assert!(format!("{:?}", element).contains("input"));
    }

    #[test]
    fn test_render_simple_grid_layout() {
        let layout = Layout {
            layout_type: "grid".to_string(),
            rows: 1,
            cols: 1,
            gap: Some(10),
            padding: Some(5),
            elements: vec![LayoutElement {
                atom: Atom {
                    type_name: "label".to_string(),
                    properties: [(
                        "text".to_string(),
                        serde_json::Value::String("Cell1".to_string()),
                    )]
                    .into(),
                },
                row: 1,
                col: 1,
                row_span: 1,
                col_span: 1,
            }],
        };
        let element = render_layout(layout);
        assert!(format!("{:?}", element).contains("div"));
    }

    #[test]
    fn test_render_unsupported_layout_type() {
        let layout = Layout {
            layout_type: "flexbox".to_string(),
            rows: 1,
            cols: 1,
            gap: None,
            padding: None,
            elements: vec![],
        };
        let element = render_layout(layout);
        assert!(format!("{:?}", element).contains("div"));
    }

    #[test]
    fn test_render_ui_from_schema_basic() {
        let schema_json = r#"
        {
            "uiElements": [
                {
                    "type": "label",
                    "properties": {
                        "text": "Hello"
                    }
                }
            ]
        }
        "#;
        let element = render_ui_from_schema(schema_json);
        assert!(format!("{:?}", element).len() > 0);
    }

    #[test]
    fn test_render_invalid_json() {
        let schema_json = r#"{"uiElements": [}"#;
        let element = render_ui_from_schema(schema_json);
        assert!(format!("{:?}", element).contains("Error"));
    }
}
