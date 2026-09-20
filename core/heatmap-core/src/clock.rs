use std::sync::atomic::{AtomicI64, Ordering};

pub trait Clock: Send + Sync {
    fn now_ms(&self) -> i64;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ms(&self) -> i64 {
        #[cfg(target_arch = "wasm32")]
        {
            js_sys::Date::now() as i64
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis() as i64)
                .unwrap_or(0)
        }
    }
}

pub struct ManualClock {
    millis: AtomicI64,
}

impl ManualClock {
    pub fn new(start_ms: i64) -> Self {
        Self {
            millis: AtomicI64::new(start_ms),
        }
    }

    pub fn advance_ms(&self, delta: u64) {
        self.millis.fetch_add(delta as i64, Ordering::SeqCst);
    }
}

impl Clock for ManualClock {
    fn now_ms(&self) -> i64 {
        self.millis.load(Ordering::SeqCst)
    }
}
