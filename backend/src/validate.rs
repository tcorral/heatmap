use serde_json::Value;

const EVENT_TYPES: &[&str] = &[
    "session_start",
    "session_end",
    "screen_view",
    "click",
    "scroll",
];

const PLATFORM_TYPES: &[&str] = &["web", "ios", "android", "macos", "windows"];

pub fn validate_event(event: &Value) -> bool {
    if event.get("schema_version").and_then(|v| v.as_i64()) != Some(1) {
        return false;
    }
    if !non_empty_string(event.get("session_id")) {
        return false;
    }
    if !non_empty_string(event.get("anonymous_id")) {
        return false;
    }
    if event.get("timestamp").and_then(|v| v.as_f64()).is_none() {
        return false;
    }
    if !validate_sdk(event.get("sdk")) {
        return false;
    }
    if !validate_platform(event.get("platform")) {
        return false;
    }
    if !validate_event_payload(event.get("event")) {
        return false;
    }
    true
}

fn non_empty_string(value: Option<&Value>) -> bool {
    value
        .and_then(|v| v.as_str())
        .map(|text| !text.is_empty())
        .unwrap_or(false)
}

fn validate_sdk(value: Option<&Value>) -> bool {
    let Some(sdk) = value else {
        return false;
    };
    non_empty_string(sdk.get("name")) && non_empty_string(sdk.get("version"))
}

fn validate_platform(value: Option<&Value>) -> bool {
    let Some(platform) = value else {
        return false;
    };
    let platform_type = platform.get("type").and_then(|v| v.as_str());
    let Some(platform_type) = platform_type else {
        return false;
    };
    if !PLATFORM_TYPES.contains(&platform_type) {
        return false;
    }
    non_empty_string(platform.get("os_version"))
        && non_empty_string(platform.get("app_version"))
}

fn validate_event_payload(value: Option<&Value>) -> bool {
    let Some(event) = value else {
        return false;
    };
    let event_type = event.get("type").and_then(|v| v.as_str());
    let Some(event_type) = event_type else {
        return false;
    };
    EVENT_TYPES.contains(&event_type)
}
