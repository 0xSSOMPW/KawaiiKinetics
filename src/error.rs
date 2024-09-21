use std::sync::Arc;

use aniscraper::error::AniRustError;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Define your error enum
#[derive(Error, Debug)]
pub enum KawaiiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("AniRustError: {0}")]
    AniRustError(#[from] AniRustError),

    #[error("AniRustError: {0}")]
    AniRustRefError(String),

    #[error("CacheError: {0}")]
    CacheError(String),
}

// Implement From for Arc<KawaiiError>
impl From<Arc<KawaiiError>> for KawaiiError {
    fn from(arc: Arc<KawaiiError>) -> Self {
        // You can choose to clone the error or match it to create the appropriate variant
        match &*arc {
            KawaiiError::InternalServerError => KawaiiError::InternalServerError,
            KawaiiError::NotFound => KawaiiError::NotFound,
            KawaiiError::AniRustError(e) => KawaiiError::AniRustRefError(e.to_string()),
            KawaiiError::AniRustRefError(e) => KawaiiError::AniRustRefError(e.to_string()),
            KawaiiError::CacheError(s) => KawaiiError::CacheError(s.clone()),
        }
    }
}

// Implement IntoResponse for KawaiiError
impl IntoResponse for KawaiiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            KawaiiError::InternalServerError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            KawaiiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            KawaiiError::AniRustError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            KawaiiError::AniRustRefError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            KawaiiError::CacheError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string()
        });

        (status_code, Json(body)).into_response()
    }
}
