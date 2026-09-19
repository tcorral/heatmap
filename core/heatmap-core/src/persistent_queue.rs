use crate::event::HeatmapEvent;
use crate::identity_store::{IdentityStore, STORE_KEY_QUEUE};

pub const QUEUE_MAX_EVENTS: usize = 10_000;
pub const QUEUE_MAX_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueLimits {
    pub max_events: usize,
    pub max_bytes: usize,
}

impl QueueLimits {
    pub fn production() -> Self {
        Self {
            max_events: QUEUE_MAX_EVENTS,
            max_bytes: QUEUE_MAX_BYTES,
        }
    }
}

pub fn load_persisted_queue(store: &dyn IdentityStore) -> Vec<HeatmapEvent> {
    let Some(raw) = store.read(STORE_KEY_QUEUE) else {
        return Vec::new();
    };
    if raw.trim().is_empty() {
        return Vec::new();
    }
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn persist_queue(store: &dyn IdentityStore, queue: &[HeatmapEvent]) {
    if queue.is_empty() {
        store.write(STORE_KEY_QUEUE, None);
        return;
    }
    let Ok(serialized) = serde_json::to_string(queue) else {
        return;
    };
    store.write(STORE_KEY_QUEUE, Some(serialized.as_str()));
}

pub fn push_with_overflow(
    queue: &mut Vec<HeatmapEvent>,
    event: HeatmapEvent,
    limits: &QueueLimits,
) -> u64 {
    queue.push(event);
    trim_oldest_until_within_limits(queue, limits)
}

fn trim_oldest_until_within_limits(queue: &mut Vec<HeatmapEvent>, limits: &QueueLimits) -> u64 {
    let mut dropped = 0u64;
    while queue.len() > 1 && queue_over_limits(queue, limits) {
        queue.remove(0);
        dropped = dropped.saturating_add(1);
    }
    dropped
}

fn queue_over_limits(queue: &[HeatmapEvent], limits: &QueueLimits) -> bool {
    queue.len() > limits.max_events || queue_serialized_bytes(queue) > limits.max_bytes
}

fn queue_serialized_bytes(queue: &[HeatmapEvent]) -> usize {
    queue
        .iter()
        .filter_map(|event| serde_json::to_vec(event).ok())
        .map(|bytes| bytes.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{
        HeatmapEvent, InteractionEvent, PlatformInfo, PlatformType, SdkInfo, SCHEMA_VERSION,
    };

    fn sample_event(timestamp: i64) -> HeatmapEvent {
        HeatmapEvent {
            schema_version: SCHEMA_VERSION,
            session_id: "sess_test".into(),
            anonymous_id: "anon_test".into(),
            user_id: None,
            timestamp,
            window_id: None,
            sdk: SdkInfo {
                name: "heatmap-web".into(),
                version: "0.1.0".into(),
            },
            platform: PlatformInfo {
                platform_type: PlatformType::Web,
                os_version: "macOS".into(),
                app_version: "1.0.0".into(),
            },
            screen: None,
            event: InteractionEvent::click(None).expect("click"),
            position: None,
            viewport: None,
        }
    }

    #[test]
    fn should_drop_oldest_when_event_count_exceeds_limit() {
        let limits = QueueLimits {
            max_events: 2,
            max_bytes: usize::MAX,
        };
        let mut queue = vec![sample_event(1), sample_event(2)];
        let dropped = push_with_overflow(&mut queue, sample_event(3), &limits);
        assert_eq!(dropped, 1);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0].timestamp, 2);
        assert_eq!(queue[1].timestamp, 3);
    }

    #[test]
    fn should_drop_oldest_when_serialized_bytes_exceed_limit() {
        let mut queue = vec![sample_event(1)];
        let one_event_bytes = queue_serialized_bytes(&queue);
        let limits = QueueLimits {
            max_events: usize::MAX,
            max_bytes: one_event_bytes,
        };
        let dropped = push_with_overflow(&mut queue, sample_event(2), &limits);
        assert_eq!(dropped, 1);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].timestamp, 2);
    }
}
