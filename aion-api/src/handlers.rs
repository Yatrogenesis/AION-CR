use axum::{Json, response::Json as ResponseJson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health_handler() -> ResponseJson<HealthResponse> {
    ResponseJson(HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    })
}