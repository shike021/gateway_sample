//! JSON-RPC 处理模块
//!
//! 实现用户信息相关的 JSON-RPC 业务逻辑。

use jsonrpc_core::{Params, Result, Value};
use serde_json::json;

/// 获取用户信息
pub async fn get_user_info(_params: Params) -> Result<Value> {
    Ok(json!({
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
        "status": "active"
    }))
}

/// 更新用户信息
pub async fn update_user_info(params: Params) -> Result<Value> {
    let params: Value = params.parse()?;
    let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let age = params.get("age").and_then(|v| v.as_u64()).unwrap_or(0);
    
    Ok(json!({
        "success": true,
        "message": format!("用户信息已更新: {} ({}岁)", name, age)
    }))
}

/// 验证用户凭证
pub async fn verify_credentials(params: Params) -> Result<Value> {
    let params: Value = params.parse()?;
    let username = params.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let password = params.get("password").and_then(|v| v.as_str()).unwrap_or("");
    
    Ok(json!({
        "authenticated": username == "admin" && password == "123456",
        "token": "sample-jwt-token"
    }))
}