use crate::components::synapse::renderer::*;
use crate::components::synapse::schema_parser::{Atom, Layout, LayoutElement};
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
