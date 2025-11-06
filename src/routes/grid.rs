//! 网格路由处理
//!
//! 处理网格相关的 HTTP 请求，将请求转发给业务处理模块。

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

// 引入业务处理模块
use crate::handlers;

// 网格项数据结构（内部存储）
#[derive(Clone, Debug)]
pub struct GridItem {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub x: i32,
    pub y: i32,
}

// 网格项数据结构（API 响应）
#[derive(Serialize)]
pub struct GridItemResponse {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub x: i32,
    pub y: i32,
}

// 创建网格项的请求体
#[derive(Deserialize)]
pub struct CreateGridItem {
    pub name: String,
    pub description: String,
    pub x: i32,
    pub y: i32,
}

// 更新网格项的请求体
#[derive(Deserialize)]
pub struct UpdateGridItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub grid_items: Arc<RwLock<Vec<GridItem>>>,
}

// 响应结构体
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

/// 获取所有网格项
pub async fn list(State(state): State<AppState>) -> Json<ApiResponse<Vec<GridItemResponse>>> {
    handlers::grid::handle_list(state).await
}

/// 获取特定网格项
pub async fn get(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<GridItemResponse>> {
    handlers::grid::handle_get(state, id).await
}

/// 创建新的网格项
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    handlers::grid::handle_create(state, payload).await
}

/// 更新网格项
pub async fn update(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    handlers::grid::handle_update(state, id, payload).await
}

/// 删除网格项
pub async fn delete(Path(id): Path<u64>, State(state): State<AppState>) -> Json<ApiResponse<()>> {
    handlers::grid::handle_delete(state, id).await
}
