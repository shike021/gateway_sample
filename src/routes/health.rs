//! Health check module
//!
//! Contains health check endpoint for server monitoring.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use crate::errors::AppError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Health status of the server
    pub status: String,
    /// Message describing the health status
    pub message: String,
    /// Unix timestamp of the health check
    pub timestamp: u64,
}

/// Health check handler
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse),
    )
)]
pub async fn health_check_handler() -> Result<impl IntoResponse, AppError> {
    // In a real application, you might check database connectivity, external services, etc.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let response = HealthResponse {
        status: "healthy".to_string(),
        message: "Server is running normally".to_string(),
        timestamp,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check_handler().await.unwrap();
        let (parts, _) = response.into_response().into_parts();
        assert_eq!(parts.status, StatusCode::OK);
    }
}
