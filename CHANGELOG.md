# Changelog

All notable changes to the `eros-nft` crate are documented here. The crate
follows [SemVer](https://semver.org/). The spec is versioned independently
under `spec/CHANGELOG.md`.

## [0.2.0] — 2026-05-12

First crates.io release. Supersedes the v0.1.0 tag (never published).

### Changed (breaking)

- `TipPersonality` enum is realigned with the values the reference engine
  (`eros-engine`) actually implements: `gold_digger`, `tsundere`, `zen`,
  `slow_warm`, `default`. The previous v0.1.0 set listed 14 values, of which
  12 silently fell back to the engine's default gift-reaction style, and two
  engine-supported values (`gold_digger`, `zen`) were not expressible at all.
  Old values (`dominant`, `warm_safe`, `tough_love`, `flirty`,
  `calm_professional`, `playful_chaotic`, `nostalgic`, `dramatic`, `warm_loud`,
  `sensual`, `playful`) now fail schema validation; remap to the closest
  supported value or `default`.
- Bundled samples remapped accordingly. The persona character descriptions
  are unchanged; only the engine-routing field moves.

### Build

- `crates/eros-nft/spec` and `crates/eros-nft/samples` are now symlinks to the
  canonical top-level `spec/` and `samples/` directories. Eliminates the
  duplicate copies that v0.1.0 carried for crates.io packaging. `cargo package`
  follows the symlinks and inlines the actual file contents into the published
  tarball, so consumer behavior is unchanged.

## [0.1.0] — 2026-05-10

Initial release.

- Implements the `eros-nft v1.0` spec.
- Types: `PersonaDraft`, `PersonaManifest`, `Compliance`, `PromptCiphertextRef`,
  `Behavior`, `TipPersonality`, `AffinityPriors`, `AvatarRef`, `AvatarSource`.
- JSON Schema 2020-12 validators with the spec schemas embedded.
- Embedded sample loader for the 15 bundled personas.
- CLI: `eros-nft validate <file>`, `eros-nft schema export {draft|manifest}`,
  `eros-nft sample list`, `eros-nft sample show <slug>`.
