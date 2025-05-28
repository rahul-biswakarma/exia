use crate::configs::llm::{clients::LLMClients, invokers::LLMInvokers};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct AppSettings {
    llm_clients: HashMap<LLMInvokers, LLMClients>,
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut clients = HashMap::new();

        for invoker in LLMInvokers::get_all_invokers() {
            clients.insert(invoker.clone(), invoker.default_client());
        }

        Self {
            llm_clients: clients,
        }
    }
}

impl AppSettings {
    pub fn get_client(&self, invoker: &LLMInvokers) -> LLMClients {
        self.llm_clients
            .get(invoker)
            .cloned()
            .unwrap_or_else(|| invoker.default_client())
    }

    pub fn set_client(&mut self, invoker: &LLMInvokers, client: LLMClients) {
        self.llm_clients.insert(invoker.clone(), client);
    }
}

#[derive(Clone, Copy)]
pub struct SettingsContext {
    pub settings: Signal<AppSettings>,
}

impl SettingsContext {
    pub fn update_llm_client(&mut self, invoker: LLMInvokers, client: LLMClients) {
        let mut current = self.settings.peek().clone();
        current.set_client(&invoker, client);
        self.settings.set(current);
    }

    pub fn get_llm_client(&self, invoker: &LLMInvokers) -> LLMClients {
        self.settings.read().get_client(invoker)
    }
}

pub fn use_settings() -> SettingsContext {
    use_context::<SettingsContext>()
}
