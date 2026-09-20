use std::sync::OnceLock;

use crate::batch::HeatmapBatch;
use crate::event::HeatmapEvent;

pub const FLUSH_MAX_EVENTS: usize = 50;
pub const FLUSH_MAX_BYTES: usize = 65536;
pub const FLUSH_INTERVAL_MS: u64 = 10_000;
pub const INGEST_MAX_EVENTS: usize = 200;
pub const INGEST_MAX_BYTES: usize = 256 * 1024;
/// Cooldown applied after a permanent HTTP classification (400/401/403/413) so
/// newly captured events do not immediately trigger another doomed upload.
pub const PERMANENT_FAILURE_BACKOFF_MS: u64 = 60_000;

const BYTE_PROBE_BATCH_ID: &str = "batch_probe_00000000000000000000000000000000";

pub fn batch_wire_bytes(batch: &HeatmapBatch) -> usize {
    serde_json::to_vec(batch)
        .map(|bytes| bytes.len())
        .unwrap_or(usize::MAX)
}

fn empty_batch_envelope_bytes() -> usize {
    static ENVELOPE: OnceLock<usize> = OnceLock::new();
    *ENVELOPE.get_or_init(|| {
        let probe = HeatmapBatch {
            batch_id: BYTE_PROBE_BATCH_ID.into(),
            events: vec![],
        };
        batch_wire_bytes(&probe)
    })
}

pub fn estimated_batch_wire_bytes(events_bytes: usize, event_count: usize) -> usize {
    if event_count == 0 {
        return 0;
    }
    empty_batch_envelope_bytes() + 2 + events_bytes + event_count.saturating_sub(1)
}

pub fn queue_flush_byte_limit_reached(events_bytes: usize, event_count: usize) -> bool {
    if event_count == 0 {
        return false;
    }
    estimated_batch_wire_bytes(events_bytes, event_count) >= FLUSH_MAX_BYTES
}

pub fn queue_flush_byte_limit_reached_for_queue(
    queue: &[HeatmapEvent],
    events_bytes: usize,
) -> bool {
    queue_flush_byte_limit_reached(events_bytes, queue.len())
}

pub fn flush_event_limit_reached(queue_len: usize) -> bool {
    queue_len >= FLUSH_MAX_EVENTS
}

pub fn flush_interval_elapsed(pending_since_ms: i64, now_ms: i64) -> bool {
    now_ms.saturating_sub(pending_since_ms) as u64 >= FLUSH_INTERVAL_MS
}

pub fn flush_error_is_permanent(err: &crate::transport::FlushError) -> bool {
    match err {
        crate::transport::FlushError::Transport(_) => false,
        crate::transport::FlushError::HttpStatus { code, .. } => {
            matches!(*code, 400 | 401 | 403 | 413)
        }
    }
}

pub fn flush_error_is_transient(err: &crate::transport::FlushError) -> bool {
    match err {
        crate::transport::FlushError::Transport(_) => true,
        crate::transport::FlushError::HttpStatus { code, .. } => {
            *code == 429 || (*code >= 500 && *code < 600)
        }
    }
}
