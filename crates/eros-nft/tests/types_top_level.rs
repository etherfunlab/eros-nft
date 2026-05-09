use eros_nft::{PersonaDraft, PersonaManifest};
use serde_json::json;

fn minimal_draft_json() -> serde_json::Value {
    json!({
        "spec_version": "1.0",
        "creator": {},
        "name": "Test",
        "tagline": "tag",
        "description": "desc",
        "greeting": "hi",
        "definition_dialogues": [],
        "system_prompt": "you are a test persona",
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

fn minimal_manifest_json() -> serde_json::Value {
    json!({
        "spec_version": "1.0",
        "persona_id": "ern:1.0:01HXY0000000000000000000000",
        "minted_at": "2026-05-09T12:34:56Z",
        "name": "Test",
        "tagline": "tag",
        "description": "desc",
        "greeting": "hi",
        "avatar": {
            "uri": "ar://abc",
            "sha256": "0".repeat(64),
            "provenance": "self_created"
        },
        "prompt_ciphertext_ref": {
            "kms_key_ref": "kms://aws/arn:aws:kms:us-west-2:1234:key/abc",
            "ciphertext_uri": "s3://x/y",
            "ciphertext_sha256": "0".repeat(64),
            "alg": "AES-256-GCM",
            "aad": "ern:1.0:01HXY0000000000000000000000"
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
fn persona_draft_round_trips() {
    let v = minimal_draft_json();
    let d: PersonaDraft = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(d.spec_version, "1.0");
    assert_eq!(d.name, "Test");
    let back = serde_json::to_value(&d).unwrap();
    assert_eq!(back, v);
}

#[test]
fn persona_manifest_round_trips() {
    let v = minimal_manifest_json();
    let m: PersonaManifest = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(m.persona_id, "ern:1.0:01HXY0000000000000000000000");
    let back = serde_json::to_value(&m).unwrap();
    assert_eq!(back, v);
}
