use eros_nft::{PersonaDraft, PersonaManifest};
use serde_json::json;

fn minimal_draft() -> PersonaDraft {
    serde_json::from_value(json!({
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
    })).unwrap()
}

fn minimal_manifest() -> PersonaManifest {
    serde_json::from_value(json!({
        "spec_version": "1.0",
        "persona_id": "ern:1.0:01HXY000000000000000000000",
        "minted_at": "2026-05-09T12:34:56Z",
        "name": "Test",
        "tagline": "tag",
        "description": "desc",
        "greeting": "hi",
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
    })).unwrap()
}

#[test]
fn minimal_draft_validates() {
    minimal_draft().validate().expect("valid Draft must pass");
}

#[test]
fn minimal_manifest_validates() {
    minimal_manifest()
        .validate()
        .expect("valid Manifest must pass");
}
