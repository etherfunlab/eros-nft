# eros-nft

Reference implementation of the [eros-nft v1 spec](https://github.com/etherfunlab/eros-nft/tree/main/spec/v1.0).

```rust
use eros_nft::{load_sample, PersonaManifest};

let (_draft, manifest) = load_sample("yuki-warm-senpai").unwrap();
manifest.validate().unwrap();
```

## What this crate provides

- Typed `PersonaDraft` and `PersonaManifest` (`serde`-derived).
- JSON Schema 2020-12 validators backed by the bundled spec schemas.
- Embedded sample loader (15 personas, SFW + NSFW).
- `eros-nft` CLI (`validate`, `schema export`, `sample list/show`).

## What this crate does NOT do

- No mint pipeline.
- No KMS / encryption.
- No Solana RPC, DAS, or Bubblegum mint helpers.
- No HTTP server or marketplace endpoints.

Those concerns belong to `eros-chat-marketplace` (closed source) and are
intentionally outside the open standard.

## License

Apache-2.0 — see `../../LICENSE-CODE`.
