//! Route module entry point
//!
//! Integrates all route modules.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use axum::Router;

pub mod grid;
pub mod health;
pub mod json_rpc;

/// Integrate all routes
pub fn app_routes() -> Router<crate::server::AppState> {
    Router::new()
        .merge(health_routes())
        .merge(grid_routes())
        .merge(json_rpc::rpc_routes())
}

/// Get health routes
fn health_routes() -> Router<crate::server::AppState> {
    Router::new().route(
        "/health",
        axum::routing::get(crate::routes::health::health_check_handler),
    )
}

/// Get grid routes
fn grid_routes() -> Router<crate::server::AppState> {
    grid::grid_routes()
}
