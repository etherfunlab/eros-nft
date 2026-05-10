//! JSON Schema 2020-12 validators backed by the embedded spec schemas.

use std::sync::OnceLock;

use jsonschema::Validator;
use serde_json::Value;

use crate::error::ValidationError;
use crate::schema::{json_schema_draft, json_schema_manifest};
use crate::types::{PersonaDraft, PersonaManifest};

fn draft_validator() -> &'static Validator {
    static V: OnceLock<Validator> = OnceLock::new();
    V.get_or_init(|| {
        let schema: Value =
            serde_json::from_str(json_schema_draft()).expect("embedded Draft schema is valid JSON");
        // The Manifest schema $refs into the Draft schema; for Draft alone we don't
        // need the Manifest doc, but we register it as a peer for symmetry.
        Validator::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .build(&schema)
            .expect("embedded Draft schema is valid JSON Schema 2020-12")
    })
}

fn manifest_validator() -> &'static Validator {
    static V: OnceLock<Validator> = OnceLock::new();
    V.get_or_init(|| {
        let manifest_schema: Value = serde_json::from_str(json_schema_manifest())
            .expect("embedded Manifest schema is valid JSON");
        let draft_schema: Value =
            serde_json::from_str(json_schema_draft()).expect("embedded Draft schema is valid JSON");
        Validator::options()
            .with_draft(jsonschema::Draft::Draft202012)
            .with_resource(
                "https://eros.nft/spec/v1.0/persona-draft.schema.json",
                jsonschema::Resource::from_contents(draft_schema)
                    .expect("Draft schema is a valid Resource"),
            )
            .build(&manifest_schema)
            .expect("embedded Manifest schema is valid JSON Schema 2020-12")
    })
}

fn validate_with(validator: &Validator, json: &Value) -> Result<(), ValidationError> {
    if let Err(err) = validator.validate(json) {
        return Err(ValidationError::SchemaViolation {
            path: err.instance_path.to_string(),
            message: err.to_string(),
        });
    }
    Ok(())
}

impl PersonaDraft {
    /// Validate this `PersonaDraft` against the v1.0 JSON Schema and the
    /// spec's cross-field invariants.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let json =
            serde_json::to_value(self).map_err(|e| ValidationError::InvalidJson(e.to_string()))?;
        validate_with(draft_validator(), &json)
    }
}

impl PersonaManifest {
    /// Validate this `PersonaManifest` against the v1.0 JSON Schema and the
    /// spec's cross-field invariants.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let json =
            serde_json::to_value(self).map_err(|e| ValidationError::InvalidJson(e.to_string()))?;
        validate_with(manifest_validator(), &json)?;
        // Cross-field invariant: aad must equal persona_id.
        if self.prompt_ciphertext_ref.aad != self.persona_id {
            return Err(ValidationError::InvariantViolation(format!(
                "prompt_ciphertext_ref.aad ({}) must equal persona_id ({})",
                self.prompt_ciphertext_ref.aad, self.persona_id
            )));
        }
        Ok(())
    }
}
