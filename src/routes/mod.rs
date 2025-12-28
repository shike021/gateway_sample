//! Route module entry point
//!
//! Integrates all route modules.
//!
//! Copyright © 2024 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use axum::Router;

pub mod grid;
pub mod json_rpc;

/// Integrate all routes
pub fn app_routes() -> Router<crate::server::AppState> {
    Router::new()
        .merge(grid_routes())
        .merge(json_rpc::rpc_routes())
}

/// Get grid routes
fn grid_routes() -> Router<crate::server::AppState> {
    grid::grid_routes()
}
