# Spec Changelog

## v1.0 errata — 2026-05-12

The v1.0 schema as published 2026-05-10 listed 14 `tip_personality` values,
but the spec's own field reference (`01-persona-draft.md`) committed that
`behavior.tip_personality` must match `eros-engine`'s `TipPersonality`. The
engine actually routes on five: `gold_digger`, `tsundere`, `zen`,
`slow_warm`, `default`. The published enum is corrected in place to match
that promise. Since v1.0 had no published consumers yet (crate `eros-nft`
v0.1.0 was tagged but never pushed to crates.io), this is treated as an
errata to v1.0 rather than a spec version bump. Schema `$id`, directory
layout, and `aad` format are unchanged. Documents written against the
original v1.0 enum using any of the dropped values must be remapped before
re-validation.

## v1.0 — 2026-05-10

Initial release.

- Defines `PersonaDraft` (mint-time input) and `PersonaManifest` (published
  artifact).
- JSON Schemas: `spec/v1.0/schemas/persona-{draft,manifest}.schema.json`.
- Compliance: layered `core` (hard-ban acknowledgments + creator attestations)
  and open `regional[]` slot.
- Solana cNFT binding model: `MetadataArgsV2.uri → PersonaManifest URI` with
  `ManifestRegistry[asset_id]` PDA carrying `manifest_sha256` for integrity.
- Out of scope (this spec): trained-persona dossier, lineage, training
  metrics — see future `eros-nft-extended`.
