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
pub const STORE_KEY_RETRY_BATCH: &str = "heatmap.retry_batch";

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
        self.data.lock().ok().and_then(|map| map.get(key).cloned())
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
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-' {
                    c
                } else {
                    '_'
                }
            })
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
                let _ = persist_value(&path, v);
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

fn persist_value(dest: &Path, contents: &str) -> io::Result<()> {
    let tmp = dest.with_extension("tmp");
    let result = write_atomic(&tmp, contents).and_then(|_| replace_file(&tmp, dest));
    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

fn replace_file(tmp: &Path, dest: &Path) -> io::Result<()> {
    match fs::rename(tmp, dest) {
        Ok(()) => Ok(()),
        Err(err) => replace_existing_destination(tmp, dest, err),
    }
}

fn replace_existing_destination(tmp: &Path, dest: &Path, rename_err: io::Error) -> io::Result<()> {
    if !tmp.exists() || !dest.exists() {
        return Err(rename_err);
    }
    fs::remove_file(dest)?;
    fs::rename(tmp, dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()))
    }

    #[test]
    fn should_replace_existing_destination_file() {
        let dir = unique_temp_dir("heatmap-replace-unit");
        fs::create_dir_all(&dir).expect("dir");
        let dest = dir.join("queue.dat");
        let tmp = dir.join("queue.tmp");
        fs::write(&dest, b"old").expect("dest");
        fs::write(&tmp, b"new").expect("tmp");
        replace_file(&tmp, &dest).expect("replace");
        assert_eq!(fs::read_to_string(&dest).expect("read"), "new");
        assert!(!tmp.exists(), "tmp file removed by replace");
    }

    #[test]
    fn should_return_rename_error_when_tmp_is_missing() {
        let dir = unique_temp_dir("heatmap-replace-missing");
        fs::create_dir_all(&dir).expect("dir");
        let dest = dir.join("queue.dat");
        let tmp = dir.join("missing.tmp");
        fs::write(&dest, b"keep").expect("dest");
        assert!(replace_file(&tmp, &dest).is_err());
        assert_eq!(fs::read_to_string(&dest).expect("read"), "keep");
    }
}
