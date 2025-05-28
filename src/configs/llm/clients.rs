#[derive(Debug, Clone, PartialEq)]
pub enum LLMClients {
    Local,
    OpenAI,
    Gemini,
}

static ALL_CLIENTS: &[LLMClients] = &[LLMClients::Local, LLMClients::OpenAI, LLMClients::Gemini];

impl LLMClients {
    pub fn label(&self) -> &'static str {
        match self {
            LLMClients::Local => "Local",
            LLMClients::OpenAI => "OpenAI",
            LLMClients::Gemini => "Gemini",
        }
    }
    pub fn get_all_clients() -> &'static [LLMClients] {
        ALL_CLIENTS
    }
}
