// src/components/synapse/schema_parser.rs
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RootSchema {
    pub ui_elements: Vec<UiElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
// UiElement is expected to be an untagged enum based on schema structure.
// The JSON objects for Atom and Layout are structurally distinct.
// Atom has "type" and "properties".
// Layout has "layoutType", "rows", "cols", "elements".
// Serde will try to deserialize into Atom first, and if it fails (e.g., missing "type" field
// or finding "layoutType" instead), it will try Layout.
#[serde(untagged)]
pub enum UiElement {
    Atom(Atom),
    Layout(Layout),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Atom {
    #[serde(rename = "type")] // Map the schema's "type" to "type_name" in Rust
    pub type_name: String,
    #[serde(default)] // If properties is missing, default to an empty HashMap
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    pub layout_type: String, // e.g., "grid"
    pub rows: u32,
    pub cols: u32,
    pub gap: Option<u32>,
    pub padding: Option<u32>,
    pub elements: Vec<LayoutElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LayoutElement {
    pub atom: Atom,
    pub row: u32,
    pub col: u32,
    #[serde(default = "default_span")]
    pub row_span: u32,
    #[serde(default = "default_span")]
    pub col_span: u32,
}

fn default_span() -> u32 {
    1
}

// Basic test to ensure deserialization works for a simple case (optional, but good practice)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simple_atom() {
        let json_data = r#"
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
        let parsed: Result<RootSchema, _> = serde_json::from_str(json_data);
        assert!(
            parsed.is_ok(),
            "Failed to parse simple atom: {:?}",
            parsed.err()
        );
        let root_schema = parsed.unwrap();
        assert_eq!(root_schema.ui_elements.len(), 1);
        // Check if the first element is an Atom and matches expectations
        match &root_schema.ui_elements[0] {
            UiElement::Atom(atom) => {
                assert_eq!(atom.type_name, "label");
                assert_eq!(
                    atom.properties.get("text").unwrap().as_str().unwrap(),
                    "Hello"
                );
            }
            _ => panic!("Expected an Atom"),
        }
    }

    #[test]
    fn test_deserialize_simple_layout() {
        let json_data = r#"
        {
            "uiElements": [
                {
                    "layoutType": "grid",
                    "rows": 1,
                    "cols": 1,
                    "elements": [
                        {
                            "atom": {
                                "type": "label",
                                "properties": {"text": "In Grid"}
                            },
                            "row": 1,
                            "col": 1
                        }
                    ]
                }
            ]
        }
        "#;
        let parsed: Result<RootSchema, _> = serde_json::from_str(json_data);
        assert!(
            parsed.is_ok(),
            "Failed to parse simple layout: {:?}",
            parsed.err()
        );
        let root_schema = parsed.unwrap();
        assert_eq!(root_schema.ui_elements.len(), 1);
        // Check if the first element is a Layout and matches expectations
        match &root_schema.ui_elements[0] {
            UiElement::Layout(layout) => {
                assert_eq!(layout.layout_type, "grid");
                assert_eq!(layout.elements.len(), 1);
                assert_eq!(layout.elements[0].atom.type_name, "label");
                assert_eq!(layout.elements[0].row_span, 1); // Check default span
                assert_eq!(layout.elements[0].col_span, 1); // Check default span
            }
            _ => panic!("Expected a Layout"),
        }
    }

    #[test]
    fn test_deserialize_untagged_choice() {
        // Test if UiElement can correctly deserialize both Atom and Layout
        // when they appear in a list.
        let json_data = r#"
        {
            "uiElements": [
                {
                    "type": "label",
                    "properties": { "text": "First Atom" }
                },
                {
                    "layoutType": "grid",
                    "rows": 1,
                    "cols": 1,
                    "elements": [
                        {
                            "atom": { "type": "label", "properties": {"text": "Atom in Grid"} },
                            "row": 1,
                            "col": 1,
                            "rowSpan": 2
                        }
                    ]
                }
            ]
        }
        "#;
        let parsed: Result<RootSchema, _> = serde_json::from_str(json_data);
        assert!(
            parsed.is_ok(),
            "Failed to parse untagged choice: {:?}",
            parsed.err()
        );
        let root_schema = parsed.unwrap();
        assert_eq!(root_schema.ui_elements.len(), 2);

        match &root_schema.ui_elements[0] {
            UiElement::Atom(atom) => assert_eq!(atom.type_name, "label"),
            _ => panic!("Expected first element to be an Atom"),
        }
        match &root_schema.ui_elements[1] {
            UiElement::Layout(layout) => {
                assert_eq!(layout.layout_type, "grid");
                assert_eq!(layout.elements[0].row_span, 2); // Explicitly set
                assert_eq!(layout.elements[0].col_span, 1); // Default
            }
            _ => panic!("Expected second element to be a Layout"),
        }
    }

    #[test]
    fn test_atom_missing_properties() {
        let json_data = r#"
        {
            "uiElements": [
                {
                    "type": "avatar"
                }
            ]
        }
        "#;
        let parsed: Result<RootSchema, _> = serde_json::from_str(json_data);
        assert!(
            parsed.is_ok(),
            "Failed to parse atom with missing properties: {:?}",
            parsed.err()
        );
        let root_schema = parsed.unwrap();
        assert_eq!(root_schema.ui_elements.len(), 1);
        match &root_schema.ui_elements[0] {
            UiElement::Atom(atom) => {
                assert_eq!(atom.type_name, "avatar");
                assert!(
                    atom.properties.is_empty(),
                    "Properties should be an empty HashMap"
                );
            }
            _ => panic!("Expected an Atom"),
        }
    }

    #[test]
    fn test_layout_missing_optional_fields() {
        let json_data = r#"
        {
            "uiElements": [
                {
                    "layoutType": "grid",
                    "rows": 2,
                    "cols": 2,
                    "elements": []
                }
            ]
        }
        "#;
        let parsed: Result<RootSchema, _> = serde_json::from_str(json_data);
        assert!(
            parsed.is_ok(),
            "Failed to parse layout with missing optional fields: {:?}",
            parsed.err()
        );
        let root_schema = parsed.unwrap();
        assert_eq!(root_schema.ui_elements.len(), 1);
        match &root_schema.ui_elements[0] {
            UiElement::Layout(layout) => {
                assert_eq!(layout.layout_type, "grid");
                assert_eq!(layout.rows, 2);
                assert_eq!(layout.cols, 2);
                assert_eq!(layout.gap, None);
                assert_eq!(layout.padding, None);
                assert!(layout.elements.is_empty());
            }
            _ => panic!("Expected a Layout"),
        }
    }
}
