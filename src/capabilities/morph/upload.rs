mod utils;

use utils::json_to_list::component_json_to_text_list;
use utils::list_to_embedding::list_to_embedding;

fn main() {
    let list = component_json_to_text_list();

    match list {
        Some(list) => {
            list_to_embedding(list).await;
        }
        None => {
            println!("No list found");
        }
    }
}
