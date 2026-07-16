//! Error types for Memory ECC subsystem.
//!
//! Memory ECC uses the standard EccError and EccResult types from the parent ECC module
//! for consistency with the broader error correction framework.

pub use crate::ecc::errors::{EccError as MemoryEccError, EccResult as MemoryEccResult};
