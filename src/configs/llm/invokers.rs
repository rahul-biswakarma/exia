use crate::configs::llm::clients::LLMClients;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LLMInvokers {
    AssignmentGenerator,
}

static ALL_INVOKERS: &[LLMInvokers] = &[LLMInvokers::AssignmentGenerator];

impl LLMInvokers {
    pub fn label(&self) -> &'static str {
        match self {
            LLMInvokers::AssignmentGenerator => "Assignment Generator",
        }
    }

    pub fn get_all_invokers() -> &'static [LLMInvokers] {
        ALL_INVOKERS
    }

    pub fn default_client(&self) -> LLMClients {
        match self {
            LLMInvokers::AssignmentGenerator => LLMClients::Local,
        }
    }
}
