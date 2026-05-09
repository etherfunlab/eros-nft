# Spec Changelog

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
