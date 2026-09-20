pub const DEFAULT_MAX_EVENTS_PER_BATCH: usize = 200;
pub const DEFAULT_MAX_BATCH_BYTES: usize = 262_144;
pub const DEFAULT_BATCHES_PER_MINUTE: u32 = 120;

#[derive(Clone, Debug)]
pub struct AppConfig {
    project_keys: Vec<String>,
    expose_test_events: bool,
    max_events_per_batch: usize,
    max_batch_bytes: usize,
    batches_per_minute: u32,
}

impl AppConfig {
    pub fn with_project_keys(keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            project_keys: keys.into_iter().map(Into::into).collect(),
            expose_test_events: false,
            max_events_per_batch: DEFAULT_MAX_EVENTS_PER_BATCH,
            max_batch_bytes: DEFAULT_MAX_BATCH_BYTES,
            batches_per_minute: DEFAULT_BATCHES_PER_MINUTE,
        }
    }

    pub fn with_batches_per_minute(mut self, limit: u32) -> Self {
        self.batches_per_minute = limit;
        self
    }

    pub fn max_events_per_batch(&self) -> usize {
        self.max_events_per_batch
    }

    pub fn max_batch_bytes(&self) -> usize {
        self.max_batch_bytes
    }

    pub fn batches_per_minute(&self) -> u32 {
        self.batches_per_minute
    }

    pub fn with_test_events(mut self, enabled: bool) -> Self {
        self.expose_test_events = enabled;
        self
    }

    pub fn expose_test_events(&self) -> bool {
        self.expose_test_events
    }

    pub fn is_valid_project_key(&self, key: &str) -> bool {
        self.project_keys.iter().any(|candidate| candidate == key)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::with_project_keys([super::TEST_PROJECT_KEY])
    }
}
