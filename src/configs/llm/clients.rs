#[derive(Debug, Clone, PartialEq)]
pub enum LLMClients {
    Gemini,
}

static ALL_CLIENTS: &[LLMClients] = &[LLMClients::Gemini];

impl LLMClients {
    pub fn label(&self) -> &'static str {
        match self {
            LLMClients::Gemini => "Gemini",
        }
    }
    pub fn get_all_clients() -> &'static [LLMClients] {
        ALL_CLIENTS
    }
}
