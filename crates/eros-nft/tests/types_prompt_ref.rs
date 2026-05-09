use eros_nft::{CipherAlg, PromptCiphertextRef};
use serde_json::json;

#[test]
fn prompt_ciphertext_ref_round_trips() {
    let v = json!({
        "kms_key_ref": "kms://aws/arn:aws:kms:us-west-2:1234:key/abc",
        "ciphertext_uri": "s3://eros-private/ciphertexts/01HXY",
        "ciphertext_sha256": "0".repeat(64),
        "alg": "AES-256-GCM",
        "aad": "ern:1.0:01HXY0000000000000000000000"
    });
    let r: PromptCiphertextRef = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(r.alg, CipherAlg::AesGcm256);
    assert_eq!(serde_json::to_value(&r).unwrap(), v);
}

#[test]
fn cipher_alg_only_aes_gcm_256() {
    let v: CipherAlg = serde_json::from_value(json!("AES-256-GCM")).unwrap();
    assert_eq!(v, CipherAlg::AesGcm256);
    let bad: Result<CipherAlg, _> = serde_json::from_value(json!("AES-128-CTR"));
    assert!(bad.is_err());
}
