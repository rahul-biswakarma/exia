use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEventHandler {
    pub action: String,
    pub target: Option<String>,
    pub payload: Option<serde_json::Value>,
    pub condition: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UIState {
    pub components: Signal<HashMap<String, ComponentState>>,
    pub global_state: Signal<serde_json::Value>,
    pub animations: Signal<HashMap<String, AnimationState>>,
    pub form_data: Signal<HashMap<String, serde_json::Value>>,
    pub errors: Signal<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub visible: bool,
    pub content: Option<String>,
    pub properties: serde_json::Value,
    pub local_state: serde_json::Value,
    pub children: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub name: String,
    pub duration: u32,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub component_id: String,
    pub event_type: String,
    pub user_data: Option<serde_json::Value>,
}

pub type ActionFn =
    Rc<dyn Fn(&super::ActionExecutor, &ActionEventHandler, &ActionContext) -> Result<(), String>>;
