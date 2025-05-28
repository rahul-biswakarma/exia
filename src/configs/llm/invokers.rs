#[derive(Debug, Clone, PartialEq)]
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
}
