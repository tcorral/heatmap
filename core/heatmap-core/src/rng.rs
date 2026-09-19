use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub trait RandomSource: Send + Sync {
    fn next_unit_f64(&self) -> f64;
}

pub struct SystemRng;

impl RandomSource for SystemRng {
    fn next_unit_f64(&self) -> f64 {
        #[cfg(target_arch = "wasm32")]
        {
            (js_sys::Math::random() as f64).clamp(0.0, 1.0)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let mixed = (nanos as u64).wrapping_mul(0x9E3779B97F4A7C15);
            (mixed as f64) / (u64::MAX as f64)
        }
    }
}

pub struct ManualRng {
    values: Mutex<Vec<f64>>,
    draw_count: AtomicUsize,
}

impl ManualRng {
    pub fn sequence(values: Vec<f64>) -> Self {
        Self {
            values: Mutex::new(values),
            draw_count: AtomicUsize::new(0),
        }
    }

    pub fn draw_count(&self) -> usize {
        self.draw_count.load(Ordering::SeqCst)
    }
}

impl RandomSource for ManualRng {
    fn next_unit_f64(&self) -> f64 {
        self.draw_count.fetch_add(1, Ordering::SeqCst);
        let mut guard = self.values.lock().expect("manual rng lock");
        if guard.is_empty() {
            return 0.0;
        }
        if guard.len() == 1 {
            return guard[0];
        }
        guard.remove(0)
    }
}

pub(crate) fn session_sampled_in(sample_rate: f64, draw: f64) -> bool {
    if sample_rate >= 1.0 {
        return true;
    }
    if sample_rate <= 0.0 {
        return false;
    }
    draw < sample_rate
}
