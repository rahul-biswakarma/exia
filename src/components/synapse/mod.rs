mod event_handler;
mod executor_integration;
mod gemini_api;
mod synapse_component;
mod ui_generator;
mod ui_renderer;
mod vector_search;

pub use event_handler::handle_element_click;
pub use executor_integration::{apply_element_to_executor, apply_ui_schema_to_executor};
pub use synapse_component::Synapse;
pub use ui_generator::generate_ui_with_llm;
pub use ui_renderer::{UIElement, UIRenderer};
