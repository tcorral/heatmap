use std::sync::{Arc, Mutex};

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use bytes::Bytes;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::config::AppConfig;
use crate::db::ElementsFilter;
use crate::error::ApiError;
use crate::service;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub config: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct BatchBody {
    pub batch_id: String,
    pub events: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct ElementsQuery {
    pub platform: String,
    pub screen: String,
    pub layout_version: Option<String>,
    pub app_version: String,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

pub async fn post_batches(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let project_key = headers
        .get("X-Project-Key")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if !state.config.is_valid_project_key(project_key) {
        return Err(ApiError::Unauthorized);
    }

    let parsed: BatchBody = parse_batch_body(&body)?;
    if parsed.batch_id.is_empty() || parsed.events.is_empty() {
        return Err(ApiError::BadRequest);
    }

    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let result = service::ingest_batch(&conn, &parsed.batch_id, &parsed.events)
        .map_err(|_| ApiError::Internal)?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({
            "batch_id": result.batch_id,
            "accepted_count": result.accepted_count,
            "rejected_count": result.rejected_count,
        })),
    ))
}

pub async fn get_elements(
    State(state): State<AppState>,
    Query(query): Query<ElementsQuery>,
) -> Result<Json<Value>, ApiError> {
    let filter = ElementsFilter {
        platform: &query.platform,
        screen: &query.screen,
        layout_version: query.layout_version.as_deref(),
        app_version: &query.app_version,
        from: query.from,
        to: query.to,
    };
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let response = service::query_elements(&conn, &filter).map_err(|_| ApiError::Internal)?;
    let elements: Vec<Value> = response
        .elements
        .into_iter()
        .map(|row| {
            json!({
                "element_id": row.element_id,
                "click_count": row.click_count,
                "unique_sessions": row.unique_sessions,
            })
        })
        .collect();
    Ok(Json(json!({
        "elements": elements,
        "unique_sessions": response.unique_sessions,
    })))
}

fn parse_batch_body(body: &Bytes) -> Result<BatchBody, ApiError> {
    serde_json::from_slice(body).map_err(|_| ApiError::BadRequest)
}
