use crate::knowledge::errors::KnowledgeError;

/// Generator specific error wrapper.
pub type GeneratorResult<T> = Result<T, GeneratorError>;

#[derive(thiserror::Error, Debug)]
pub enum GeneratorError {
    #[error("io error: {0}")]
    Io(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("pipeline error: {0}")]
    Pipeline(String),

    #[error("internal: {0}")]
    Internal(String),
}

impl From<KnowledgeError> for GeneratorError {
    fn from(err: KnowledgeError) -> Self {
        GeneratorError::Pipeline(err.to_string())
    }
}

impl From<std::io::Error> for GeneratorError {
    fn from(err: std::io::Error) -> Self {
        GeneratorError::Io(err.to_string())
    }
}
