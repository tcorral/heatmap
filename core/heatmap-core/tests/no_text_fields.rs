use heatmap_core::{
    CaptureCandidate, HeatmapEvent, InteractionEvent, InteractionType, PlatformInfo, PlatformType,
    Position, ScreenInfo, SdkInfo, Viewport, SCHEMA_VERSION,
};
use serde_json::Value;

fn forbidden_keys_in_value(value: &Value) -> Vec<String> {
    const FORBIDDEN: &[&str] = &["text", "label", "value", "title"];
    let mut found = Vec::new();
    collect_forbidden(value, FORBIDDEN, &mut found);
    found
}

fn collect_forbidden(value: &Value, forbidden: &[&str], found: &mut Vec<String>) {
    if let Some(object) = value.as_object() {
        for (key, nested) in object {
            if forbidden.contains(&key.as_str()) {
                found.push(key.clone());
            }
            collect_forbidden(nested, forbidden, found);
        }
    } else if let Some(array) = value.as_array() {
        for item in array {
            collect_forbidden(item, forbidden, found);
        }
    }
}

#[test]
fn should_serialize_heatmap_event_without_sensitive_json_keys() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_test".into(),
        anonymous_id: "anon_test".into(),
        user_id: None,
        timestamp: 1,
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
        screen: Some(ScreenInfo {
            id: "Home".into(),
            layout_version: None,
        }),
        event: InteractionEvent::click(Some("nav.home".into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: None,
    };
    let json = serde_json::to_value(&event).expect("serialize");
    assert!(
        forbidden_keys_in_value(&json).is_empty(),
        "unexpected sensitive keys in {json}"
    );
}

#[test]
fn capture_candidate_shape_has_no_text_label_or_value_fields() {
    let candidate = CaptureCandidate {
        timestamp: 1,
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
        interaction: InteractionEvent::click(None).expect("click"),
        position: None,
        viewport: Some(Viewport {
            width: 100,
            height: 100,
        }),
        window_id: None,
    };
    let CaptureCandidate {
        timestamp,
        sdk,
        platform,
        screen,
        interaction,
        position,
        viewport,
        window_id,
    } = candidate;
    assert_eq!(interaction.interaction_type, InteractionType::Click);
    let _ = (
        timestamp,
        sdk,
        platform,
        screen,
        interaction,
        position,
        viewport,
        window_id,
    );
}
