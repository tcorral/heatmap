//! Native FFI for the Heatmap SDK (UniFFI + C ABI).
//!
//! Generate Swift/Kotlin bindings (install matching bindgen once):
//!
//! ```text
//! cargo install uniffi-bindgen --version 0.28.3 --locked
//! uniffi-bindgen generate \
//!   --config core/heatmap-ffi/uniffi.toml \
//!   --crate heatmap_ffi \
//!   --language swift \
//!   --out-dir /path/to/bindings
//! ```
//!
//! Repo scripts: `sdk/apple/scripts/generate-uniffi.sh` (Swift),
//! `sdk/android/scripts/generate-uniffi.sh` (Kotlin; run against Android `.so` for release).

mod android_adapter;
mod config;
mod errors;
mod flush_lock;
mod ios_adapter;
mod macos_adapter;
pub mod public_api;
mod uniffi_api;
mod windows_adapter;

pub mod c_abi;

pub use public_api::PublicHeatmapSdk;

uniffi::setup_scaffolding!();
