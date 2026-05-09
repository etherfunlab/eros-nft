# 04 — prompt_ciphertext_ref

The plaintext system prompt is never published. `PersonaManifest` carries
`prompt_ciphertext_ref`, a structured object that separately identifies the
encryption key, the ciphertext object, and the algorithm parameters.

## Field reference

| Field | Required | Purpose |
|---|---|---|
| `kms_key_ref` | yes | Reference to the KMS key needed to decrypt. Opaque KMS-style URI (`kms://<provider>/<provider_specific>`). |
| `ciphertext_uri` | yes | Reference to the ciphertext object. May be a private URI (`s3://`, `r2://`, `https://internal/`, `db://table/row/col`). |
| `ciphertext_sha256` | yes | SHA-256 of the ciphertext bytes, lowercase hex (64 chars). |
| `alg` | yes | Cipher identifier. v1 REQUIRES `"AES-256-GCM"`. |
| `aad` | yes | Additional authenticated data. v1 REQUIRES `aad == persona_id`. |

## kms_key_ref examples

- `kms://aws/arn:aws:kms:us-west-2:1234:key/abc`
- `kms://gcp/projects/x/locations/y/keyRings/z/cryptoKeys/k`
- `kms://supabase-vault/<vault_key_id>`
- `kms://eros-self-hosted/<key_id>`

The spec does not enumerate KMS providers; the prefix after `kms://` is opaque.

## Required guarantees

1. Encryption algorithm MUST be AES-256-GCM.
2. AAD MUST equal `persona_id`. Prevents ciphertext-swap attacks across personas.
3. `ciphertext_sha256` lets verifiers (with access to the ciphertext) confirm
   integrity.
4. Where the actual ciphertext is stored is implementation choice.

## What this spec does NOT cover

- KMS access control (who is allowed to decrypt) — implementation choice.
- Decryption rate limiting, audit logging, retention — implementation choice.
- Prompt-extraction defense runtime layer — implementation choice.

## Why this is "leak reduction, not leak prevention"

A determined buyer with a working chat session can, with effort, infer the
persona's behavior and recreate a similar prompt externally. The encrypted-prompt
model raises the cost of bulk extraction and prevents trivial download; it does
not guarantee scarcity.
