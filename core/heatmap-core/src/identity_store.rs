use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub const STORE_KEY_ANONYMOUS_ID: &str = "heatmap.anonymous_id";
pub const STORE_KEY_USER_ID: &str = "heatmap.user_id";
pub const STORE_KEY_SESSION_ID: &str = "heatmap.session_id";
pub const STORE_KEY_LAST_EVENT_AT: &str = "heatmap.last_event_at";
pub const STORE_KEY_SAMPLE_IN: &str = "heatmap.sample_in";
pub const STORE_KEY_QUEUE: &str = "heatmap.queue";

pub trait IdentityStore: Send + Sync {
    fn read(&self, key: &str) -> Option<String>;
    fn write(&self, key: &str, value: Option<&str>);
}

pub struct InMemoryIdentityStore {
    data: Mutex<HashMap<String, String>>,
}

impl InMemoryIdentityStore {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryIdentityStore {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityStore for InMemoryIdentityStore {
    fn read(&self, key: &str) -> Option<String> {
        self.data
            .lock()
            .ok()
            .and_then(|map| map.get(key).cloned())
    }

    fn write(&self, key: &str, value: Option<&str>) {
        let Ok(mut map) = self.data.lock() else {
            return;
        };
        match value {
            Some(v) => {
                map.insert(key.to_owned(), v.to_owned());
            }
            None => {
                map.remove(key);
            }
        }
    }
}

/// Disk-backed [IdentityStore] for mobile adapters (queue + session survive process death).
pub struct FileIdentityStore {
    dir: PathBuf,
}

impl FileIdentityStore {
    pub fn new(dir: PathBuf) -> io::Result<Self> {
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    fn path_for(&self, key: &str) -> PathBuf {
        let safe: String = key
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' { c } else { '_' })
            .collect();
        self.dir.join(format!("{safe}.dat"))
    }
}

impl IdentityStore for FileIdentityStore {
    fn read(&self, key: &str) -> Option<String> {
        let path = self.path_for(key);
        fs::read_to_string(path).ok()
    }

    fn write(&self, key: &str, value: Option<&str>) {
        let path = self.path_for(key);
        match value {
            Some(v) => {
                let tmp = path.with_extension("tmp");
                if write_atomic(&tmp, v).is_ok() {
                    let _ = fs::rename(&tmp, &path);
                }
            }
            None => {
                let _ = fs::remove_file(path);
            }
        }
    }
}

fn write_atomic(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents.as_bytes())
}
