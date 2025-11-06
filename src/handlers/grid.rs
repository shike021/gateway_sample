//! 网格业务处理模块
//!
//! 包含网格相关的所有业务逻辑。

use axum::{http::StatusCode, response::Json};
use std::sync::RwLock;

// 从路由模块导入需要的类型
use crate::routes::grid::{
    ApiResponse, AppState, CreateGridItem, GridItem, GridItemResponse, UpdateGridItem,
};

/// 处理获取所有网格项的业务逻辑
pub async fn handle_list(state: AppState) -> Json<ApiResponse<Vec<GridItemResponse>>> {
    let grid_items = state.grid_items.read().unwrap();

    // 打印调试信息
    println!("Grid items count: {}", grid_items.len());
    for item in grid_items.iter() {
        println!(
            "Item: id={}, name={}, description={}, x={}, y={}",
            item.id, item.name, item.description, item.x, item.y
        );
    }

    // 转换为响应格式
    let response_items: Vec<GridItemResponse> = grid_items
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
        message: "成功获取网格项列表".to_string(),
    })
}

/// 处理获取特定网格项的业务逻辑
pub async fn handle_get(state: AppState, id: u64) -> Json<ApiResponse<GridItemResponse>> {
    let grid_items = state.grid_items.read().unwrap();
    match grid_items.iter().find(|item| item.id == id) {
        Some(item) => {
            let response_item = GridItemResponse {
                id: item.id,
                name: item.name.clone(),
                description: item.description.clone(),
                x: item.x,
                y: item.y,
            };
            Json(ApiResponse {
                success: true,
                data: Some(response_item),
                message: "成功获取网格项".to_string(),
            })
        }
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: "未找到指定的网格项".to_string(),
        }),
    }
}

/// 处理创建新网格项的业务逻辑
pub async fn handle_create(
    state: AppState,
    payload: CreateGridItem,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    // 打印调试信息
    println!(
        "Received payload: name={}, description={}, x={}, y={}",
        payload.name, payload.description, payload.x, payload.y
    );

    let mut grid_items = state.grid_items.write().unwrap();

    // 生成新的 ID（在实际应用中可能需要更复杂的 ID 生成策略）
    let new_id = grid_items.iter().map(|item| item.id).max().unwrap_or(0) + 1;

    let new_item = GridItem {
        id: new_id,
        name: payload.name.clone(),
        description: payload.description.clone(),
        x: payload.x,
        y: payload.y,
    };

    // 打印创建的项
    println!(
        "Creating item: id={}, name={}, description={}, x={}, y={}",
        new_item.id, new_item.name, new_item.description, new_item.x, new_item.y
    );

    grid_items.push(new_item.clone());

    // 转换为响应格式
    let response_item = GridItemResponse {
        id: new_item.id,
        name: new_item.name,
        description: new_item.description,
        x: new_item.x,
        y: new_item.y,
    };

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            data: Some(response_item),
            message: "成功创建网格项".to_string(),
        }),
    )
}

/// 处理更新网格项的业务逻辑
pub async fn handle_update(
    state: AppState,
    id: u64,
    payload: UpdateGridItem,
) -> (StatusCode, Json<ApiResponse<GridItemResponse>>) {
    let mut grid_items = state.grid_items.write().unwrap();

    // 查找要更新的项
    if let Some(item) = grid_items.iter_mut().find(|item| item.id == id) {
        // 更新字段
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

        // 转换为响应格式
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
                message: "成功更新网格项".to_string(),
            }),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: "未找到指定的网格项".to_string(),
            }),
        )
    }
}

/// 处理删除网格项的业务逻辑
pub async fn handle_delete(state: AppState, id: u64) -> Json<ApiResponse<()>> {
    let mut grid_items = state.grid_items.write().unwrap();

    // 查找要删除的项的索引
    let index = grid_items.iter().position(|item| item.id == id);

    match index {
        Some(idx) => {
            grid_items.remove(idx);
            Json(ApiResponse {
                success: true,
                data: None,
                message: "成功删除网格项".to_string(),
            })
        }
        None => Json(ApiResponse {
            success: false,
            data: None,
            message: "未找到指定的网格项".to_string(),
        }),
    }
}
