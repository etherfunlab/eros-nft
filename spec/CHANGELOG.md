# Spec Changelog

## Unreleased

- **Breaking:** `Behavior.tip_personality` enum reduced from 14 values to the
  five that `eros-engine` actually routes on: `gold_digger`, `tsundere`,
  `zen`, `slow_warm`, `default`. Documents written against the v1.0 enum that
  use any other value will fail validation; the recommended migration is to
  remap to the closest supported value (most expressive personas fall back to
  `default`).

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
