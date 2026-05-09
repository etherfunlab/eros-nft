use eros_nft::{Compliance, ComplianceCore, ComplianceRegional};
use serde_json::json;

fn full_core() -> serde_json::Value {
    json!({
        "is_nsfw": false,
        "contains_real_person_likeness": false,
        "uses_third_party_ip": false,
        "real_person_disclaimer_acknowledged": false,
        "third_party_ip_disclaimer_acknowledged": false,
        "creator_acknowledgments": ["no_self_harm_encouragement", "no_csam", "no_minor_sexualization"]
    })
}

#[test]
fn compliance_core_round_trips() {
    let v = full_core();
    let c: ComplianceCore = serde_json::from_value(v.clone()).unwrap();
    assert!(!c.is_nsfw);
    assert_eq!(c.creator_acknowledgments.len(), 3);
    assert_eq!(serde_json::to_value(&c).unwrap(), v);
}

#[test]
fn compliance_regional_carries_arbitrary_fields() {
    let v = json!({
        "region": "JP",
        "pack_id": "xyz.example.adult-jp",
        "pack_version": "1.0",
        "fields": { "age_gate": 18, "nsfw_blocked": false }
    });
    let r: ComplianceRegional = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(r.pack_id, "xyz.example.adult-jp");
    assert_eq!(r.fields["age_gate"], 18);
    assert_eq!(serde_json::to_value(&r).unwrap(), v);
}

#[test]
fn compliance_top_level_round_trips() {
    let v = json!({ "core": full_core(), "regional": [] });
    let c: Compliance = serde_json::from_value(v.clone()).unwrap();
    assert!(c.regional.is_empty());
    assert_eq!(serde_json::to_value(&c).unwrap(), v);
}
