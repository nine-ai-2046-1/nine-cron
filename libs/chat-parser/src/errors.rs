use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("preclean error: {0}")]
    PreClean(String),
    #[error("serde error: {0}")]
    Serde(String),
    #[error("substring extraction failed: {0}")]
    Substring(String),
    #[error("validation error: {0}")]
    Validation(String),
}
