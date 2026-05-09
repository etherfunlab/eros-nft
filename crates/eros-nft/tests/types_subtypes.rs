use eros_nft::{AvatarProvenance, AvatarRef, AvatarSource, Creator, DialogueExample};
use serde_json::json;

#[test]
fn creator_round_trips() {
    let v = json!({ "wallet_address": "Es5b8L7L8z3oH4z3yWv2K9mN6oR5sT7uV9wX1yZ2aB3c", "display_name": "Enrique" });
    let c: Creator = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(c.wallet_address.as_deref(), Some("Es5b8L7L8z3oH4z3yWv2K9mN6oR5sT7uV9wX1yZ2aB3c"));
    let back = serde_json::to_value(&c).unwrap();
    assert_eq!(back, v);
}

#[test]
fn dialogue_round_trips() {
    let v = json!({ "user": "Hi", "persona": "Hello back" });
    let d: DialogueExample = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(d.user, "Hi");
    assert_eq!(d.persona, "Hello back");
    assert_eq!(serde_json::to_value(&d).unwrap(), v);
}

#[test]
fn avatar_provenance_serializes_snake_case() {
    let v = serde_json::to_value(AvatarProvenance::SelfCreated).unwrap();
    assert_eq!(v, json!("self_created"));
    let v: AvatarProvenance = serde_json::from_value(json!("ai_generated")).unwrap();
    assert_eq!(v, AvatarProvenance::AiGenerated);
}

#[test]
fn avatar_source_round_trips() {
    let v = json!({
        "uri": "ar://abc",
        "provenance": "self_created",
        "provenance_attestation": "I made this."
    });
    let a: AvatarSource = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(a.uri, "ar://abc");
    assert_eq!(a.provenance, AvatarProvenance::SelfCreated);
    assert_eq!(serde_json::to_value(&a).unwrap(), v);
}

#[test]
fn avatar_ref_round_trips() {
    let v = json!({
        "uri": "ar://xyz",
        "sha256": "0".repeat(64),
        "provenance": "licensed"
    });
    let a: AvatarRef = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(a.sha256.len(), 64);
    assert_eq!(serde_json::to_value(&a).unwrap(), v);
}
