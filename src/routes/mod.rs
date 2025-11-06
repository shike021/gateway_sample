//! 路由模块
//!
//! 定义应用程序的所有路由。

use axum::{
    routing::{delete, get, post, put},
    Router,
};

// 将 grid 模块设为公有
pub mod grid;

/// 创建网格相关的路由
pub fn grid_routes() -> Router<grid::AppState> {
    Router::new()
        .route("/grid", get(grid::list).post(grid::create))
        .route(
            "/grid/{id}",
            get(grid::get).put(grid::update).delete(grid::delete),
        )
}
