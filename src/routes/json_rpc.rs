//! JSON-RPC 路由处理
//!
//! 处理 JSON-RPC 相关的请求，将请求转发给业务处理模块。

use axum::{extract::State, response::Json};
use jsonrpc_core::{IoHandler, Value};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::handlers::user_info as rpc;
use crate::server::AppState;

/// 定义 JSON-RPC 相关的路由
pub fn rpc_routes() -> Router<AppState> {
    Router::new().route("/rpc", post(handle_rpc))
}

/// 处理 JSON-RPC 请求
pub async fn handle_rpc(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let mut io = IoHandler::new();
    
    // 注册 RPC 方法
    io.add_method("get_user_info", rpc::get_user_info);
    io.add_method("update_user_info", rpc::update_user_info);
    io.add_method("verify_credentials", rpc::verify_credentials);
    
    // 处理请求
    let response = io.handle_request(&payload.to_string()).await;
    Json(response.unwrap_or(json!({"error": "Invalid request"})))
}