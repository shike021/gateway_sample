//! JSON-RPC 路由处理
//!
//! 处理 JSON-RPC 相关的请求，将请求转发给业务处理模块。

use axum::{extract::State, response::Json, routing::post, Router};
use jsonrpc_core::{IoHandler, Value};
use serde_json::json;

use crate::handlers::user_info as rpc;
use crate::server::AppState;

/// 定义 JSON-RPC 相关的路由
pub fn rpc_routes() -> Router<AppState> {
    Router::new().route("/jsonrpc", post(handle_rpc))
}

/// 创建并配置 JSON-RPC 处理器
fn create_rpc_handler() -> IoHandler {
    let mut io = IoHandler::new();
    
    io.add_method("get_user_info", rpc::get_user_info);
    io.add_method("update_user_info", rpc::update_user_info);
    io.add_method("verify_credentials", rpc::verify_credentials);
    
    io
}

/// 处理 JSON-RPC 请求
pub async fn handle_rpc(
    State(_state): State<AppState>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let io = create_rpc_handler();
    
    let response = io.handle_request(&payload.to_string()).await;
    match response {
        Some(resp) => {
            let json_value: Value = serde_json::from_str(&resp).unwrap_or_else(|_| json!({"error": "Invalid response"}));
            Json(json_value)
        }
        None => Json(json!({"error": "Invalid request"})),
    }
}