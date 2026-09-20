use std::fs;
use std::path::PathBuf;

use heatmap_core::{
    HeatmapEvent, InteractionEvent, InteractionType, PlatformInfo, PlatformType, Position,
    ScreenInfo, SdkInfo, Viewport, SCHEMA_VERSION,
};
use serde_json::Value;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test/fixtures")
        .join(name)
}

fn load_fixture(name: &str) -> Value {
    let raw = fs::read_to_string(fixture_path(name)).expect("read fixture");
    serde_json::from_str(&raw).expect("parse fixture")
}

fn assert_serializes_to_fixture(event: &HeatmapEvent, fixture_name: &str) {
    let expected = load_fixture(fixture_name);
    let actual = serde_json::to_value(event).expect("serialize event");
    assert_eq!(actual, expected);
}

#[test]
fn should_match_session_start_web_fixture_when_all_optionals_present() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_a1b2c3d4-e5f6-7890-abcd-ef1234567890".into(),
        anonymous_id: "anon_web_001".into(),
        user_id: Some("user_checkout_demo".into()),
        timestamp: 1_726_771_200_000,
        window_id: Some("win_browser_main".into()),
        sdk: SdkInfo {
            name: "heatmap-web".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Web,
            os_version: "macOS 14.6".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Home".into(),
            layout_version: None,
        }),
        event: InteractionEvent::session_start().expect("session_start"),
        position: None,
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
    };

    assert_serializes_to_fixture(&event, "session-start-web.json");
}

#[test]
fn should_match_minimal_session_start_android_fixture_when_optionals_omitted() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_android_minimal_001".into(),
        anonymous_id: "anon_android_001".into(),
        user_id: None,
        timestamp: 1_726_771_200_100,
        window_id: None,
        sdk: SdkInfo {
            name: "heatmap-android".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Android,
            os_version: "Android 14".into(),
            app_version: "1.0.0".into(),
        },
        screen: None,
        event: InteractionEvent::session_start().expect("session_start"),
        position: None,
        viewport: None,
    };

    assert_serializes_to_fixture(&event, "session-start-minimal-android.json");
}

#[test]
fn should_match_screen_view_home_ios_fixture() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_ios_journey_001".into(),
        anonymous_id: "anon_ios_001".into(),
        user_id: None,
        timestamp: 1_726_771_200_200,
        window_id: None,
        sdk: SdkInfo {
            name: "heatmap-ios".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Ios,
            os_version: "iOS 17.5".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Home".into(),
            layout_version: Some("2024-09".into()),
        }),
        event: InteractionEvent::screen_view().expect("screen_view"),
        position: None,
        viewport: Some(Viewport {
            width: 390,
            height: 844,
        }),
    };

    assert_serializes_to_fixture(&event, "screen-view-home-ios.json");
}

#[test]
fn should_match_click_checkout_pay_web_fixture() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_a1b2c3d4-e5f6-7890-abcd-ef1234567890".into(),
        anonymous_id: "anon_web_001".into(),
        user_id: Some("user_checkout_demo".into()),
        timestamp: 1_726_771_205_000,
        window_id: Some("win_browser_main".into()),
        sdk: SdkInfo {
            name: "heatmap-web".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Web,
            os_version: "macOS 14.6".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Checkout".into(),
            layout_version: Some("2024-09".into()),
        }),
        event: InteractionEvent::click(Some("checkout.pay".into())).expect("click"),
        position: Some(Position::new(0.5, 0.5).expect("position")),
        viewport: Some(Viewport {
            width: 1280,
            height: 720,
        }),
    };

    assert_serializes_to_fixture(&event, "click-checkout-pay-web.json");
}

#[test]
fn should_match_unmarked_click_android_fixture_without_element_id() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_android_journey_001".into(),
        anonymous_id: "anon_android_002".into(),
        user_id: None,
        timestamp: 1_726_771_204_500,
        window_id: None,
        sdk: SdkInfo {
            name: "heatmap-android".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Android,
            os_version: "Android 14".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Checkout".into(),
            layout_version: None,
        }),
        event: InteractionEvent::click(None).expect("click"),
        position: Some(Position::new(0.12, 0.88).expect("position")),
        viewport: Some(Viewport {
            width: 412,
            height: 915,
        }),
    };

    assert_serializes_to_fixture(&event, "click-unmarked-cancel-android.json");
}

#[test]
fn should_match_scroll_checkout_macos_fixture() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_macos_journey_001".into(),
        anonymous_id: "anon_macos_001".into(),
        user_id: None,
        timestamp: 1_726_771_204_000,
        window_id: Some("win_checkout_primary".into()),
        sdk: SdkInfo {
            name: "heatmap-macos".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Macos,
            os_version: "macOS 14.6".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Checkout".into(),
            layout_version: None,
        }),
        event: InteractionEvent::scroll().expect("scroll"),
        position: None,
        viewport: Some(Viewport {
            width: 1440,
            height: 900,
        }),
    };

    assert_serializes_to_fixture(&event, "scroll-checkout-macos.json");
}

#[test]
fn should_match_session_end_windows_fixture() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_windows_journey_001".into(),
        anonymous_id: "anon_windows_001".into(),
        user_id: Some("user_win_demo".into()),
        timestamp: 1_726_771_210_000,
        window_id: Some("hwnd_checkout".into()),
        sdk: SdkInfo {
            name: "heatmap-windows".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Windows,
            os_version: "Windows 11 23H2".into(),
            app_version: "1.0.0".into(),
        },
        screen: Some(ScreenInfo {
            id: "Confirmation".into(),
            layout_version: None,
        }),
        event: InteractionEvent::session_end().expect("session_end"),
        position: None,
        viewport: Some(Viewport {
            width: 1920,
            height: 1080,
        }),
    };

    assert_serializes_to_fixture(&event, "session-end-windows.json");
}

#[test]
fn should_omit_absent_optional_fields_from_serialized_json() {
    let event = HeatmapEvent {
        schema_version: SCHEMA_VERSION,
        session_id: "sess_android_minimal_001".into(),
        anonymous_id: "anon_android_001".into(),
        user_id: None,
        timestamp: 1_726_771_200_100,
        window_id: None,
        sdk: SdkInfo {
            name: "heatmap-android".into(),
            version: "0.1.0".into(),
        },
        platform: PlatformInfo {
            platform_type: PlatformType::Android,
            os_version: "Android 14".into(),
            app_version: "1.0.0".into(),
        },
        screen: None,
        event: InteractionEvent::session_start().expect("session_start"),
        position: None,
        viewport: None,
    };

    let value = serde_json::to_value(&event).expect("serialize");
    let object = value.as_object().expect("object");
    for key in ["user_id", "window_id", "screen", "position", "viewport"] {
        assert!(
            !object.contains_key(key),
            "expected {key} to be omitted, got {value}"
        );
    }
    let event_obj = object
        .get("event")
        .and_then(Value::as_object)
        .expect("event");
    assert!(!event_obj.contains_key("element_id"));
}

#[test]
fn should_accept_position_at_window_fraction_boundaries() {
    assert!(Position::new(0.0, 0.0).is_ok());
    assert!(Position::new(1.0, 1.0).is_ok());
}

#[test]
fn should_reject_position_outside_window_fraction_range() {
    assert!(Position::new(-0.01, 0.5).is_err());
    assert!(Position::new(0.5, 1.01).is_err());
}

#[test]
fn should_reject_out_of_scope_interaction_type_strings() {
    for label in ["tap", "custom", "hover", "visibility", "unknown"] {
        let result = InteractionType::parse(label);
        assert!(result.is_err(), "expected {label} to be rejected");
    }
}

#[test]
fn should_reject_element_id_on_non_click_interaction_types() {
    let err = InteractionEvent::new(InteractionType::Scroll, Some("checkout.pay".into()));
    assert!(err.is_err());
}
