use std::env;

use axum::serve;
use heatmap_backend::{app, open_db, AppConfig, TEST_PROJECT_KEY};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".into());
    let db_path = env::var("DATABASE_PATH").unwrap_or_else(|_| "heatmap-backend.db".into());
    let expose_test_events = env::var("HEATMAP_EXPOSE_TEST_EVENTS")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let project_key = env::var("HEATMAP_PROJECT_KEY").unwrap_or_else(|_| TEST_PROJECT_KEY.into());

    let db = open_db(&db_path).expect("open sqlite database");
    let config = AppConfig::with_project_keys([project_key]).with_test_events(expose_test_events);
    let router = app(config, db);
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|err| panic!("bind {addr}: {err}"));
    eprintln!("heatmap-backend listening on http://{addr}");
    serve(listener, router)
        .await
        .expect("serve heatmap-backend");
}
