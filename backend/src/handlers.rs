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
use crate::db::{self, ElementsFilter, HeatmapFilter};
use crate::error::ApiError;
use crate::rate_limit::RateLimiter;
use crate::service;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub config: AppConfig,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
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

#[derive(Debug, Deserialize)]
pub struct HeatmapQuery {
    pub platform: String,
    pub screen: String,
    pub layout_version: Option<String>,
    pub app_version: String,
    pub from: Option<f64>,
    pub to: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct StillsQuery {
    pub platform: String,
    pub screen: String,
    pub layout_version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostStillBody {
    pub platform: String,
    pub screen: String,
    pub layout_version: Option<String>,
    pub content_type: String,
    pub image_base64: String,
}

pub async fn post_batches(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let project_key = project_key_from_headers(&headers);
    authorize_project_key(&state.config, project_key)?;
    let parsed = validate_batch_request(&body, &state.config)?;
    enforce_rate_limit_if_new(&state, project_key, &parsed.batch_id)?;
    ingest_and_accept(&state, &parsed)
}

fn authorize_project_key(config: &AppConfig, project_key: &str) -> Result<(), ApiError> {
    if config.is_valid_project_key(project_key) {
        Ok(())
    } else {
        Err(ApiError::Unauthorized)
    }
}

fn validate_batch_request(body: &Bytes, config: &AppConfig) -> Result<BatchBody, ApiError> {
    reject_if_oversize_bytes(body, config.max_batch_bytes())?;
    let parsed: BatchBody = parse_batch_body(body)?;
    if parsed.batch_id.is_empty() || parsed.events.is_empty() {
        return Err(ApiError::BadRequest);
    }
    reject_if_oversize_events(parsed.events.len(), config.max_events_per_batch())?;
    Ok(parsed)
}

fn enforce_rate_limit_if_new(
    state: &AppState,
    project_key: &str,
    batch_id: &str,
) -> Result<(), ApiError> {
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let is_new = db::find_batch(&conn, batch_id)
        .map_err(|_| ApiError::Internal)?
        .is_none();
    drop(conn);
    if is_new && !acquire_rate_slot(state, project_key) {
        return Err(ApiError::TooManyRequests);
    }
    Ok(())
}

fn ingest_and_accept(
    state: &AppState,
    parsed: &BatchBody,
) -> Result<(StatusCode, Json<Value>), ApiError> {
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
    Ok(Json(elements_response_json(response)))
}

fn elements_response_json(response: service::ElementsResponse) -> Value {
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
    json!({
        "elements": elements,
        "unique_sessions": response.unique_sessions,
    })
}

pub async fn get_heatmap(
    State(state): State<AppState>,
    Query(query): Query<HeatmapQuery>,
) -> Result<Json<Value>, ApiError> {
    let filter = HeatmapFilter {
        platform: &query.platform,
        screen: &query.screen,
        layout_version: query.layout_version.as_deref(),
        app_version: &query.app_version,
        from: query.from,
        to: query.to,
    };
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let response = service::query_heatmap(&conn, &filter).map_err(|_| ApiError::Internal)?;
    let bins: Vec<Value> = response
        .bins
        .into_iter()
        .map(|row| {
            json!({
                "bin_x": row.bin_x,
                "bin_y": row.bin_y,
                "count": row.count,
            })
        })
        .collect();
    Ok(Json(json!({ "bins": bins })))
}

pub async fn get_stills(
    State(state): State<AppState>,
    Query(query): Query<StillsQuery>,
) -> Result<Json<Value>, ApiError> {
    let layout = query.layout_version.as_deref().unwrap_or("");
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let still = service::query_still(&conn, &query.platform, &query.screen, layout)
        .map_err(|_| ApiError::Internal)?;
    let Some(still) = still else {
        return Err(ApiError::NotFound);
    };
    Ok(Json(json!({
        "content_type": still.content_type,
        "image_base64": still.image_base64,
    })))
}

pub async fn post_stills(
    State(state): State<AppState>,
    Json(body): Json<PostStillBody>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    if body.platform.is_empty() || body.screen.is_empty() || body.image_base64.is_empty() {
        return Err(ApiError::BadRequest);
    }
    let layout = body.layout_version.as_deref().unwrap_or("");
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    service::upsert_still(
        &conn,
        &body.platform,
        &body.screen,
        layout,
        &body.content_type,
        &body.image_base64,
    )
    .map_err(|_| ApiError::Internal)?;
    Ok((StatusCode::CREATED, Json(json!({ "status": "created" }))))
}

pub async fn get_screens(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let response = service::query_screens(&conn).map_err(|_| ApiError::Internal)?;
    let screens: Vec<Value> = response
        .screens
        .into_iter()
        .map(|row| {
            json!({
                "platform": row.platform,
                "id": row.id,
                "layout_version": row.layout_version,
            })
        })
        .collect();
    Ok(Json(json!({ "screens": screens })))
}

pub async fn get_events(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    if !state.config.expose_test_events() {
        return Err(ApiError::NotFound);
    }
    let conn = state.db.lock().map_err(|_| ApiError::Internal)?;
    let events = db::list_raw_events(&conn).map_err(|_| ApiError::Internal)?;
    Ok(Json(json!({ "events": events })))
}

fn project_key_from_headers(headers: &HeaderMap) -> &str {
    headers
        .get("X-Project-Key")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
}

fn reject_if_oversize_bytes(body: &Bytes, max_bytes: usize) -> Result<(), ApiError> {
    if body.len() > max_bytes {
        return Err(ApiError::PayloadTooLarge);
    }
    Ok(())
}

fn reject_if_oversize_events(count: usize, max_events: usize) -> Result<(), ApiError> {
    if count > max_events {
        return Err(ApiError::PayloadTooLarge);
    }
    Ok(())
}

fn acquire_rate_slot(state: &AppState, project_key: &str) -> bool {
    let mut limiter = state.rate_limiter.lock().expect("rate limiter lock");
    limiter.allow(project_key, state.config.batches_per_minute())
}

fn parse_batch_body(body: &Bytes) -> Result<BatchBody, ApiError> {
    serde_json::from_slice(body).map_err(|_| ApiError::BadRequest)
}
