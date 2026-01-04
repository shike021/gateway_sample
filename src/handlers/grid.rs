//! Grid business logic module
//!
//! Contains business logic processing functions related to grid.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Serialize;

use crate::routes::rest::{CreateGridItem, GridItem, GridItemResponse, UpdateGridItem};
use crate::server::AppState;

// Response structure
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

#[allow(dead_code)]
/// 获取所有网格项
pub async fn list(State(state): State<AppState>) -> Json<ApiResponse<Vec<GridItemResponse>>> {
    let items = state
        .grid_items
        .read()
        .expect("Failed to acquire read lock on grid_items");
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
        message: "Successfully retrieved grid item list".to_string(),
    })
}

#[allow(dead_code)]
/// 获取特定网格项
pub async fn get_by_id(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<GridItemResponse>> {
    let items = state
        .grid_items
        .read()
        .expect("Failed to acquire read lock on grid_items");
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
            message: "Successfully retrieved grid item".to_string(),
        }),
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: "Specified grid item not found".to_string(),
        }),
    }
}

#[allow(dead_code)]
/// Create new grid item
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    let mut items = state
        .grid_items
        .write()
        .expect("Failed to acquire write lock on grid_items");

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
            message: "Grid item created successfully".to_string(),
        }),
    )
}

#[allow(dead_code)]
/// Update grid item
pub async fn update(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateGridItem>,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    let mut items = state
        .grid_items
        .write()
        .expect("Failed to acquire write lock on grid_items");
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
                    message: "Successfully updated grid item".to_string(),
                }),
            )
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "Specified grid item not found".to_string(),
            }),
        ),
    }
}

#[allow(dead_code)]
/// Delete grid item
pub async fn delete_by_id(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Json<ApiResponse<()>> {
    let mut items = state
        .grid_items
        .write()
        .expect("Failed to acquire write lock on grid_items");
    let initial_len = items.len();
    items.retain(|item| item.id != id);

    if items.len() < initial_len {
        Json(ApiResponse {
            success: true,
            data: Some(()),
            message: "Successfully deleted grid item".to_string(),
        })
    } else {
        Json(ApiResponse {
            success: false,
            data: None,
            message: "Grid item not found".to_string(),
        })
    }
}
