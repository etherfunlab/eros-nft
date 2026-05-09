//! Wire types for `PersonaDraft` and `PersonaManifest`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Creator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DialogueExample {
    pub user: String,
    pub persona: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AvatarProvenance {
    SelfCreated,
    AiGenerated,
    Licensed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvatarSource {
    pub uri: String,
    pub provenance: AvatarProvenance,
    pub provenance_attestation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AvatarRef {
    pub uri: String,
    pub sha256: String,
    pub provenance: AvatarProvenance,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TipPersonality {
    SlowWarm,
    Tsundere,
    Dominant,
    WarmSafe,
    ToughLove,
    Flirty,
    CalmProfessional,
    PlayfulChaotic,
    Nostalgic,
    Dramatic,
    WarmLoud,
    Sensual,
    Playful,
    Default,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct AffinityPriors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrigue: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intimacy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patience: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Behavior {
    pub tip_personality: TipPersonality,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_priors: Option<AffinityPriors>,
}
