//! Memory ECC subsystem for validation, correction, classification, and verification
//! of memory entries before storage, update, consolidation, and retrieval.
//!
//! This module implements a deterministic error correction layer for the memory system,
//! following the same architectural patterns as the Planner ECC.

pub mod classifier;
pub mod context;
pub mod corrector;
pub mod engine;
pub mod errors;
pub mod pipeline;
pub mod policy;
pub mod report;
pub mod rules;
pub mod scorer;
pub mod tests;
pub mod types;
pub mod validator;

pub use classifier::MemoryErrorClassifier;
pub use context::MemoryEccContext;
pub use corrector::MemoryCorrector;
pub use engine::MemoryEccEngine;
pub use errors::{MemoryEccError, MemoryEccResult};
pub use pipeline::MemoryEccPipeline;
pub use policy::MemoryPolicy;
pub use report::MemoryEccReport;
pub use scorer::MemoryConfidenceScorer;
pub use types::MemoryValidationEntry;
pub use validator::MemoryValidator;
