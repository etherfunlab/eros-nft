# Contributing to eros-nft

Thank you for your interest in eros-nft. This repository hosts an open standard
and a reference Rust implementation. Different artifacts have different rules.

## Spec changes

Spec documents live in `spec/v1.0/` and are licensed CC-BY-4.0. Before opening
a PR that changes a published spec version, please:

1. Open a discussion or issue describing the proposed change and the motivating
   use case.
2. If accepted, the change goes into `spec/CHANGELOG.md` first; the spec
   version is bumped per the rules in `spec/v1.0/00-overview.md`.

## Crate changes

The Rust crate lives in `crates/eros-nft/` and is licensed Apache-2.0. PRs must:

1. Build clean: `cargo build --all`
2. Pass tests: `cargo test --all`
3. Pass lint: `cargo clippy --all -- -D warnings`
4. Pass format: `cargo fmt --check`

These checks run in CI on every PR.

## Sample personas

`samples/` contains demonstration personas. Each is a Draft + Manifest + README.
New samples are welcome but must:

1. Validate via `cargo run -- validate <path>`.
2. Carry plausible compliance attestations.
3. Not depict identifiable real people.

## Sign-off

By contributing you agree that your contributions are licensed under the
appropriate license for the artifact you change (CC-BY-4.0 for spec, Apache-2.0
for code).
