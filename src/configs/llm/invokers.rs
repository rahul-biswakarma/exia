use crate::configs::llm::clients::LLMClients;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LLMInvokers {
    AssignmentGenerator,
    Synapse,
}

static ALL_INVOKERS: &[LLMInvokers] = &[LLMInvokers::AssignmentGenerator, LLMInvokers::Synapse];

impl LLMInvokers {
    pub fn label(&self) -> &'static str {
        match self {
            LLMInvokers::AssignmentGenerator => "Assignment Generator",
            LLMInvokers::Synapse => "Synapse UI Generator",
        }
    }

    pub fn get_all_invokers() -> &'static [LLMInvokers] {
        ALL_INVOKERS
    }

    pub fn default_client(&self) -> LLMClients {
        match self {
            LLMInvokers::AssignmentGenerator => LLMClients::Gemini,
            LLMInvokers::Synapse => LLMClients::Gemini,
        }
    }
}
