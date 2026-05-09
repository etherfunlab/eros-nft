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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceCore {
    pub is_nsfw: bool,
    pub contains_real_person_likeness: bool,
    pub uses_third_party_ip: bool,
    pub real_person_disclaimer_acknowledged: bool,
    pub third_party_ip_disclaimer_acknowledged: bool,
    pub creator_acknowledgments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceRegional {
    pub region: String,
    pub pack_id: String,
    pub pack_version: String,
    pub fields: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Compliance {
    pub core: ComplianceCore,
    pub regional: Vec<ComplianceRegional>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CipherAlg {
    #[serde(rename = "AES-256-GCM")]
    AesGcm256,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptCiphertextRef {
    pub kms_key_ref: String,
    pub ciphertext_uri: String,
    pub ciphertext_sha256: String,
    pub alg: CipherAlg,
    pub aad: String,
}
