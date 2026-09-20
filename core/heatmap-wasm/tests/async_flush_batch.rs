use heatmap_core::INGEST_MAX_EVENTS;
use heatmap_wasm::PublicHeatmapSdk;

fn test_sdk() -> PublicHeatmapSdk {
    let config = r#"{
        "project_key": "pk_test",
        "endpoint": "https://ingest.example.com"
    }"#;
    PublicHeatmapSdk::init_from_json(config).expect("init")
}

fn record_click(sdk: &mut PublicHeatmapSdk, timestamp_ms: i64) {
    sdk.record_activation_click(
        0.5,
        0.5,
        Some("checkout.pay".into()),
        1280,
        720,
        timestamp_ms,
    )
    .expect("record");
}

#[test]
fn should_take_bounded_batch_on_begin_and_restore_on_failed_complete() {
    let mut sdk = test_sdk();
    let mut ts = 1_726_771_205_000_i64;
    while sdk.queued_event_count_for_testing() < INGEST_MAX_EVENTS + 5 {
        record_click(&mut sdk, ts);
        ts += 1;
    }
    let total_before = sdk.queued_event_count_for_testing();
    assert!(total_before > INGEST_MAX_EVENTS);

    let json = sdk
        .begin_flush_batch_json()
        .expect("serialized batch for upload");
    assert!(!json.is_empty());
    let remaining = sdk.queued_event_count_for_testing();
    assert!(remaining > 0, "overflow events stay queued during upload");
    assert!(remaining < total_before);

    sdk.complete_flush(false);
    assert!(
        sdk.queued_event_count_for_testing() > 0,
        "failed upload restores taken events"
    );
}

#[test]
fn should_drop_uploaded_events_after_successful_complete() {
    let mut sdk = test_sdk();
    record_click(&mut sdk, 1_726_771_205_000);
    let before = sdk.queued_event_count_for_testing();
    assert!(before > 0);

    let json = sdk.begin_flush_batch_json().expect("batch");
    assert!(!json.is_empty());
    assert!(sdk.queued_event_count_for_testing() < before);

    sdk.complete_flush(true);
    assert_eq!(
        sdk.queued_event_count_for_testing(),
        0,
        "accepted upload removes events from queue"
    );
    assert!(sdk.begin_flush_batch_json().is_none());
}

#[test]
fn should_no_op_complete_flush_when_no_pending_batch() {
    let mut sdk = test_sdk();
    record_click(&mut sdk, 1_726_771_205_000);
    sdk.complete_flush(true);
    sdk.complete_flush(false);
    assert!(sdk.queued_event_count_for_testing() > 0);
}

#[test]
fn should_drop_batch_on_permanent_http_complete_without_restore() {
    let mut sdk = test_sdk();
    record_click(&mut sdk, 1_726_771_205_000);
    let before = sdk.queued_event_count_for_testing();
    sdk.begin_flush_batch_json().expect("batch");
    sdk.complete_flush_http(401, None);
    assert_eq!(
        sdk.queued_event_count_for_testing(),
        0,
        "permanent HTTP failure drops the in-flight batch"
    );
    assert!(before > 0);
}

#[test]
fn should_restore_batch_on_transient_http_complete() {
    let mut sdk = test_sdk();
    record_click(&mut sdk, 1_726_771_205_000);
    let before = sdk.queued_event_count_for_testing();
    sdk.begin_flush_batch_json().expect("batch");
    sdk.complete_flush_http(503, None);
    assert_eq!(
        sdk.queued_event_count_for_testing(),
        before,
        "transient HTTP failure restores the in-flight batch"
    );
}

#[test]
fn should_restore_batch_on_http_429_complete() {
    let mut sdk = test_sdk();
    record_click(&mut sdk, 1_726_771_205_000);
    let before = sdk.queued_event_count_for_testing();
    sdk.begin_flush_batch_json().expect("batch");
    sdk.complete_flush_http(429, Some(5));
    assert_eq!(sdk.queued_event_count_for_testing(), before);
}

#[test]
fn should_not_begin_flush_before_retry_after_and_allow_after_expiry() {
    use std::sync::Arc;

    use heatmap_core::ManualClock;

    let clock = Arc::new(ManualClock::new(0));
    let mut sdk = PublicHeatmapSdk::init_from_json_with_clock(
        r#"{
            "project_key": "pk_test",
            "endpoint": "https://ingest.example.com"
        }"#,
        clock.clone(),
    )
    .expect("init");
    record_click(&mut sdk, 1_000);
    sdk.begin_flush_batch_json().expect("first upload");
    sdk.complete_flush_http(429, Some(5));
    assert!(sdk.begin_flush_batch_json().is_none());
    clock.advance_ms(4_999);
    assert!(sdk.begin_flush_batch_json().is_none());
    clock.advance_ms(1);
    assert!(sdk.begin_flush_batch_json().is_some());
}
