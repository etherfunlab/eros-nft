# eros-nft

Open standard for predefined AI persona NFT cards on Solana cNFT.

A persona card is a self-contained Solana cNFT that grants the holder permission
to chat with a specific AI persona. This repo defines the format (`PersonaDraft`,
`PersonaManifest`), publishes JSON Schema 2020-12 contracts, ships a Rust
reference crate (`eros-nft` on crates.io), and bundles 15 sample personas.

- **Spec:** `spec/v1.0/` (CC-BY-4.0)
- **Reference crate:** `crates/eros-nft/` (Apache-2.0, on crates.io)
- **Samples:** `samples/` (15 personas, SFW + NSFW)

The trained-persona / dossier-transfer / lineage scenarios live in a separate
`eros-nft-extended` spec, not in this repo.

## Status

v1.0 of the spec is the current baseline. The reference crate publishes from
`crates/eros-nft/`. See `CHANGELOG.md` and `spec/CHANGELOG.md`.

## License

- Spec documents: CC-BY-4.0 (see `LICENSE-SPEC`)
- Code and JSON Schemas: Apache-2.0 (see `LICENSE-CODE`)
