use std::collections::VecDeque;

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

pub fn event_json_bytes(event: &HeatmapEvent) -> usize {
    serde_json::to_vec(event).map(|bytes| bytes.len()).unwrap_or(0)
}

pub fn queue_json_bytes(queue: &VecDeque<HeatmapEvent>) -> usize {
    queue.iter().map(event_json_bytes).sum()
}

pub fn load_persisted_queue(store: &dyn IdentityStore) -> VecDeque<HeatmapEvent> {
    let Some(raw) = store.read(STORE_KEY_QUEUE) else {
        return VecDeque::new();
    };
    if raw.trim().is_empty() {
        return VecDeque::new();
    }
    serde_json::from_str(&raw).unwrap_or_default()
}

pub fn persist_queue(store: &dyn IdentityStore, queue: &VecDeque<HeatmapEvent>) {
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
    queue: &mut VecDeque<HeatmapEvent>,
    queued_bytes: &mut usize,
    event: HeatmapEvent,
    limits: &QueueLimits,
) -> u64 {
    *queued_bytes = queued_bytes.saturating_add(event_json_bytes(&event));
    queue.push_back(event);
    trim_oldest_until_within_limits(queue, queued_bytes, limits)
}

fn trim_oldest_until_within_limits(
    queue: &mut VecDeque<HeatmapEvent>,
    queued_bytes: &mut usize,
    limits: &QueueLimits,
) -> u64 {
    let mut dropped = 0u64;
    while queue.len() > 1 && queue_over_limits(queue.len(), *queued_bytes, limits) {
        if let Some(removed) = queue.pop_front() {
            *queued_bytes = queued_bytes.saturating_sub(event_json_bytes(&removed));
            dropped = dropped.saturating_add(1);
        }
    }
    dropped
}

fn queue_over_limits(len: usize, serialized_bytes: usize, limits: &QueueLimits) -> bool {
    len > limits.max_events || serialized_bytes > limits.max_bytes
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
        let mut queue = VecDeque::from([sample_event(1), sample_event(2)]);
        let mut queued_bytes = queue_json_bytes(&queue);
        let dropped = push_with_overflow(&mut queue, &mut queued_bytes, sample_event(3), &limits);
        assert_eq!(dropped, 1);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0].timestamp, 2);
        assert_eq!(queue[1].timestamp, 3);
    }

    #[test]
    fn should_drop_oldest_when_serialized_bytes_exceed_limit() {
        let mut queue = VecDeque::from([sample_event(1)]);
        let mut queued_bytes = queue_json_bytes(&queue);
        let limits = QueueLimits {
            max_events: usize::MAX,
            max_bytes: queued_bytes,
        };
        let dropped = push_with_overflow(&mut queue, &mut queued_bytes, sample_event(2), &limits);
        assert_eq!(dropped, 1);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue[0].timestamp, 2);
    }
}
