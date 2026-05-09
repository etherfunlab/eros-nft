# 02 — PersonaManifest (published artifact)

`PersonaManifest` is the public document published after mint. It is suitable
for storage on Arweave, IPFS, or as the value of a Solana Metaplex Core asset
metadata URI. **It MUST NOT contain any plaintext prompt, raw dossier, or PII.**

## JSON Schema

See `spec/v1.0/schemas/persona-manifest.schema.json`.

## Manifest is immutable and chain-agnostic

`PersonaManifest` is published once and never republished. It contains no
chain-instance fields, no asset IDs, no owner state. This avoids any circular
dependency on `asset_id` and preserves content-addressed storage semantics.

The cNFT chain instance binding lives outside the Manifest. See
[06-chain-profiles/solana-cnft.md](06-chain-profiles/solana-cnft.md).

## Field reference

| Field | Type | Required | Notes |
|---|---|---|---|
| `$schema` | URI | optional | If present, MUST be the canonical Manifest schema URL. |
| `spec_version` | string | yes | `"1.0"` for this spec. |
| `persona_id` | string | yes | URN form `ern:1.0:<ULID>` (`ern` = Eros Resource Name). |
| `minted_at` | string (RFC3339) | yes | UTC. |
| `name`, `tagline`, `description`, `greeting` | string | yes | Same constraints as Draft. |
| `avatar.uri` | string | yes | `http(s):`, `ar:`, `ipfs:`. |
| `avatar.sha256` | string (64 lowercase hex) | yes | SHA-256 of avatar bytes. |
| `avatar.provenance` | enum | yes | Same as Draft. |
| `prompt_ciphertext_ref.kms_key_ref` | string | yes | `kms://<provider>/...` URI. |
| `prompt_ciphertext_ref.ciphertext_uri` | string | yes | Implementation-defined. |
| `prompt_ciphertext_ref.ciphertext_sha256` | string (64 lowercase hex) | yes | |
| `prompt_ciphertext_ref.alg` | enum | yes | `"AES-256-GCM"` (only value in v1.0). |
| `prompt_ciphertext_ref.aad` | string | yes | MUST equal `persona_id`. |
| `behavior.tip_personality` | enum | yes | |
| `behavior.affinity_priors` | object | optional | |
| `compliance.core` | object | yes | |
| `compliance.regional` | array | yes | May be empty. |

## Document size budget

Target ≤ 4 KB. Keep `description` and dialogue snippets bounded.

## What is intentionally NOT in PersonaManifest

| Field | Why omitted |
|---|---|
| `chain.*` | Pre-mint Manifest cannot reference post-mint asset_id. |
| `last_observed_owner` / owner state | Mutable; would break Manifest immutability. |
| `lineage` (prior owners) | Trained-persona concept. `eros-nft-extended`. |
| `training_metrics` | Trained-persona concept. |
| `style_fingerprint_hash` | Trained-persona concept. |
| `dossier` | Trained-persona concept. |

## Example

See [`samples/persona-yuki-warm-senpai/manifest.json`](../../samples/persona-yuki-warm-senpai/manifest.json).
