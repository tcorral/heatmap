use std::collections::HashMap;
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
