use crate::error::RavenError;

/// Runtime specific errors can wrap RavenError.
pub type RuntimeResult<T> = Result<T, RavenError>;
