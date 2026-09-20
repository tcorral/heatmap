use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    Unauthorized,
    BadRequest,
    NotFound,
    PayloadTooLarge,
    TooManyRequests { retry_after_secs: u64 },
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TooManyRequests { retry_after_secs } => {
                let status = StatusCode::TOO_MANY_REQUESTS;
                (
                    status,
                    [(header::RETRY_AFTER, retry_after_secs.to_string())],
                    Json(json!({ "error": status.canonical_reason() })),
                )
                    .into_response()
            }
            other => {
                let status = match other {
                    ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
                    ApiError::BadRequest => StatusCode::BAD_REQUEST,
                    ApiError::NotFound => StatusCode::NOT_FOUND,
                    ApiError::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
                    ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
                    ApiError::TooManyRequests { .. } => unreachable!(),
                };
                (status, Json(json!({ "error": status.canonical_reason() }))).into_response()
            }
        }
    }
}
