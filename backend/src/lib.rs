mod config;
mod db;
mod error;
mod handlers;
mod service;
mod validate;

use std::sync::{Arc, Mutex};

use axum::{routing::get, routing::post, Router};
use rusqlite::Connection;

pub use config::AppConfig;
pub use handlers::AppState;

pub const TEST_PROJECT_KEY: &str = "test-project-key";

pub fn app(config: AppConfig, db: Connection) -> Router {
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        config,
    };
    Router::new()
        .route("/v1/batches", post(handlers::post_batches))
        .route("/v1/elements", get(handlers::get_elements))
        .with_state(state)
}

pub fn test_app() -> Router {
    let db = db::open_in_memory().expect("open in-memory sqlite");
    app(AppConfig::default(), db)
}
