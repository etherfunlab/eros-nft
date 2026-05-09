//! Embedded JSON Schema 2020-12 documents for `PersonaDraft` and `PersonaManifest`.
//!
//! These are the same files served at `spec/v1.0/schemas/*.json` in the repo.

const DRAFT_SCHEMA: &str = include_str!("../spec/v1.0/schemas/persona-draft.schema.json");
const MANIFEST_SCHEMA: &str = include_str!("../spec/v1.0/schemas/persona-manifest.schema.json");

/// Returns the embedded `PersonaDraft` JSON Schema 2020-12 document.
pub fn json_schema_draft() -> &'static str {
    DRAFT_SCHEMA
}

/// Returns the embedded `PersonaManifest` JSON Schema 2020-12 document.
pub fn json_schema_manifest() -> &'static str {
    MANIFEST_SCHEMA
}
