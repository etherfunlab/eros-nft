use eros_nft::ValidationError;

#[test]
fn validation_error_implements_display_and_error() {
    let e = ValidationError::SchemaViolation {
        path: "/name".to_string(),
        message: "must be at most 64 characters".to_string(),
    };
    assert_eq!(
        e.to_string(),
        "schema violation at /name: must be at most 64 characters"
    );
    let _: &dyn std::error::Error = &e;
}

#[test]
fn validation_error_invalid_json_variant() {
    let e = ValidationError::InvalidJson("expected `,` at line 3".to_string());
    assert!(e.to_string().contains("invalid JSON"));
}
