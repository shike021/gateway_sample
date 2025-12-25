//! 路由模块入口

use axum::Router;
use crate::handlers;

pub mod grid;

/// 整合所有路由
pub fn app_routes() -> Router<crate::server::AppState> {
    Router::new().merge(grid_routes())
}

/// 获取网格路由
fn grid_routes() -> Router<crate::server::AppState> {
    grid::grid_routes()
}
