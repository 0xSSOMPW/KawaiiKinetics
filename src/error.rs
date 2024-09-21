use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

use aniscraper::error::AniRustError;

#[derive(Error, Debug)]
pub enum KawaiiError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found")]
    NotFound,

    #[error("AniRustError: {0}")]
    AniRustError(#[from] AniRustError),
}

impl IntoResponse for KawaiiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            KawaiiError::InternalServerError => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            KawaiiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            KawaiiError::AniRustError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string()
        });

        (status_code, Json(body)).into_response()
    }
}
