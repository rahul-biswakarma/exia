// src/components/synapse/renderer.rs
use dioxus::prelude::*;
use crate::components::synapse::schema_parser::{RootSchema, UiElement, Atom, Layout, LayoutElement}; // Ensure LayoutElement is imported

// Main entry point for rendering UI from a JSON schema string
pub fn render_ui_from_schema(cx: Scope, schema_json: &str) -> Element {
    match serde_json::from_str::<RootSchema>(schema_json) {
        Ok(root_schema) => {
            let rendered_elements = root_schema.ui_elements.into_iter().map(|element| {
                match element {
                    UiElement::Atom(atom) => render_atom(cx, atom),
                    UiElement::Layout(layout) => render_layout(cx, layout),
                }
            }).collect::<Vec<Element>>();

            cx.render(rsx! {
                for element in rendered_elements {
                    element
                }
            })
        }
        Err(e) => {
            eprintln!("Error parsing schema JSON: {:?}", e);
            None
        }
    }
}

// Implementation for render_atom function
fn render_atom(cx: Scope, atom: Atom) -> Element {
    match atom.type_name.as_str() {
        "label" => {
            if let Some(text_value_json) = atom.properties.get("text") {
                if let Some(text_str) = text_value_json.as_str() {
                    cx.render(rsx!( label { "{text_str}" } ))
                } else {
                    eprintln!("Warning: Atom 'label' has 'text' property but it's not a string. Atom: {:?}", atom);
                    None
                }
            } else {
                eprintln!("Warning: Atom 'label' is missing 'text' property. Atom: {:?}", atom);
                None
            }
        }
        "portal" => { 
            cx.render(rsx!( div {} ))
        }
        "separator" => {
            cx.render(rsx!( hr {} ))
        }
        "checkbox" | "switch" => {
            let class_name = if atom.type_name == "switch" { Some("switch") } else { None };

            let checked_attr = atom.properties.get("checked").and_then(|v| v.as_bool());
            let disabled_attr = atom.properties.get("disabled").and_then(|v| v.as_bool());
            let value_attr = atom.properties.get("value").and_then(|v| v.as_str()).map(|s| s.to_string());
            let id_attr = atom.properties.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());

            cx.render(rsx!(input {
                r#type: "checkbox",
                class: class_name,
                checked: checked_attr,
                disabled: disabled_attr,
                value: value_attr.map(|s| format_args!("{}", s)),
                id: id_attr.map(|s| format_args!("{}", s)),
            }))
        }
        "avatar" => {
            let src_attr = atom.properties.get("src").and_then(|v| v.as_str()).map(|s| s.to_string());
            let alt_attr = atom.properties.get("alt").and_then(|v| v.as_str()).map(|s| s.to_string());
            let size_attr = atom.properties.get("size").and_then(|v| v.as_u64()); // u64 for size

            if src_attr.is_none() {
                eprintln!("Warning: Atom 'avatar' is missing 'src' property. Atom: {:?}", atom);
                // Potentially render a placeholder or nothing
            }
            
            cx.render(rsx!(img {
                src: src_attr.map(|s| format_args!("{}", s)),
                alt: alt_attr.map(|s| format_args!("{}", s)),
                width: size_attr.map(|s| format_args!("{}px", s)), // Dioxus might prefer numbers directly for some attributes
                height: size_attr.map(|s| format_args!("{}px", s)), // but px formatting is safer for style-like attributes
            }))
        }
        "progress" => {
            // HTML progress element usually takes value as number, not string.
            // Dioxus should handle Option<f64> or Option<u64> for numeric attributes.
            let value_attr = atom.properties.get("value").and_then(|v| v.as_f64());
            let max_attr = atom.properties.get("max").and_then(|v| v.as_f64());

            cx.render(rsx!(progress {
                value: value_attr,
                max: max_attr,
            }))
        }
        "slider" => {
            let value_attr = atom.properties.get("value").and_then(|v| v.as_f64());
            let min_attr = atom.properties.get("min").and_then(|v| v.as_f64());
            let max_attr = atom.properties.get("max").and_then(|v| v.as_f64());
            let step_attr = atom.properties.get("step").and_then(|v| v.as_f64());
            let disabled_attr = atom.properties.get("disabled").and_then(|v| v.as_bool());
            let id_attr = atom.properties.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());

            cx.render(rsx!(input {
                r#type: "range",
                value: value_attr,
                min: min_attr,
                max: max_attr,
                step: step_attr,
                disabled: disabled_attr,
                id: id_attr.map(|s| format_args!("{}",s)),
            }))
        }
        unknown_type => {
            eprintln!("Warning: Unknown atom type encountered: {}. Atom: {:?}", unknown_type, atom);
            None
        }
    }
}

// Implementation for render_layout function
fn render_layout(cx: Scope, layout: Layout) -> Element {
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

            let cells = layout.elements.into_iter().map(|element_config: LayoutElement| {
                let cell_style_str = format!(
                    "grid-row-start: {}; grid-column-start: {}; grid-row-end: span {}; grid-column-end: span {};",
                    element_config.row,
                    element_config.col,
                    element_config.row_span, 
                    element_config.col_span  
                );
                let atom_element = render_atom(cx, element_config.atom); 
                
                rsx! {
                    div {
                        style: "{cell_style_str}",
                        key: "{element_config.row}-{element_config.col}", 
                        atom_element
                    }
                }
            }).collect::<Vec<Element>>();

            cx.render(rsx! {
                div {
                    style: "{container_style_str}",
                    cells.into_iter() 
                }
            })
        }
        unsupported_layout => {
            eprintln!("Warning: Unsupported layout type: {}", unsupported_layout);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::synapse::schema_parser::LayoutElement; 
    use std::collections::HashMap; // For creating properties in tests

    fn render_to_html_string(element_closure: impl FnOnce(Scope) -> Element + 'static) -> String {
        let mut dom = VirtualDom::new(element_closure);
        dom.rebuild_to_vec(); 
        dioxus::ssr::render_lazy(&mut dom)
    }
    
    // --- render_atom tests (existing) ---
    #[test]
    fn test_render_label_atom_correctly() {
        let atom = Atom {
            type_name: "label".to_string(),
            properties: [("text".to_string(), serde_json::Value::String("Hello Dioxus".to_string()))].into(),
        };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<label>Hello Dioxus</label>");
    }
    #[test]
    fn test_render_label_atom_missing_text() {
        let atom = Atom { type_name: "label".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<!---->");
    }
    #[test]
    fn test_render_label_atom_wrong_text_type() {
        let atom = Atom { type_name: "label".to_string(), properties: [("text".to_string(), serde_json::Value::Number(123.into()))].into() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<!---->");
    }
    #[test]
    fn test_render_portal_atom() {
        let atom = Atom { type_name: "portal".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<div></div>");
    }
    #[test]
    fn test_render_separator_atom() {
        let atom = Atom { type_name: "separator".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<hr/>");
    }
    #[test]
    fn test_render_unknown_atom_type() {
        let atom = Atom { type_name: "unknownAtom".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), "<!---->");
    }

    // --- New render_atom tests ---

    // Checkbox Tests
    #[test]
    fn test_render_checkbox_basic() {
        let atom = Atom { type_name: "checkbox".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="checkbox"/>"#);
    }
    #[test]
    fn test_render_checkbox_checked() {
        let atom = Atom { type_name: "checkbox".to_string(), properties: [("checked".to_string(), serde_json::Value::Bool(true))].into() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="checkbox" checked="true"/>"#);
    }
    #[test]
    fn test_render_checkbox_all_props() {
        let props = [
            ("checked".to_string(), serde_json::Value::Bool(true)),
            ("disabled".to_string(), serde_json::Value::Bool(true)),
            ("value".to_string(), serde_json::Value::String("val1".to_string())),
            ("id".to_string(), serde_json::Value::String("chk1".to_string())),
        ].into_iter().collect();
        let atom = Atom { type_name: "checkbox".to_string(), properties: props };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="checkbox" checked="true" disabled="true" value="val1" id="chk1"/>"#);
    }

    // Switch Tests (renders as checkbox with class "switch")
    #[test]
    fn test_render_switch_basic() {
        let atom = Atom { type_name: "switch".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="checkbox" class="switch"/>"#);
    }
    #[test]
    fn test_render_switch_checked_and_id() {
        let props = [
            ("checked".to_string(), serde_json::Value::Bool(true)),
            ("id".to_string(), serde_json::Value::String("sw1".to_string())),
        ].into_iter().collect();
        let atom = Atom { type_name: "switch".to_string(), properties: props };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="checkbox" class="switch" checked="true" id="sw1"/>"#);
    }
    
    // Avatar Tests
    #[test]
    fn test_render_avatar_basic() { // Missing src will render img tag but src might be missing or empty
        let atom = Atom { type_name: "avatar".to_string(), properties: [("alt".to_string(), serde_json::Value::String("User Avatar".to_string()))].into() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<img alt="User Avatar"/>"#);
    }
    #[test]
    fn test_render_avatar_with_src_alt_size() {
        let props = [
            ("src".to_string(), serde_json::Value::String("image.png".to_string())),
            ("alt".to_string(), serde_json::Value::String("User".to_string())),
            ("size".to_string(), serde_json::json!(32_u64)), // Use u64 for size
        ].into_iter().collect();
        let atom = Atom { type_name: "avatar".to_string(), properties: props };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<img src="image.png" alt="User" width="32px" height="32px"/>"#);
    }

    // Progress Tests
    #[test]
    fn test_render_progress_basic() {
        let atom = Atom { type_name: "progress".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<progress></progress>"#);
    }
    #[test]
    fn test_render_progress_with_value_max() {
        let props = [
            ("value".to_string(), serde_json::json!(0.5_f64)),
            ("max".to_string(), serde_json::json!(1.0_f64)),
        ].into_iter().collect();
        let atom = Atom { type_name: "progress".to_string(), properties: props };
        // Dioxus SSR renders number attributes directly.
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<progress value="0.5" max="1"></progress>"#);
    }
    
    // Slider Tests
    #[test]
    fn test_render_slider_basic() {
        let atom = Atom { type_name: "slider".to_string(), properties: HashMap::new() };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="range"/>"#);
    }
    #[test]
    fn test_render_slider_all_props() {
        let props = [
            ("value".to_string(), serde_json::json!(50_f64)),
            ("min".to_string(), serde_json::json!(0_f64)),
            ("max".to_string(), serde_json::json!(100_f64)),
            ("step".to_string(), serde_json::json!(1_f64)),
            ("disabled".to_string(), serde_json::Value::Bool(false)), // Note: disabled="false" is not rendered by browsers
            ("id".to_string(), serde_json::Value::String("sl1".to_string())),
        ].into_iter().collect();
        let atom = Atom { type_name: "slider".to_string(), properties: props };
        // Dioxus SSR for boolean attributes: if Some(false), it might omit it or render as "false".
        // Standard HTML behavior: presence of `disabled` means true, absence means false.
        // Dioxus typically omits boolean attributes if they are false.
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="range" value="50" min="0" max="100" step="1" id="sl1"/>"#);
    }
     #[test]
    fn test_render_slider_disabled_true() {
        let props = [
            ("disabled".to_string(), serde_json::Value::Bool(true)),
        ].into_iter().collect();
        let atom = Atom { type_name: "slider".to_string(), properties: props };
        assert_eq!(render_to_html_string(|cx| render_atom(cx, atom.clone())), r#"<input type="range" disabled="true"/>"#);
    }


    // --- render_layout tests (existing) ---
    #[test]
    fn test_render_simple_grid_layout() {
        let layout = Layout {
            layout_type: "grid".to_string(), rows: 1, cols: 1, gap: Some(10), padding: Some(5),
            elements: vec![ LayoutElement {
                    atom: Atom { type_name: "label".to_string(), properties: [("text".to_string(), serde_json::Value::String("Cell1".to_string()))].into() },
                    row: 1, col: 1, row_span: 1, col_span: 1,
                }]};
        let expected_html = r#"<div style="display: grid; grid-template-rows: repeat(1, 1fr); grid-template-columns: repeat(1, 1fr); gap: 10px; padding: 5px;"><div style="grid-row-start: 1; grid-column-start: 1; grid-row-end: span 1; grid-column-end: span 1;" key="1-1"><label>Cell1</label></div></div>"#;
        assert_eq!(render_to_html_string(|cx| render_layout(cx, layout.clone())), expected_html);
    }
    #[test]
    fn test_render_grid_layout_no_gap_padding() {
        let layout = Layout {
            layout_type: "grid".to_string(), rows: 1, cols: 1, gap: None, padding: None,
            elements: vec![ LayoutElement {
                    atom: Atom { type_name: "label".to_string(), properties: [("text".to_string(), serde_json::Value::String("Cell1".to_string()))].into() },
                    row: 1, col: 1, row_span: 1, col_span: 1,
                }]};
        let expected_html = r#"<div style="display: grid; grid-template-rows: repeat(1, 1fr); grid-template-columns: repeat(1, 1fr);"><div style="grid-row-start: 1; grid-column-start: 1; grid-row-end: span 1; grid-column-end: span 1;" key="1-1"><label>Cell1</label></div></div>"#;
        assert_eq!(render_to_html_string(|cx| render_layout(cx, layout.clone())), expected_html);
    }
    #[test]
    fn test_render_grid_layout_with_spanning() {
        let layout = Layout {
            layout_type: "grid".to_string(), rows: 2, cols: 2, gap: None, padding: None,
            elements: vec![ LayoutElement {
                    atom: Atom { type_name: "label".to_string(), properties: [("text".to_string(), serde_json::Value::String("Span".to_string()))].into() },
                    row: 1, col: 1, row_span: 2, col_span: 2,
                }]};
        let expected_html = r#"<div style="display: grid; grid-template-rows: repeat(2, 1fr); grid-template-columns: repeat(2, 1fr);"><div style="grid-row-start: 1; grid-column-start: 1; grid-row-end: span 2; grid-column-end: span 2;" key="1-1"><label>Span</label></div></div>"#;
        assert_eq!(render_to_html_string(|cx| render_layout(cx, layout.clone())), expected_html);
    }
    #[test]
    fn test_render_unsupported_layout_type() {
        let layout = Layout { layout_type: "flexbox".to_string(), rows: 1, cols: 1, gap: None, padding: None, elements: vec![] };
        assert_eq!(render_to_html_string(|cx| render_layout(cx, layout.clone())), "<!---->"); 
    }
    #[test]
    fn test_render_grid_with_empty_elements() {
        let layout = Layout { layout_type: "grid".to_string(), rows: 1, cols: 1, gap: None, padding: None, elements: vec![] };
        let expected_html = r#"<div style="display: grid; grid-template-rows: repeat(1, 1fr); grid-template-columns: repeat(1, 1fr);"></div>"#;
        assert_eq!(render_to_html_string(|cx| render_layout(cx, layout.clone())), expected_html);
    }

    // --- render_ui_from_schema tests (existing) ---
    #[test]
    fn test_render_schema_with_grid_layout() {
        let schema_json = r#"
        { "uiElements": [ { "layoutType": "grid", "rows": 1, "cols": 2, "gap": 5, "elements": [
            { "atom": {"type": "label", "properties": {"text": "Left"}}, "row": 1, "col": 1 },
            { "atom": {"type": "label", "properties": {"text": "Right"}}, "row": 1, "col": 2 }
        ]}]} "#;
        let expected_html_part1 = r#"<div style="display: grid; grid-template-rows: repeat(1, 1fr); grid-template-columns: repeat(2, 1fr); gap: 5px;"><div style="grid-row-start: 1; grid-column-start: 1; grid-row-end: span 1; grid-column-end: span 1;" key="1-1"><label>Left</label></div>"#;
        let expected_html_part2 = r#"<div style="grid-row-start: 1; grid-column-start: 2; grid-row-end: span 1; grid-column-end: span 1;" key="1-2"><label>Right</label></div></div>"#;
        let rendered_html = render_to_html_string(|cx| render_ui_from_schema(cx, schema_json));
        assert_eq!(rendered_html, format!("{}{}", expected_html_part1, expected_html_part2));
    }
    #[test]
    fn test_render_empty_schema() {
        let schema_json = r#"{"uiElements": []}"#;
        assert_eq!(render_to_html_string(|cx| render_ui_from_schema(cx, schema_json)), "<!---->");
    }
    #[test]
    fn test_render_invalid_json() {
        let schema_json = r#"{"uiElements": [}"#; 
        assert_eq!(render_to_html_string(|cx| render_ui_from_schema(cx, schema_json)), "<!---->");
    }
    #[test]
    fn test_render_complex_schema_mixed_elements() {
        let schema_json = r#"
        { "uiElements": [ { "type": "separator" },
            { "layoutType": "grid", "rows": 2, "cols": 2, "gap": 5, "padding": 10, "elements": [
                { "atom": {"type": "label", "properties": {"text": "TopLeft"}}, "row": 1, "col": 1 },
                { "atom": {"type": "portal"}, "row": 1, "col": 2 },
                { "atom": {"type": "label", "properties": {"text": "BottomSpan"}}, "row": 2, "col": 1, "colSpan": 2 }
            ]}]} "#;
        let rendered_html = render_to_html_string(|cx| render_ui_from_schema(cx, schema_json));
        let expected_separator_html = "<hr/>";
        let expected_grid_container_style = "display: grid; grid-template-rows: repeat(2, 1fr); grid-template-columns: repeat(2, 1fr); gap: 5px; padding: 10px;";
        let expected_cell1_html = format!(r#"<div style="{}" key="1-1">{}</div>"#, "grid-row-start: 1; grid-column-start: 1; grid-row-end: span 1; grid-column-end: span 1;", "<label>TopLeft</label>");
        let expected_cell2_html = format!(r#"<div style="{}" key="1-2">{}</div>"#, "grid-row-start: 1; grid-column-start: 2; grid-row-end: span 1; grid-column-end: span 1;", "<div></div>");
        let expected_cell3_html = format!(r#"<div style="{}" key="2-1">{}</div>"#, "grid-row-start: 2; grid-column-start: 1; grid-row-end: span 1; grid-column-end: span 2;", "<label>BottomSpan</label>");
        let expected_final_html = format!( "{}{}{}{}{}{}{}{}", expected_separator_html, r#"<div style=""#, expected_grid_container_style, r#"">"#, expected_cell1_html, expected_cell2_html, expected_cell3_html, "</div>");
        assert_eq!(rendered_html, expected_final_html, "Full HTML structure does not match.");
    }
}
```
