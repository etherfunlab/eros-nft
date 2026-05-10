//! eros-nft: reference implementation of the eros-nft v1 spec.
//!
//! See <https://github.com/etherfunlab/eros-nft> for the spec and samples.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! use eros_nft::{PersonaManifest, load_sample};
//!
//! let (_draft, manifest) = load_sample("yuki-warm-senpai").unwrap();
//! // manifest.validate().unwrap(); // validate() added in Phase 6
//! ```

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod error;
pub mod sample;
pub mod schema;
pub mod types;
pub mod validate;

pub use error::ValidationError;
pub use sample::{list_samples, load_sample};
pub use schema::{json_schema_draft, json_schema_manifest};
pub use types::*;
