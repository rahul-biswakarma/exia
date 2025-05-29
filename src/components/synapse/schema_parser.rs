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
