use std::collections::HashMap;
use std::time::{Duration, Instant};

const WINDOW: Duration = Duration::from_secs(60);

pub struct RateLimiter {
    windows: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn allow(&mut self, project_key: &str, limit: u32) -> bool {
        let now = Instant::now();
        let entries = self.windows.entry(project_key.to_string()).or_default();
        entries.retain(|stamp| now.duration_since(*stamp) < WINDOW);
        if entries.len() >= limit as usize {
            return false;
        }
        entries.push(now);
        true
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
