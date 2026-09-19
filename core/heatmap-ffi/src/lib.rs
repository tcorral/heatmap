//! Native FFI for the Heatmap SDK (UniFFI + C ABI).
//!
//! Generate Swift/Kotlin bindings (install matching bindgen once):
//!
//! ```text
//! cargo install uniffi-bindgen --version 0.28.3 --locked
//! uniffi-bindgen generate \
//!   --config core/heatmap-ffi/uniffi.toml \
//!   --crate core/heatmap-ffi \
//!   --language swift \
//!   --out-dir /path/to/bindings
//! ```

mod config;
mod errors;
pub mod public_api;
mod uniffi_api;

pub mod c_abi;

pub use public_api::PublicHeatmapSdk;

uniffi::setup_scaffolding!();
