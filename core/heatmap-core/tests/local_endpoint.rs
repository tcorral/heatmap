use heatmap_core::{HeatmapConfig, HeatmapSdk, InitError};

fn base_config(endpoint: &str) -> HeatmapConfig {
    HeatmapConfig {
        project_key: "pk_test".into(),
        endpoint: endpoint.into(),
        session_timeout_ms: HeatmapConfig::DEFAULT_SESSION_TIMEOUT_MS,
        sample_rate: HeatmapConfig::DEFAULT_SAMPLE_RATE,
        exclude_elements: vec![],
        exclude_screens: vec![],
        mask_elements: vec![],
    }
}

#[test]
fn should_allow_http_localhost_endpoint_for_local_e2e() {
    assert!(HeatmapSdk::init(base_config("http://127.0.0.1:8080")).is_ok());
    assert!(HeatmapSdk::init(base_config("http://localhost:8080")).is_ok());
}

#[test]
fn should_reject_http_non_local_endpoint() {
    match HeatmapSdk::init(base_config("http://ingest.example.com")) {
        Err(err) => assert!(matches!(err, InitError::InsecureEndpoint)),
        Ok(_) => panic!("expected remote HTTP endpoint to fail init"),
    }
}

#[test]
fn should_reject_http_loopback_prefix_lookalikes() {
    for endpoint in [
        "http://127.0.0.1.evil.com",
        "http://localhost.evil.com",
        "http://127.0.0.1.example",
    ] {
        match HeatmapSdk::init(base_config(endpoint)) {
            Err(err) => assert!(matches!(err, InitError::InsecureEndpoint)),
            Ok(_) => panic!("expected {endpoint} to fail init"),
        }
    }
}
