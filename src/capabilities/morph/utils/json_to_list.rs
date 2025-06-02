use serde_json::json;
use std::collections::HashMap;
use std::fs;

pub fn component_json_to_text_list() -> Option<HashMap<String, String>> {
    let component_json_file_content =
        fs::read_to_string("src/capabilities/morph/data/components.json");

    let mut atom_map: HashMap<String, String> = HashMap::new();

    match component_json_file_content {
        Ok(content) => {
            let component_json = serde_json::from_str(&content).unwrap();

            for atom in component_json.iter() {
                if atom_map.contains_key(&atom.name) {
                    let current_value = atom_map.get(&atom.name).unwrap();
                    atom_map.insert(
                        atom.name,
                        format!("{}\n{}", current_value, atom.embeddingText),
                    );
                } else {
                    atom_map.insert(atom.name, atom.embeddingText);
                }
            }

            return Some(atom_map);
        }
        Err(e) => {
            println!("Error reading component.json: {}", e);
            return None;
        }
    }
}
