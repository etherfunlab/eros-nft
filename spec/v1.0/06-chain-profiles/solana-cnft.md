# 06 — Solana cNFT binding model (v1.0)

`PersonaManifest` is chain-agnostic. The mapping from a Solana cNFT instance
to a Manifest lives outside the Manifest, in the cNFT itself plus
marketplace-side state.

## The binding chain

```
Solana cNFT asset (Bubblegum V2 leaf)
    ↓ (MetadataArgsV2.uri at mint)
PersonaManifest URI (e.g. ar://abc123...)
    ↓ (fetch; integrity verified via marketplace registry, see below)
PersonaManifest JSON (immutable, content-addressed by manifest_sha256)
```

1. The cNFT's `MetadataArgsV2.uri` (recorded at mint via `mintV2`) points to
   the published `PersonaManifest`.
2. Manifest integrity is verified out-of-band by reading a marketplace-managed
   on-chain registry PDA `ManifestRegistry[asset_id]` that stores
   `(manifest_uri, manifest_sha256, persona_id, spec_version)`. Bubblegum V2's
   own `data_hash` is computed internally from `MetadataArgsV2` per V2 rules;
   this spec does NOT override that hashing.
3. The mapping `cNFT.asset_id → PersonaManifest` is recovered by either reading
   `metadata_uri` via DAS or reading the registry PDA.
4. The reverse mapping `PersonaManifest.persona_id → cNFT.asset_id` is NOT in
   the Manifest. Marketplaces maintain their own `persona_id → asset_id[]`
   index.

## Required of a v1 Solana implementation

- `tree_address` MUST be a Bubblegum **V2** tree.
- `collection` MUST be a Metaplex **Core** collection. The deprecated Token
  Metadata + Auction House stack is not supported.
- The cNFT `MetadataArgsV2.uri` field MUST point to a fetchable Manifest URI.
- A spec-conformant implementation MUST publish a `ManifestRegistry[asset_id]`
  PDA (or equivalent immutable on-chain account) containing `manifest_sha256`.
  The registry account is initialized at or immediately after mint and has no
  setter ix.

## Outside-the-spec (marketplace-managed)

- `asset_id` of any specific cNFT instance.
- Current owner of any specific cNFT — authoritative source is DAS.
- Access-control decisions MUST query DAS (or verify wallet signature) at
  decision time.

## Future chain profiles

When v1.1+ adds another chain (e.g., TON, Base), a parallel
`06-chain-profiles/<chain>.md` will specify that chain's binding model. The
Manifest format itself does not change.
