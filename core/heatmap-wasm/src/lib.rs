mod config;
pub mod public_api;
mod stub_transport;

#[cfg(target_arch = "wasm32")]
mod fetch_transport;
#[cfg(target_arch = "wasm32")]
mod local_storage_identity;
#[cfg(target_arch = "wasm32")]
mod web_context;

pub use public_api::PublicHeatmapSdk;

#[cfg(target_arch = "wasm32")]
mod wasm_bindings;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindings::HeatmapSdkJs;
