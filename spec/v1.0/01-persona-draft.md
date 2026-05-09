# 01 — PersonaDraft (mint-time input)

`PersonaDraft` is the document a creator submits to a marketplace mint pipeline.
It contains the plaintext system prompt and raw avatar source.

## JSON Schema

See `spec/v1.0/schemas/persona-draft.schema.json`.

## Plaintext data-flow requirements

A spec-conformant pipeline that handles `PersonaDraft` MUST satisfy ALL of the
following:

1. **No durable plaintext persistence.** No log, trace span, panic report,
   crash dump, APM event, queue payload, retry buffer, or moderation-provider
   stored payload may contain the plaintext `system_prompt` or
   `definition_dialogues` content after the encryption step.
2. **Explicit redaction at every boundary.** HTTP request bodies must be
   redacted in access logs. Tracing subscribers must filter `system_prompt` and
   `definition_dialogues` fields. Panic and crash hooks must not serialize
   Draft instances.
3. **Provider retention contracts.** Any external service the pipeline calls
   MUST be configured for zero-retention or its retention terms MUST be
   documented in the pipeline's compliance audit.
4. **Bounded request handling.** Request body size and timeout limits should
   preclude long-lived in-flight buffers. Crash dumps that capture process
   memory must be disabled on hosts that handle Drafts.
5. **No client-side persistence beyond submission.** Browser form state holding
   a Draft MUST be cleared from local/session storage as soon as submission
   completes.

The spec acknowledges that "memory only" is not enforceable in a multi-process
pipeline; the requirements above describe the realizable target.

## Field reference

| Field | Type | Required | Notes |
|---|---|---|---|
| `$schema` | URI | optional | If present, MUST be the canonical Draft schema URL. |
| `spec_version` | string | yes | `"1.0"` for this spec. |
| `creator.wallet_address` | string | optional | Solana base58 pubkey if connected. |
| `creator.display_name` | string | optional | Free-form. |
| `name` | string (1-64 chars) | yes | |
| `tagline` | string (1-200 chars) | yes | One-line description. |
| `description` | string (1-2000 chars) | yes | Long description. |
| `greeting` | string (1-1000 chars) | yes | First persona message in chat. |
| `definition_dialogues` | array | yes | Multi-turn examples. 0-50 items. |
| `definition_dialogues[].user` | string (1-2000 chars) | yes | |
| `definition_dialogues[].persona` | string (1-2000 chars) | yes | |
| `system_prompt` | string (1-32000 chars) | yes | Plaintext; pipeline encrypts before publishing Manifest. |
| `avatar_source.uri` | string | yes | `data:`, `http(s):`, `ar:`, `ipfs:`. |
| `avatar_source.provenance` | enum | yes | `self_created` \| `ai_generated` \| `licensed`. |
| `avatar_source.provenance_attestation` | string (1-1000 chars) | yes | Creator declaration. |
| `behavior.tip_personality` | enum | yes | See [Tip Personality enum](#tip-personality-enum). |
| `behavior.affinity_priors` | object | optional | 6 floats in `[-1.0, 1.0]` for `warmth`, others in `[0.0, 1.0]`. |
| `compliance.core` | object | yes | See [03-compliance.md](03-compliance.md). |
| `compliance.regional` | array | yes | May be empty. |

## Tip Personality enum

`slow_warm`, `tsundere`, `dominant`, `warm_safe`, `tough_love`, `flirty`,
`calm_professional`, `playful_chaotic`, `nostalgic`, `dramatic`, `warm_loud`,
`sensual`, `playful`, `default`.

## Example

See [`samples/persona-yuki-warm-senpai/draft.json`](../../samples/persona-yuki-warm-senpai/draft.json).
