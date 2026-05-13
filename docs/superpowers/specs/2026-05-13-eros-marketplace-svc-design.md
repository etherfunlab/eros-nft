# `eros-marketplace-svc` — Design

**Date:** 2026-05-13
**Status:** Draft (pending user review)
**Scope:** v0.1 of the off-chain marketplace backend service that connects creators, sellers, buyers, the on-chain `eros-marketplace-solana` program, and the `eros-engine` chat service.

## 1. Ecosystem position

`eros-marketplace-svc` is the middle tier the ecosystem is missing. The other four pieces already exist:

| Repo | Role | License |
|---|---|---|
| `eros-nft` | Open persona NFT spec, JSON Schemas, Rust crate (`eros-nft` on crates.io) | Apache-2.0 / CC-BY-4.0 |
| `eros-marketplace-solana` | Anchor program for atomic on-chain settlement (5 ix, 3 PDAs) | Apache-2.0 |
| `eros-engine` | Open chat engine (Axum + Postgres + pgvector), the `/comp/*` API | AGPL-3.0 |
| `eros-engine-web` | Consumer frontend (Nuxt). Has `/marketplace/*` placeholder pages | closed |

`eros-marketplace-svc` orchestrates: persona mint → manifest pinning → cNFT mint → listing signature → on-chain settlement → ownership reconciliation → engine access gate. It does not chat, does not sign on behalf of users, and does not submit the final buy transaction.

```
┌─────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│  eros-engine-web    │ →  │ eros-marketplace-svc│ →  │ eros-marketplace-   │
│  /marketplace/*     │ ←  │   (this design)     │ ←  │  solana             │
└─────────────────────┘    │                     │    └─────────────────────┘
                           │ • mint pipeline     │
┌─────────────────────┐    │ • KMS + manifest    │    ┌─────────────────────┐
│  eros-engine        │ ←  │ • SaleOrder sign    │ ←  │ Helius webhooks +   │
│  /comp/personas     │    │ • catalog/listings  │    │ DAS reconciler      │
└─────────────────────┘    │ • indexer/webhook   │    └─────────────────────┘
                           └─────────────────────┘
                                   │
                                   ↓
                          ┌─────────────────────┐
                          │ Postgres + KMS +    │
                          │ Arweave / IPFS pin  │
                          └─────────────────────┘
```

## 2. Goals

1. Provide the missing service layer so `eros-engine-web`'s `useMarketplace()` composable can swap `placeholderListings` for a real API.
2. Operate the persona mint pipeline end-to-end: `PersonaDraft` → encrypted prompt → published `PersonaManifest` → cNFT mint on Solana, with idempotent state.
3. Coordinate the on-chain settlement flow: derive canonical `SaleOrder` bytes, verify seller's ed25519 signature, mirror `set_listing_quote` / `cancel_listing` on-chain, never custody buyer or seller keys.
4. Reconcile chain → DB via Helius webhook (primary) and DAS pull (fallback), so the engine learns that a buyer now owns a persona and can chat with it.
5. Stay open source (Apache-2.0) so closed-source product features can depend on it as a library, not fork it.

## 3. Non-goals (v0.1)

- Auction, Dutch auction, time-decay pricing. **Fixed-price listings only.**
- Multi-edition / multiple copies of one persona. **One persona = one cNFT.**
- Multi-chain. Solana only.
- The `eros-nft-extended` (trained-persona memory dossier) flow.
- Mobile push, email notifications.
- Custodial wallets. Sellers off-chain-sign the SaleOrder canonical bytes in their own wallet; they never give svc a key.
- Buy-transaction submission. The frontend builds the 2-instruction tx (Ed25519Program precompile + `execute_purchase`) and submits to RPC directly.
- Royalty splits beyond `[creator, platform]`. The on-chain `RoyaltyRegistry` is two-recipient and this service mirrors that.

## 4. Architecture — three subsystems, one binary

### 4.1 Mint pipeline (creator → minted cNFT)

State machine, persisted in Postgres, idempotent at every step:

```
draft_submitted
    → schema_validated         (eros-nft crate PersonaDraft::validate)
    → moderated                (image + prompt content; pluggable, stub by default)
    → prompt_encrypted         (AES-256-GCM, aad=persona_id, KMS-wrapped DEK)
    → ciphertext_stored        (S3-compatible object; sha256 recorded)
    → manifest_assembled       (eros-nft crate PersonaManifest, prompt_ciphertext_ref populated)
    → manifest_pinned          (Irys / Arweave / web3.storage; URI recorded)
    → collection_resolved      (use existing or create new Core collection)
    → cnft_minted              (Bubblegum V2 mint_v2 against the collection;
                                asset_id captured from mint log; DAS lookup
                                by (tree, leaf_index, tx_signature) as fallback)
    → registries_initialized   (svc-as-admin calls init_registries(asset_id,
                                royalty_recipient, royalty_bps,
                                platform_fee_recipient, platform_fee_bps,
                                manifest_uri, manifest_sha256, persona_id,
                                spec_version) — REQUIRED before any listing
                                is purchasable; without it execute_purchase
                                fails because RoyaltyRegistry doesn't exist)
    → indexed                  (personas + ownership rows populated)
```

Each transition writes to `mint_jobs.state` along with the step's output (tx_signature, URI, sha256, or asset_id as applicable). Retries reconcile against recorded output rather than recomputing — pinning, minting, and `init_registries` each produce non-deterministic side effects that cannot be re-derived from inputs alone.

Collection-creation sub-flow (runs once per collection, not per mint):

1. mpl-core `create_v2` with `PermanentTransferDelegate.authority = derive_sale_authority(collection)` — the PDA is `[SALE_AUTHORITY_SEED, collection_pubkey]` per `eros-marketplace-solana` v0.2.
2. Call `register_collection(collection)` on the program (admin-gated; svc holds the admin keypair via KMS).
3. Bubblegum V2 `create_tree_config_v2` for the cMT that will hold this collection's cNFTs.

### 4.2 Listings + SaleOrder orchestration (seller → on-chain listing)

The canonical `SaleOrder` layout MUST match `eros-marketplace-solana::sale_order::SaleOrder` byte-for-byte. v0.2 of that program defines it as 120 bytes: `asset_id(32) + collection(32) + seller_wallet(32) + price_lamports(8) + listing_nonce(8) + expires_at(8)`. The svc consumes the program's `SaleOrder` struct directly via `cargo add eros-marketplace-solana` — no manual mirror.

**Who submits what:** the on-chain `set_listing_quote` is **admin-gated** (`has_one = admin`; see `set_listing_quote.rs`). That is a deliberate program-side constraint: it prevents an attacker from poisoning a seller's `last_seen_nonce` watermark or front-running an unrelated SaleOrder signature into an active state. The svc, as the holder of the admin key, is therefore the submitter for both `set_listing_quote` and `init_registries`. The seller's wallet only signs the SaleOrder canonical bytes off-chain (no gas, no on-chain tx for listing). For cancellation, `cancel_listing` requires the seller as `Signer` (`cancel_listing.rs:10`), so the seller submits it themselves; svc only prepares the ix payload and watches the webhook.

**Nonce model:** `ListingState` PDA is keyed by `(asset_id, seller_wallet)`. Inside that PDA, `last_seen_nonce` is **strict-monotonic forever** (`require!(listing_nonce > s.last_seen_nonce, NonceNotMonotonic)`). svc therefore maintains a `listing_nonce_watermarks(asset_id, seller_wallet, last_issued_nonce)` table — one row per `(asset, seller)` pair. Nonces are never reclaimed and never reused. `/listings/quote` returns `last_issued_nonce + 1`.

Listing flow:

```
client → POST /listings/quote { asset_id, price_lamports, expires_at }
       ← svc allocates next nonce for (asset_id, caller_wallet) from the
         per-pair watermark; returns { canonical_bytes (hex), listing_nonce,
         sale_order_fields }
                     [no ix payload returned — svc itself will submit on-chain]

client → wallet signs ed25519 over canonical_bytes (off-chain, free)
       → POST /listings { sale_order_fields, seller_signature }
       ← svc verifies:
           (a) ed25519 sig is valid for sale_order_fields.seller_wallet
           (b) seller_wallet is currently bound to the caller's identity
               (see §4.4)
           (c) seller_wallet currently owns asset_id (DAS check)
           (d) listing_nonce matches the unconsumed watermark
       → svc submits set_listing_quote(asset_id, seller_wallet, listing_nonce)
         signed by the admin key (KMS-wrapped); svc-paid rent
       ← on tx confirm via webhook, listing.state = 'active'; visible in catalog
```

Note: in v0.2 there is no per-listing Bubblegum leaf delegate. Transfers go through the collection's `PermanentTransferDelegate` plugin, whose authority is the `sale_authority` PDA derived from `[SALE_AUTHORITY_SEED, collection]`. The seller does NOT do a per-listing delegate ix; collection-level setup at mint time covers it.

Cancel flow:

```
seller → POST /listings/:id/cancel/prepare
       ← svc returns { cancel_listing_ix }
seller → wallet signs + submits cancel_listing tx (seller is the Signer
         required by the program; svc cannot do this on the seller's behalf)
       → POST /listings/:id/cancel/confirm { tx_signature }
       ← svc records tx_signature; webhook flips listings.state to 'cancelled'
```

### 4.3 Indexer / reconciler (chain → svc DB → engine notify)

Two paths to ensure ground truth is the chain, not the DB:

**Webhook (primary, low-latency):**

```
Helius webhook → POST /internal/webhooks/helius
              → dedup by tx_signature (marketplace.webhook_events table)
              → for execute_purchase: update listing.purchased_at, ownership
              → for Bubblegum transfer: update ownership table
              → enqueue engine-notify job: persona_id ↔ new_owner_wallet
```

**DAS pull (fallback, eventual consistency):**

```
hourly job → for each managed collection:
  → fetch all assets via Helius DAS getAssetsByGroup
  → diff against marketplace.ownership table
  → reconcile any drift; alert on persistent drift > 3 cycles
```

**Engine notification:**

The engine is the source of chat access gating. Owners can chat; non-owners can't. The svc pushes ownership changes to the engine via a thin HTTP call (`POST /comp/internal/persona-ownership`, server-to-server auth). If the engine call fails, the engine's own background job pulls from svc on a schedule — both sides converge.

## 5. Crate layout

New repo at `/Users/enriquephlin/dev-local/oss-eros/eros-marketplace-svc/`. Mirrors the proven `eros-engine` 4-crate split (this is the 6-crate variant — extracting `chain` and `pinner` for testability since both have non-trivial external surface).

```
eros-marketplace-svc/
├── crates/
│   ├── eros-marketplace-svc-core/    # Domain types, SaleOrder (re-export from program crate),
│   │                                 # royalty math, persona_id derivation. Zero I/O.
│   ├── eros-marketplace-svc-kms/     # trait KmsProvider {
│   │                                 #   encrypt_dek(plaintext_dek) -> wrapped_dek;
│   │                                 #   decrypt_dek(wrapped_dek) -> plaintext_dek;
│   │                                 # }
│   │                                 # impls: supabase-vault (default), aws-kms, self-hosted
│   ├── eros-marketplace-svc-chain/   # Anchor client + mpl-bubblegum + mpl-core wrappers.
│   │                                 # trait ChainClient hides RPC for testability.
│   ├── eros-marketplace-svc-pinner/  # trait Pinner { pin_json(bytes) -> uri; pin_image(bytes) -> uri; }
│   │                                 # impls: irys (default), web3-storage, local-stub
│   ├── eros-marketplace-svc-store/   # sqlx Postgres, schema = 'marketplace'.
│   └── eros-marketplace-svc-server/  # axum + OpenAPI/Scalar + Helius webhook handler.
│                                     # Not published to crates.io; image to ghcr.io.
├── docker/  Dockerfile
├── fly.toml
├── rust-toolchain.toml
└── examples/
    └── seed_collection/              # admin CLI helper: create collection + register
```

Workspace `Cargo.toml` mirrors eros-engine's: `resolver = "3"`, `edition = "2024"`, `rust-version = "1.85"`, `license = "Apache-2.0"`.

Library crates `eros-marketplace-svc-core`, `-kms`, `-chain`, `-pinner`, `-store` get published to crates.io so closed-source downstream can depend on them. `-server` stays unpublished, shipped only as a Docker image to `ghcr.io/etherfunlab/eros-marketplace-svc`.

## 6. HTTP API surface (v0.1)

All `/listings/*`, `/mint/*`, `/personas/*` end-user routes require `Authorization: Bearer <Supabase JWT>` (same `AuthValidator` trait as `eros-engine`). `/admin/*` and `/internal/*` use separate keys.

### Public (catalog, signed-in browse)

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/listings` | Paginated catalog. Query: `state=active`, `sort`, `min_price`, `max_price`, `tag`, `search`, `nsfw`. Returns rich listing rows (manifest preview + on-chain state). |
| `GET` | `/listings/:id` | Single listing detail. |
| `GET` | `/personas/:persona_id` | Public Manifest preview (no plaintext prompt; just `name`, `avatar`, traits, etc.). |
| `GET` | `/collections` | List managed collections. |

### Authenticated user (mint, list, cancel)

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/mint/draft` | Submit `PersonaDraft` JSON + image bytes (multipart). Returns `mint_job_id`. |
| `GET` | `/mint/jobs/:id` | Poll mint state machine. Returns current `state` + `asset_id` if minted. |
| `POST` | `/listings/quote` | Allocate next `listing_nonce` for (asset, caller_wallet); return canonical SaleOrder bytes to be signed off-chain. No on-chain ix payload — svc itself submits `set_listing_quote` once the signature lands. |
| `POST` | `/listings` | Submit signed SaleOrder + seller signature. svc verifies sig + ownership + watermark, then submits `set_listing_quote` on-chain as admin and watches the webhook. |
| `POST` | `/listings/:id/cancel/prepare` | Return an unsigned `cancel_listing` ix payload (the program requires the seller as `Signer`, so seller must submit). |
| `POST` | `/listings/:id/cancel/confirm` | Record the seller-submitted `tx_signature`; webhook flips state. |
| `GET` | `/me/owned` | List asset_ids the caller's wallet owns (joined with manifest preview). |
| `GET` | `/me/listings` | List the caller's listings, any state. |

### Internal / admin

| Method | Path | Purpose |
|---|---|---|
| `POST` | `/internal/webhooks/helius` | Helius webhook target. Signed with shared secret. |
| `POST` | `/admin/collections` | Create + register a new Core collection (admin only). |
| `POST` | `/admin/listings/:id/takedown` | Force-remove a listing from catalog (e.g., content violation). |
| `GET` | `/admin/jobs/dead` | List failed mint jobs that exceeded retry budget. |

OpenAPI surfaced at `/api-docs/openapi.json`; Scalar UI at `/docs`. Same pattern as eros-engine.

## 7. Data model (Postgres, schema `marketplace`)

Concise sketch; column types and indexes finalized in the plan.

```
collections          (collection_pubkey PK, name, created_at, registered_at,
                      tree_config_pubkey, merkle_tree_pubkey)

mint_jobs            (id PK, creator_user_id, state ENUM, draft_jsonb,
                      persona_id, manifest_jsonb, manifest_uri, ciphertext_uri,
                      ciphertext_sha256, asset_id NULL, collection_pubkey,
                      retry_count, last_error, created_at, updated_at)

personas             (persona_id PK, asset_id UNIQUE, collection_pubkey FK,
                      manifest_uri, manifest_jsonb, name, traits_jsonb,
                      nsfw_flag, created_at)
                     # populated when mint_jobs transitions to 'indexed'

listings             (id PK, asset_id FK personas.asset_id,
                      seller_wallet, price_lamports, listing_nonce,
                      expires_at, seller_signature BYTEA,
                      state ENUM('pending_chain','active','cancelled','sold','expired'),
                      set_quote_tx_sig, sold_tx_sig, sold_to_wallet, sold_at,
                      created_at, updated_at,
                      UNIQUE (asset_id, seller_wallet, listing_nonce),
                      partial unique index (asset_id, seller_wallet)
                        WHERE state = 'active')   # at most one live listing per pair

ownership            (asset_id PK FK personas.asset_id,
                      owner_wallet, last_transfer_tx_sig, last_transfer_at)

webhook_events       (tx_signature PK, source ENUM('helius','das_reconcile'),
                      event_type, raw_jsonb, processed_at NULL, error_msg NULL)

listing_nonce_watermarks (asset_id, seller_wallet, last_issued_nonce u64,
                          updated_at,
                          PRIMARY KEY (asset_id, seller_wallet))
                     # strict-monotonic forever; matches the on-chain
                     # ListingState.last_seen_nonce semantics. Nonces are
                     # never reclaimed or reused.
```

`personas`, `ownership`, and `listings.state` are derived views of the chain. The webhook + DAS jobs maintain them. Anything else (drafts, mint jobs, signatures) is svc-native.

## 8. Decisions and assumptions

Each decision below is an explicit choice. The reasoning is captured so future-you can re-evaluate when the constraint changes.

| # | Decision | Why | What changes if revisited |
|---|---|---|---|
| 1 | New repo `eros-marketplace-svc` | `eros-marketplace-solana`'s README already references this name | Folding into a monorepo would invert several other decisions |
| 2 | Apache-2.0 | Aligns with `eros-marketplace-solana`; commercial features sit in closed-source downstream that depends on these crates | AGPL would force chain.rs to be open even if downstream wraps it |
| 3 | Axum + tokio + sqlx + Postgres | Matches `eros-engine`'s stack — shared ops, monitoring, migration tooling | Switching frameworks means duplicating Dockerfile, fly.toml, JWT middleware |
| 4 | KMS default = supabase-vault | Engine already uses Supabase; cheapest to operate at v0.1 scale | AWS KMS for prod-scale; trait is unchanged |
| 5 | Auth = Supabase JWT (`AuthValidator` trait) | Same identity layer as engine | Other IdPs supported by impl-ing the trait |
| 6 | Pinner default = Irys (Bundlr successor) | Native to Solana, low-friction billing in SOL | web3.storage or local-stub for tests; trait abstraction makes swap trivial |
| 7 | Moderation = stub trait | Content policy is a product decision, not a platform decision; closed-source downstream plugs in real classifiers | Real impl ships as separate crate downstream |
| 8 | svc submits the **admin-gated** on-chain ix as itself (`set_listing_quote`, `init_registries`, `register_collection`, `housekeeping_clear`); seller submits `cancel_listing`; buyer submits the purchase tx. svc never holds end-user keys. | The program's `has_one = admin` constraint on those four ix means svc is the only entity that can submit them. Buyer key custody is impossible because the Ed25519Program precompile forces a buyer-built tx | If sponsored gas for buyers becomes a goal, that's an orthogonal change; the admin-gated svc submission stays. |
| 9 | Indexer = Helius webhook + DAS pull | v0.1 cost / complexity floor | Self-hosted Geyser-based indexer if Helius spend becomes the gate |
| 10 | DB is cache; chain is truth | Recoverable from chain via DAS at any point | Don't add fields the chain doesn't know about (e.g., "favorited_at" — that's a different service) |
| 11 | `core` crate consumes `eros-marketplace-solana` program crate directly | SaleOrder canonical layout is the program's; manually mirroring it caused the v0.1.1 → v0.2 break | If we ever fork the program, this needs a feature-flagged stub |
| 12 | `core` crate consumes `eros-nft` crate directly | Manifest / Draft validation is the spec's; no mirror | Same as above for the spec |
| 13 | listing nonce is per-`(asset_id, seller_wallet)`, strict-monotonic forever | Mirrors the on-chain `ListingState.last_seen_nonce` semantics. A global Postgres sequence would issue nonces that fail the on-chain `NonceNotMonotonic` check the moment a new (asset, seller) PDA is created. Reclaiming or reusing a nonce is impossible by program design | Sharding the svc means partitioning the watermarks table by (asset, seller); the per-pair monotonic invariant must survive any sharding scheme |

## 9. Phasing

Each phase is independently shippable, independently testable, and leaves the tree green.

| Phase | Scope | Definition of done |
|---|---|---|
| **P1 — Bootstrap** | workspace skeleton, sqlx migrations, Anchor client, admin CLI for `register_collection` | CLI registers a devnet collection end-to-end; CI is green |
| **P2 — Mint pipeline** | `/mint/draft` → KMS → ciphertext store → Irys pin → Bubblegum `mint_v2`. Idempotent state machine. | Can take a `PersonaDraft` JSON to an `asset_id` on devnet |
| **P3 — Listings** | `/listings/quote` (returns unsigned ix payloads), `/listings` (verify off-chain sig + watch seller tx), `/listings/:id/cancel/{prepare,confirm}`, `GET /listings` catalog. | `eros-engine-web`'s `useMarketplace()` swaps from `placeholderListings` to `$fetch('/api/marketplace/listings')` against this service |
| **P4 — Indexer** | Helius webhook receive + dedup + replay; DAS hourly reconcile job; engine ownership push | A purchase on devnet flips `listings.state` and the buyer can chat with the persona in `eros-engine` |
| **P5 — Admin** | takedown, moderation queue read API, royalty audit endpoint, dead-job inspection | Closed-source downstream can begin building real moderation against this surface |

Sequencing rationale: P1 unlocks chain calls; P2 produces something to list; P3 makes listings real; P4 makes purchases observable; P5 makes the surface operable. Each later phase can be backed out without breaking earlier ones.

## 10. Risks and mitigations

| Risk | Mitigation |
|---|---|
| **SaleOrder canonical bytes drift from the program** | `core` crate depends on `eros-marketplace-solana` program crate; serialize via the program's `canonical_bytes()`. Never manually mirror the layout. |
| **Manifest schema drift from `eros-nft` spec** | `core` crate depends on `eros-nft` crate; use `PersonaManifest::validate()`, never reimplement the validator. |
| **Webhook replay or missed delivery** | Dedup by `tx_signature` in `webhook_events`. Hourly DAS reconcile catches anything the webhook missed. Alert on drift > 3 cycles. |
| **KMS decryption latency on the chat hot path** | The engine decrypts directly via the same KMS provider. svc only holds plaintext during mint (one-shot, async). |
| **Concurrent writes to the same listing pair** | The watermark table row for `(asset_id, seller_wallet)` is the serialization point — allocation uses `UPDATE ... WHERE last_issued_nonce = $expected RETURNING last_issued_nonce + 1` for optimistic concurrency. On-chain, `set_listing_quote`'s `NonceNotMonotonic` check is the second line of defense. |
| **svc allocates a nonce on `/listings/quote` but the seller never returns to `/listings`** | The watermark advances regardless — that nonce is burned forever. This is acceptable: the on-chain program treats nonces the same way (strict monotonic, no reclaim). Worst case is a small forward gap in seller's nonce history; no on-chain `ListingState` is ever created for the unused nonce because svc only submits `set_listing_quote` after receiving the signature. |
| **svc submits `set_listing_quote` on-chain but the tx fails or is reorged** | The DB row stays `pending_chain` with `set_quote_tx_sig` recorded. A reconcile job retries with a fresh nonce (the next watermark value) and a fresh canonical-bytes signature from the seller — the program will reject any retry that uses a nonce ≤ `last_seen_nonce`, so retries must always advance. |
| **Mint job stuck mid-state** | Each transition is a function `(state, payload) → next_state`; jobs that exceed `retry_count` land in `/admin/jobs/dead`. |
| **Admin key compromise** | Admin keypair lives behind KMS; svc requests temporary signing per ix. Blast radius is wider than a single ix: admin gates **`register_collection`**, **`init_registries`**, **`set_listing_quote`**, and **`housekeeping_clear`**. A compromised admin can: register hostile collections, poison royalty/manifest registries on un-initialized assets, advance any seller's nonce watermark to DoS them out of listing, or activate a signed SaleOrder nonce out-of-band. Mitigations: per-ix KMS authorization policies, audit log on every signing request, alert on unexpected `init_registries` / `register_collection` calls (low volume by design), key rotation cadence (see §KMS in the implementation plan). |

## 11. Out-of-scope (recorded for future rounds)

- Trained-persona / `eros-nft-extended` mint flow (entirely separate spec).
- Auction / Dutch auction listings.
- Multi-edition cNFTs.
- Cross-chain marketplaces.
- Real moderation classifier — stays in closed-source downstream.
- Sponsored gas for buyers.
- Push / email notifications.
- Mobile clients.
- Royalty splits > 2 recipients (would also require an on-chain change).

## 12. Files this design will touch (when implemented)

New repo at `/Users/enriquephlin/dev-local/oss-eros/eros-marketplace-svc/`. The implementation plan will enumerate every file; this design only commits to:

- 6 new crates under `crates/`
- `Dockerfile`, `fly.toml`, `rust-toolchain.toml`, workspace `Cargo.toml`, `CHANGELOG.md`, `README.md`
- `.github/workflows/ci.yml` modeled on `eros-engine`'s
- migrations under `crates/eros-marketplace-svc-store/migrations/`
- examples under `examples/seed_collection/`

The current `eros-nft` repo is unchanged by this design except for hosting this spec document.

## 13. Open follow-ups (next-round candidates)

- A separate read-only "ownership oracle" crate that engines other than `eros-engine` could embed.
- Sealed-bid auction extension (would touch the on-chain program too).
- Royalty distribution audit dashboard (admin-only).
- Mint-job pause/resume API for moderation appeals.
