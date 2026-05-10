# Changelog

All notable changes to the `eros-nft` crate are documented here. The crate
follows [SemVer](https://semver.org/). The spec is versioned independently
under `spec/CHANGELOG.md`.

## [Unreleased]

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
