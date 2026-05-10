use eros_nft::{AffinityPriors, Behavior, TipPersonality};
use serde_json::json;

#[test]
fn tip_personality_serializes_snake_case() {
    assert_eq!(
        serde_json::to_value(TipPersonality::SlowWarm).unwrap(),
        json!("slow_warm")
    );
    assert_eq!(
        serde_json::to_value(TipPersonality::CalmProfessional).unwrap(),
        json!("calm_professional")
    );
    let p: TipPersonality = serde_json::from_value(json!("warm_loud")).unwrap();
    assert_eq!(p, TipPersonality::WarmLoud);
}

#[test]
fn affinity_priors_partial_round_trip() {
    let v = json!({ "warmth": 0.2, "trust": 0.1 });
    let a: AffinityPriors = serde_json::from_value(v).unwrap();
    assert_eq!(a.warmth, Some(0.2));
    assert_eq!(a.intrigue, None);
    let back = serde_json::to_value(a).unwrap();
    assert_eq!(back, json!({ "warmth": 0.2, "trust": 0.1 }));
}

#[test]
fn behavior_required_tip_personality_only() {
    let v = json!({ "tip_personality": "tsundere" });
    let b: Behavior = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(b.tip_personality, TipPersonality::Tsundere);
    assert!(b.affinity_priors.is_none());
    assert_eq!(serde_json::to_value(&b).unwrap(), v);
}
