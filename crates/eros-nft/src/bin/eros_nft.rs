//! eros-nft CLI: validate, schema export, sample inspection.

use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use eros_nft::{
    json_schema_draft, json_schema_manifest, list_samples, load_sample,
    PersonaDraft, PersonaManifest,
};

#[derive(Parser)]
#[command(name = "eros-nft", version, about = "Validate and inspect eros-nft documents.")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Validate a Draft or Manifest JSON file. Auto-detects type.
    Validate {
        /// Path to a JSON file containing a PersonaDraft or PersonaManifest.
        path: PathBuf,
    },
    /// Inspect or export embedded JSON Schemas.
    Schema {
        #[command(subcommand)]
        action: SchemaAction,
    },
    /// Inspect bundled sample personas.
    Sample {
        #[command(subcommand)]
        action: SampleAction,
    },
}

#[derive(Subcommand)]
enum SchemaAction {
    /// Print an embedded JSON Schema to stdout.
    Export {
        /// Which schema to export.
        kind: SchemaKind,
    },
}

#[derive(ValueEnum, Clone)]
enum SchemaKind {
    Draft,
    Manifest,
}

#[derive(Subcommand)]
enum SampleAction {
    /// List the slugs of all bundled samples.
    List,
    /// Print a sample's draft.json and manifest.json.
    Show {
        /// Sample slug, e.g. "yuki-warm-senpai".
        slug: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Validate { path } => cmd_validate(path),
        Cmd::Schema { action: SchemaAction::Export { kind } } => {
            match kind {
                SchemaKind::Draft => println!("{}", json_schema_draft()),
                SchemaKind::Manifest => println!("{}", json_schema_manifest()),
            }
            Ok(())
        }
        Cmd::Sample { action: SampleAction::List } => {
            for s in list_samples() {
                println!("{s}");
            }
            Ok(())
        }
        Cmd::Sample { action: SampleAction::Show { slug } } => {
            let (draft, manifest) =
                load_sample(&slug).ok_or_else(|| anyhow!("unknown sample: {slug}"))?;
            println!("=== draft.json ===");
            println!("{}", serde_json::to_string_pretty(&draft)?);
            println!("\n=== manifest.json ===");
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
    }
}

fn cmd_validate(path: PathBuf) -> Result<()> {
    let bytes = std::fs::read(&path).with_context(|| format!("reading {path:?}"))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .with_context(|| format!("parsing JSON from {path:?}"))?;

    // Auto-detect: Manifest has `persona_id`; Draft does not.
    if value.get("persona_id").is_some() {
        let m: PersonaManifest = serde_json::from_value(value).context("parsing as Manifest")?;
        m.validate().context("validating Manifest")?;
        println!("ok: PersonaManifest {path:?}");
    } else {
        let d: PersonaDraft = serde_json::from_value(value).context("parsing as Draft")?;
        d.validate().context("validating Draft")?;
        println!("ok: PersonaDraft {path:?}");
    }
    Ok(())
}
