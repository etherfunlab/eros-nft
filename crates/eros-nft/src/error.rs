//! Error types for parsing and validation.

use thiserror::Error;

/// Reasons a `PersonaDraft` or `PersonaManifest` may fail validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    /// The input was not valid JSON.
    #[error("invalid JSON: {0}")]
    InvalidJson(String),

    /// The JSON parsed but did not match the type's structural shape.
    #[error("type mismatch at {path}: {message}")]
    TypeMismatch { path: String, message: String },

    /// The JSON matched the type's shape but failed a JSON Schema constraint.
    #[error("schema violation at {path}: {message}")]
    SchemaViolation { path: String, message: String },

    /// A spec-required cross-field invariant did not hold (e.g.
    /// `contains_real_person_likeness=true` requires acknowledgment).
    #[error("invariant violation: {0}")]
    InvariantViolation(String),
}
