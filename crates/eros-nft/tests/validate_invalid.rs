use eros_nft::{PersonaDraft, PersonaManifest};
use serde_json::{json, Value};

fn good_draft() -> Value {
    json!({
        "spec_version": "1.0",
        "creator": {},
        "name": "Test", "tagline": "tag", "description": "desc", "greeting": "hi",
        "definition_dialogues": [],
        "system_prompt": "x",
        "avatar_source": {
            "uri": "data:image/svg+xml;base64,PHN2Zy8+",
            "provenance": "self_created",
            "provenance_attestation": "self"
        },
        "behavior": { "tip_personality": "default" },
        "compliance": {
            "core": {
                "is_nsfw": false,
                "contains_real_person_likeness": false,
                "uses_third_party_ip": false,
                "real_person_disclaimer_acknowledged": false,
                "third_party_ip_disclaimer_acknowledged": false,
                "creator_acknowledgments": ["no_self_harm_encouragement","no_csam","no_minor_sexualization"]
            },
            "regional": []
        }
    })
}

#[test]
fn missing_acknowledgment_is_rejected() {
    let mut v = good_draft();
    v["compliance"]["core"]["creator_acknowledgments"] = json!(["no_self_harm_encouragement", "no_csam"]);
    let d: PersonaDraft = serde_json::from_value(v).unwrap();
    assert!(d.validate().is_err());
}

#[test]
fn real_person_without_disclaimer_is_rejected() {
    let mut v = good_draft();
    v["compliance"]["core"]["contains_real_person_likeness"] = json!(true);
    // disclaimer left as false → fail
    let d: PersonaDraft = serde_json::from_value(v).unwrap();
    assert!(d.validate().is_err());
}

#[test]
fn unknown_tip_personality_is_rejected() {
    let mut v = good_draft();
    v["behavior"]["tip_personality"] = json!("nonexistent");
    let parse_err = serde_json::from_value::<PersonaDraft>(v).err();
    assert!(parse_err.is_some());
}

#[test]
fn manifest_aad_mismatch_is_rejected() {
    let v = json!({
        "spec_version": "1.0",
        "persona_id": "ern:1.0:01HXY000000000000000000000",
        "minted_at": "2026-05-09T12:34:56Z",
        "name": "Test", "tagline": "tag", "description": "desc", "greeting": "hi",
        "avatar": {
            "uri": "ar://abc",
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "provenance": "self_created"
        },
        "prompt_ciphertext_ref": {
            "kms_key_ref": "kms://aws/arn:aws:kms:us-west-2:1234:key/abc",
            "ciphertext_uri": "s3://x/y",
            "ciphertext_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "alg": "AES-256-GCM",
            "aad": "ern:1.0:01HXY111111111111111111111"
        },
        "behavior": { "tip_personality": "default" },
        "compliance": {
            "core": {
                "is_nsfw": false,
                "contains_real_person_likeness": false,
                "uses_third_party_ip": false,
                "real_person_disclaimer_acknowledged": false,
                "third_party_ip_disclaimer_acknowledged": false,
                "creator_acknowledgments": ["no_self_harm_encouragement","no_csam","no_minor_sexualization"]
            },
            "regional": []
        }
    });
    let m: PersonaManifest = serde_json::from_value(v).unwrap();
    let err = m.validate().unwrap_err();
    assert!(format!("{}", err).contains("aad"));
}

#[test]
fn manifest_invalid_persona_id_pattern_is_rejected() {
    let mut v = json!({
        "spec_version": "1.0",
        "persona_id": "not-a-urn",
        "minted_at": "2026-05-09T12:34:56Z",
        "name": "Test", "tagline": "tag", "description": "desc", "greeting": "hi",
        "avatar": {
            "uri": "ar://abc",
            "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "provenance": "self_created"
        },
        "prompt_ciphertext_ref": {
            "kms_key_ref": "kms://aws/arn:aws:kms:us-west-2:1234:key/abc",
            "ciphertext_uri": "s3://x/y",
            "ciphertext_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "alg": "AES-256-GCM",
            "aad": "ern:1.0:01HXY000000000000000000000"
        },
        "behavior": { "tip_personality": "default" },
        "compliance": {
            "core": {
                "is_nsfw": false,
                "contains_real_person_likeness": false,
                "uses_third_party_ip": false,
                "real_person_disclaimer_acknowledged": false,
                "third_party_ip_disclaimer_acknowledged": false,
                "creator_acknowledgments": ["no_self_harm_encouragement","no_csam","no_minor_sexualization"]
            },
            "regional": []
        }
    });
    v["persona_id"] = json!("not-a-urn");
    let m: PersonaManifest = serde_json::from_value(v).unwrap();
    assert!(m.validate().is_err());
}
