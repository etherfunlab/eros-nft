# eros-nft

> Open standard for predefined AI persona NFT cards on Solana cNFT.

A persona card is a self-contained Solana cNFT that grants the holder permission
to chat with a specific predefined AI persona. This repo defines the format,
publishes JSON Schema 2020-12 contracts, ships a Rust reference crate
(`eros-nft` on crates.io), and bundles 15 sample personas.

[![CI](https://github.com/etherfunlab/eros-nft/actions/workflows/ci.yml/badge.svg)](https://github.com/etherfunlab/eros-nft/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/eros-nft.svg)](https://crates.io/crates/eros-nft)
[![Docs.rs](https://docs.rs/eros-nft/badge.svg)](https://docs.rs/eros-nft)
[![Spec License: CC-BY-4.0](https://img.shields.io/badge/Spec%20License-CC--BY--4.0-blue.svg)](LICENSE-SPEC)
[![Code License: Apache-2.0](https://img.shields.io/badge/Code%20License-Apache--2.0-green.svg)](LICENSE-CODE)

English · [中文](README.zh.md)

## What's in this repo

| Path | Contents |
|---|---|
| `spec/v1.0/` | Normative spec documents (CC-BY-4.0). |
| `spec/v1.0/schemas/` | `persona-draft.schema.json`, `persona-manifest.schema.json` (JSON Schema 2020-12, Apache-2.0). |
| `crates/eros-nft/` | Rust reference crate (Apache-2.0). Types, validators, sample loader, CLI. |
| `samples/` | 15 demo personas (5 NSFW). Each has `draft.json` + `manifest.json` + `README.md`. |

## Quick start

### Validate a document via CLI

```bash
cargo install eros-nft
eros-nft validate ./my-persona-manifest.json
```

### Use the crate

```toml
[dependencies]
eros-nft = "0.1"
```

```rust
use eros_nft::{load_sample, PersonaManifest};

fn main() {
    let (_draft, manifest) = load_sample("yuki-warm-senpai").unwrap();
    manifest.validate().unwrap();
    println!("{} ({})", manifest.name, manifest.persona_id);
}
```

### List bundled samples

```bash
eros-nft sample list
eros-nft sample show yuki-warm-senpai
```

## Concepts

A persona NFT card has two document forms:

- **`PersonaDraft`** — what a creator submits to a marketplace mint pipeline.
  Contains the plaintext system prompt and raw avatar source. Lives only in
  the pipeline; never published.
- **`PersonaManifest`** — the published artifact. Suitable for Arweave / IPFS
  pinning and as the metadata URI of a Solana cNFT. Carries `prompt_ciphertext_ref`
  (KMS reference + ciphertext SHA-256), not the plaintext.

The cNFT itself is the chain anchor; the Manifest is chain-agnostic. See
[`spec/v1.0/06-chain-profiles/solana-cnft.md`](spec/v1.0/06-chain-profiles/solana-cnft.md).

## Out of scope

- Trained-persona transfer (memory dossier, lineage, training metrics) lives in
  a separate future spec, `eros-nft-extended`.
- Marketplace business logic (mint pipeline, royalty enforcement, takedown)
  lives in `eros-chat-marketplace` (closed source).

## License

- Spec documents: **CC-BY-4.0** (see [LICENSE-SPEC](LICENSE-SPEC))
- Code and JSON Schemas: **Apache-2.0** (see [LICENSE-CODE](LICENSE-CODE))
