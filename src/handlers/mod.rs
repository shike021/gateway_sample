//! 业务处理模块
//!
//! 包含所有的业务逻辑处理函数。

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use std::sync::{Arc, RwLock};

use crate::routes::grid::{GridItem, CreateGridItem, UpdateGridItem, GridItemResponse};
use crate::server::AppState;

// 响应结构体
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

/// 获取所有网格项
pub async fn list(State(state): State<AppState>) -> Json<ApiResponse<Vec<GridItemResponse>>> {
    let items = state.grid_items.read().unwrap();
    let response_items: Vec<GridItemResponse> = items
        .iter()
        .map(|item| GridItemResponse {
            id: item.id,
            name: item.name.clone(),
            description: item.description.clone(),
            x: item.x,
            y: item.y,
        })
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(response_items),
        message: "获取网格项列表成功".to_string(),
    })
}

/// 获取特定网格项
pub async fn get_by_id(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<GridItemResponse>> {
    let items = state.grid_items.read().unwrap();
    let item = items.iter().find(|item| item.id == id);

    match item {
        Some(item) => Json(ApiResponse {
            success: true,
            data: Some(GridItemResponse {
                id: item.id,
                name: item.name.clone(),
                description: item.description.clone(),
                x: item.x,
                y: item.y,
            }),
            message: "获取网格项成功".to_string(),
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: "未找到指定的网格项".to_string(),
        }),
    }
}

/// 创建新的网格项
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    let mut items = state.grid_items.write().unwrap();

    // 生成新的 ID（简单实现）
    let new_id = items.iter().map(|item| item.id).max().unwrap_or(0) + 1;

    let new_item = GridItem {
        id: new_id,
        name: payload.name,
        description: payload.description,
        x: payload.x,
        y: payload.y,
    };

    let response_item = GridItemResponse {
        id: new_item.id,
        name: new_item.name.clone(),
        description: new_item.description.clone(),
        x: new_item.x,
        y: new_item.y,
    };

    items.push(new_item);

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            data: Some(response_item),
            message: "创建网格项成功".to_string(),
        }),
    )
}

/// 更新网格项
pub async fn update(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    let mut items = state.grid_items.write().unwrap();
    let item = items.iter_mut().find(|item| item.id == id);

    match item {
        Some(item) => {
            if let Some(name) = payload.name {
                item.name = name;
            }
            if let Some(description) = payload.description {
                item.description = description;
            }
            if let Some(x) = payload.x {
                item.x = x;
            }
            if let Some(y) = payload.y {
                item.y = y;
            }

            let response_item = GridItemResponse {
                id: item.id,
                name: item.name.clone(),
                description: item.description.clone(),
                x: item.x,
                y: item.y,
            };

            (
                StatusCode::OK,
                Json(ApiResponse {
                    success: true,
                    data: Some(response_item),
                    message: "更新网格项成功".to_string(),
                }),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "未找到指定的网格项".to_string(),
            }),
        ),
    }
}

/// 删除网格项
pub async fn delete_by_id(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<()>> {
    let mut items = state.grid_items.write().unwrap();
    let initial_len = items.len();
    items.retain(|item| item.id != id);

    if items.len() < initial_len {
        Json(ApiResponse {
            success: true,
            data: Some(()),
            message: "删除网格项成功".to_string(),
        })
    } else {
        Json(ApiResponse {
            success: false,
            data: None,
            message: "未找到指定的网格项".to_string(),
        })
    }
}
