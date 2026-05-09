# eros-nft v1.0 — Overview

## Status

Stable. Backwards-incompatible changes require a new major version (`v2.0`).

## Ecosystem positioning

eros-nft is one node in a two-track ecosystem.

| Form              | Lite (eros-chat)               | Full (eros-app)                  |
|-------------------|--------------------------------|----------------------------------|
| Chat engine       | eros-engine (OSS)              | eros-gateway (closed)            |
| NFT spec          | **eros-nft (this doc)**        | eros-nft-extended (separate)     |
| Marketplace       | eros-chat-marketplace (closed) | eros-app-marketplace (closed)    |

Trained-persona / dossier-transfer / lineage live in the `eros-app` track and
are out of scope here.

## What this spec is

- An open document standard for predefined AI persona NFT cards.
- Two JSON Schemas: `PersonaDraft` (mint-time input) and `PersonaManifest`
  (published artifact).
- The Solana cNFT binding model: how a Solana cNFT references a Manifest. The
  Manifest itself is chain-agnostic.

## What this spec is not

- A trained-persona spec.
- An on-chain inference protocol.
- A marketplace protocol.
- A regulatory framework.

## Versioning

The spec uses RFC-style major/minor: `v1.0`, `v1.1`, `v2.0`. The version is
recorded in `PersonaManifest.spec_version`. Backwards-incompatible changes
require a major bump and a new `spec/v2.0/` directory.

## Conformance

A spec-conformant document MUST satisfy the JSON Schema in
`spec/v1.0/schemas/`. Schema validation does NOT evaluate prompt content for
compliance with the prohibitions in §03; that is implementation-side
moderation.
