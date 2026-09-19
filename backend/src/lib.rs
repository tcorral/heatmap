mod config;
pub mod db;
mod error;
mod handlers;
mod rate_limit;
mod service;
mod validate;

use std::sync::{Arc, Mutex};

use axum::{routing::get, routing::post, Router};
use rusqlite::Connection;
use tower_http::cors::CorsLayer;

pub use config::AppConfig;
pub use handlers::AppState;

pub const TEST_PROJECT_KEY: &str = "test-project-key";

pub fn app(config: AppConfig, db: Connection) -> Router {
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        config: config.clone(),
        rate_limiter: Arc::new(Mutex::new(rate_limit::RateLimiter::new())),
    };
    let mut router = Router::new()
        .route("/v1/batches", post(handlers::post_batches))
        .route("/v1/elements", get(handlers::get_elements))
        .route("/v1/heatmap", get(handlers::get_heatmap))
        .route("/v1/screens", get(handlers::get_screens));
    if config.expose_test_events() {
        router = router.route("/v1/events", get(handlers::get_events));
    }
    router.layer(CorsLayer::permissive()).with_state(state)
}

pub fn open_db(path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(path)?;
    db::init_schema(&conn)?;
    Ok(conn)
}

pub fn test_app() -> Router {
    let db = db::open_in_memory().expect("open in-memory sqlite");
    app(AppConfig::default(), db)
}
