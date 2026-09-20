use std::collections::HashMap;
use std::time::{Duration, Instant};

const WINDOW: Duration = Duration::from_secs(60);

pub enum RateLimitDecision {
    Allowed,
    Denied { retry_after_secs: u64 },
}

pub struct RateLimiter {
    windows: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn check(&mut self, project_key: &str, limit: u32) -> RateLimitDecision {
        let now = Instant::now();
        let entries = self.windows.entry(project_key.to_string()).or_default();
        entries.retain(|stamp| now.duration_since(*stamp) < WINDOW);
        if entries.len() >= limit as usize {
            let retry_after_secs = entries
                .first()
                .map(|oldest| {
                    WINDOW
                        .saturating_sub(now.duration_since(*oldest))
                        .as_secs()
                        .max(1)
                })
                .unwrap_or(1);
            return RateLimitDecision::Denied { retry_after_secs };
        }
        entries.push(now);
        RateLimitDecision::Allowed
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
