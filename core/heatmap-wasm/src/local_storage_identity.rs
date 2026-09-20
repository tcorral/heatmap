#[cfg(target_arch = "wasm32")]
use heatmap_core::IdentityStore;
#[cfg(target_arch = "wasm32")]
use web_sys::{window, Storage};

#[cfg(target_arch = "wasm32")]
pub struct LocalStorageIdentityStore;

#[cfg(target_arch = "wasm32")]
impl LocalStorageIdentityStore {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for LocalStorageIdentityStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
impl IdentityStore for LocalStorageIdentityStore {
    fn read(&self, key: &str) -> Option<String> {
        storage()?.get_item(key).ok().flatten()
    }

    fn write(&self, key: &str, value: Option<&str>) {
        let Some(storage) = storage() else {
            return;
        };
        match value {
            Some(v) => {
                let _ = storage.set_item(key, v);
            }
            None => {
                let _ = storage.remove_item(key);
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn storage() -> Option<Storage> {
    window()?.local_storage().ok().flatten()
}
