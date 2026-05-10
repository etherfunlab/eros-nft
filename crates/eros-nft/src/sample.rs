//! Embedded sample loader. Each sample lives at
//! `samples/persona-<slug>/{draft.json,manifest.json,README.md}` in the repo.
//!
//! The bundled sample names are stable; new samples may be added in minor crate
//! versions but never removed in a patch release.

use include_dir::{Dir, include_dir};
use serde_json::Value;

use crate::types::{PersonaDraft, PersonaManifest};

static SAMPLES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/samples");

/// List the slugs of all bundled samples (e.g. `"yuki-warm-senpai"`).
pub fn list_samples() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = SAMPLES_DIR
        .dirs()
        .filter_map(|d| d.path().file_name()?.to_str()?.strip_prefix("persona-"))
        .collect();
    out.sort_unstable();
    out
}

/// Load a bundled sample by slug. Returns `(draft, manifest)` parsed into typed
/// structs. Returns `None` if the slug is unknown.
pub fn load_sample(slug: &str) -> Option<(PersonaDraft, PersonaManifest)> {
    let dirname = format!("persona-{slug}");
    let dir = SAMPLES_DIR.get_dir(&dirname)?;
    let draft_bytes = dir.get_file(format!("{dirname}/draft.json"))?.contents();
    let manifest_bytes = dir.get_file(format!("{dirname}/manifest.json"))?.contents();
    let draft: PersonaDraft = serde_json::from_slice(draft_bytes).ok()?;
    let manifest: PersonaManifest = serde_json::from_slice(manifest_bytes).ok()?;
    Some((draft, manifest))
}

/// Raw access to a sample's bytes for tooling that bypasses typed parsing.
pub fn load_sample_raw(slug: &str) -> Option<(Value, Value)> {
    let dirname = format!("persona-{slug}");
    let dir = SAMPLES_DIR.get_dir(&dirname)?;
    let draft_bytes = dir.get_file(format!("{dirname}/draft.json"))?.contents();
    let manifest_bytes = dir.get_file(format!("{dirname}/manifest.json"))?.contents();
    let draft: Value = serde_json::from_slice(draft_bytes).ok()?;
    let manifest: Value = serde_json::from_slice(manifest_bytes).ok()?;
    Some((draft, manifest))
}
