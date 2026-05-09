# 05 — Image / avatar policy

## Spec requires

1. `avatar.uri` MUST be a fetchable address (`http`, `https`, `ar`, `ipfs`).
2. `avatar.sha256` MUST be the SHA-256 of the avatar's raw bytes, lowercase hex (64 chars).
3. `avatar.provenance` MUST be one of `self_created`, `ai_generated`, `licensed`.
4. In `PersonaDraft`, `avatar_source.provenance_attestation` MUST be a non-empty plaintext creator declaration.

## Spec does NOT require

- Image content inspection (NSFW, deepfake, real-person likeness).
- Verification of `licensed` provenance claims.
- Any takedown protocol.

## Spec recommends (non-binding)

> "Implementations accepting `provenance: 'licensed'` Drafts SHOULD require
> creators to upload license documentation out-of-band. This spec performs no
> license verification."
