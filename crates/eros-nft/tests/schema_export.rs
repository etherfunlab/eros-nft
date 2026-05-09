use eros_nft::{json_schema_draft, json_schema_manifest};

#[test]
fn schemas_are_non_empty_json() {
    let s = json_schema_draft();
    assert!(s.contains("PersonaDraft"));
    let _: serde_json::Value = serde_json::from_str(s).unwrap();
}

#[test]
fn manifest_schema_references_persona_id_pattern() {
    let s = json_schema_manifest();
    assert!(s.contains("persona_id"));
    assert!(s.contains("ern:1\\\\.0:"));
    let _: serde_json::Value = serde_json::from_str(s).unwrap();
}
