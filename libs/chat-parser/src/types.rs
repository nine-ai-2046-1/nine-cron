use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NormalizedParams {
    pub time: Option<String>,
    pub date: Option<String>,
    pub recurrence: Option<String>,
    pub title: Option<String>,
    pub cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NormalizedChatResponse {
    pub action: Option<String>,
    pub needs_clarification: bool,
    pub clarification_question: Option<String>,
    pub title: Option<String>,
    pub params: Option<NormalizedParams>,
}
