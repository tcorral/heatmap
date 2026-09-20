use serde_json::{Map, Value};

const EVENT_TYPES: &[&str] = &[
    "session_start",
    "session_end",
    "screen_view",
    "click",
    "scroll",
];

const PLATFORM_TYPES: &[&str] = &["web", "ios", "android", "macos", "windows"];

pub fn validate_event(event: &Value) -> bool {
    schema_version_is_one(event.get("schema_version"))
        && non_empty_string(event.get("session_id"))
        && non_empty_string(event.get("anonymous_id"))
        && event.get("timestamp").and_then(Value::as_f64).is_some()
        && optional_non_empty_string(event.get("user_id"))
        && optional_non_empty_string(event.get("window_id"))
        && validate_sdk(event.get("sdk"))
        && validate_platform(event.get("platform"))
        && validate_screen(event.get("screen"))
        && validate_event_payload(event.get("event"))
        && validate_position(event.get("position"))
        && validate_viewport(event.get("viewport"))
}

fn schema_version_is_one(value: Option<&Value>) -> bool {
    matches!(value, Some(v) if v.as_i64() == Some(1) || v.as_u64() == Some(1))
}

fn non_empty_string(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.is_empty())
}

fn optional_non_empty_string(value: Option<&Value>) -> bool {
    match value {
        None => true,
        Some(v) => non_empty_string(Some(v)),
    }
}

fn object_only_keys(object: &Map<String, Value>, allowed: &[&str]) -> bool {
    object.keys().all(|key| allowed.contains(&key.as_str()))
}

fn positive_json_integer(value: Option<&Value>) -> bool {
    match value {
        Some(v) if v.as_i64().is_some_and(|n| n > 0) => true,
        Some(v) if v.as_u64().is_some_and(|n| n > 0) => true,
        _ => false,
    }
}

fn validate_sdk(value: Option<&Value>) -> bool {
    let Some(sdk) = value.and_then(Value::as_object) else {
        return false;
    };
    if !object_only_keys(sdk, &["name", "version"]) {
        return false;
    }
    non_empty_string(sdk.get("name")) && non_empty_string(sdk.get("version"))
}

fn validate_platform(value: Option<&Value>) -> bool {
    let Some(platform) = value.and_then(Value::as_object) else {
        return false;
    };
    if !object_only_keys(platform, &["type", "os_version", "app_version"]) {
        return false;
    }
    let platform_type = platform.get("type").and_then(Value::as_str);
    let Some(platform_type) = platform_type else {
        return false;
    };
    if !PLATFORM_TYPES.contains(&platform_type) {
        return false;
    }
    non_empty_string(platform.get("os_version")) && non_empty_string(platform.get("app_version"))
}

fn validate_screen(value: Option<&Value>) -> bool {
    let Some(screen) = value else {
        return true;
    };
    let Some(screen) = screen.as_object() else {
        return false;
    };
    if !object_only_keys(screen, &["id", "layout_version"]) {
        return false;
    }
    non_empty_string(screen.get("id")) && optional_non_empty_string(screen.get("layout_version"))
}

fn validate_event_payload(value: Option<&Value>) -> bool {
    let Some(event) = value.and_then(Value::as_object) else {
        return false;
    };
    if !object_only_keys(event, &["type", "element_id"]) {
        return false;
    }
    let event_type = event.get("type").and_then(Value::as_str);
    let Some(event_type) = event_type else {
        return false;
    };
    if !EVENT_TYPES.contains(&event_type) {
        return false;
    }
    match event.get("element_id") {
        None => true,
        Some(element_id) if event_type == "click" => non_empty_string(Some(element_id)),
        Some(_) => false,
    }
}

fn validate_position(value: Option<&Value>) -> bool {
    let Some(position) = value else {
        return true;
    };
    let Some(position) = position.as_object() else {
        return false;
    };
    if !object_only_keys(position, &["x", "y"]) {
        return false;
    }
    let Some(x) = position.get("x").and_then(Value::as_f64) else {
        return false;
    };
    let Some(y) = position.get("y").and_then(Value::as_f64) else {
        return false;
    };
    (0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y)
}

fn validate_viewport(value: Option<&Value>) -> bool {
    let Some(viewport) = value else {
        return true;
    };
    let Some(viewport) = viewport.as_object() else {
        return false;
    };
    if !object_only_keys(viewport, &["width", "height"]) {
        return false;
    }
    positive_json_integer(viewport.get("width")) && positive_json_integer(viewport.get("height"))
}

#[cfg(test)]
mod tests {
    use super::validate_event;
    use serde_json::{json, Value};

    fn base_event(event_type: &str) -> Value {
        json!({
            "schema_version": 1,
            "session_id": "sess",
            "anonymous_id": "anon",
            "timestamp": 1.0,
            "sdk": { "name": "heatmap-web", "version": "0.1.0" },
            "platform": {
                "type": "web",
                "os_version": "macOS",
                "app_version": "1.0.0"
            },
            "event": { "type": event_type }
        })
    }

    #[test]
    fn accepts_valid_click_scroll_and_screen_view() {
        assert!(validate_event(&base_event("click")));
        assert!(validate_event(&base_event("scroll")));
        assert!(validate_event(&base_event("screen_view")));
    }

    #[test]
    fn rejects_viewport_width_zero() {
        let mut event = base_event("click");
        event["viewport"] = json!({ "width": 0, "height": 720 });
        assert!(!validate_event(&event));
    }

    #[test]
    fn rejects_viewport_non_integer_width() {
        let mut event = base_event("click");
        event["viewport"] = json!({ "width": 12.5, "height": 720 });
        assert!(!validate_event(&event));
    }

    #[test]
    fn rejects_screen_with_empty_id() {
        let mut event = base_event("click");
        event["screen"] = json!({ "id": "" });
        assert!(!validate_event(&event));
    }

    #[test]
    fn rejects_screen_missing_id() {
        let mut event = base_event("click");
        event["screen"] = json!({});
        assert!(!validate_event(&event));
    }

    #[test]
    fn accepts_omitted_screen() {
        assert!(validate_event(&base_event("click")));
    }

    #[test]
    fn rejects_empty_layout_version_on_screen() {
        let mut event = base_event("click");
        event["screen"] = json!({ "id": "Checkout", "layout_version": "" });
        assert!(!validate_event(&event));
    }

    #[test]
    fn rejects_empty_user_id_and_window_id() {
        let mut with_user = base_event("click");
        with_user["user_id"] = json!("");
        assert!(!validate_event(&with_user));

        let mut with_window = base_event("click");
        with_window["window_id"] = json!("");
        assert!(!validate_event(&with_window));
    }

    #[test]
    fn rejects_element_id_on_scroll_and_screen_view() {
        let mut scroll = base_event("scroll");
        scroll["event"] = json!({ "type": "scroll", "element_id": "btn" });
        assert!(!validate_event(&scroll));

        let mut view = base_event("screen_view");
        view["event"] = json!({ "type": "screen_view", "element_id": "btn" });
        assert!(!validate_event(&view));
    }

    #[test]
    fn accepts_click_with_element_id_rejects_empty_element_id() {
        let mut with_id = base_event("click");
        with_id["event"] = json!({ "type": "click", "element_id": "checkout.pay" });
        assert!(validate_event(&with_id));

        let mut empty_id = base_event("click");
        empty_id["event"] = json!({ "type": "click", "element_id": "" });
        assert!(!validate_event(&empty_id));
    }

    #[test]
    fn accepts_unknown_top_level_field() {
        let mut event = base_event("click");
        event["extra"] = json!(true);
        assert!(validate_event(&event));
    }

    #[test]
    fn rejects_extra_field_on_sdk_object() {
        let mut event = base_event("click");
        event["sdk"] = json!({
            "name": "heatmap-web",
            "version": "0.1.0",
            "extra": true
        });
        assert!(!validate_event(&event));
    }
}
