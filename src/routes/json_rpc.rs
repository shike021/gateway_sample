//! JSON-RPC route handling
//!
//! Handles JSON-RPC related requests and forwards requests to business logic modules.
//!
//! Copyright © 2024 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use axum::{extract::State, response::Json, routing::post, Router, http::StatusCode};
use jsonrpc_core::{IoHandler, Value};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::handlers::user_info as rpc;
use crate::server::AppState;

/// JSON-RPC 2.0 request
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcRequest {
    jsonrpc: Option<String>,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 error response
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

/// JSON-RPC error codes
#[allow(dead_code)]
mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

/// Define JSON-RPC related routes
pub fn rpc_routes() -> Router<AppState> {
    Router::new().route("/jsonrpc", post(handle_rpc))
}

/// Create and configure JSON-RPC handler
pub fn create_rpc_handler() -> IoHandler {
    let mut io = IoHandler::new();
    
    io.add_method("get_user_info", rpc::get_user_info);
    io.add_method("update_user_info", rpc::update_user_info);
    io.add_method("verify_credentials", rpc::verify_credentials);
    
    io
}

/// Validate JSON-RPC request format
fn validate_request(payload: &Value) -> Result<(), JsonRpcError> {
    if !payload.is_object() {
        return Err(JsonRpcError {
            code: error_codes::INVALID_REQUEST,
            message: "Invalid Request: payload must be an object".to_string(),
            data: None,
        });
    }

    let request: JsonRpcRequest = match serde_json::from_value(payload.clone()) {
        Ok(req) => req,
        Err(_) => {
            return Err(JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: "Invalid Request: missing required fields".to_string(),
                data: None,
            });
        }
    };

    if request.method.is_empty() {
        return Err(JsonRpcError {
            code: error_codes::INVALID_REQUEST,
            message: "Invalid Request: method is required".to_string(),
            data: None,
        });
    }

    if let Some(ref jsonrpc_version) = request.jsonrpc {
        if jsonrpc_version != "2.0" {
            return Err(JsonRpcError {
                code: error_codes::INVALID_REQUEST,
                message: "Invalid Request: jsonrpc version must be 2.0".to_string(),
                data: None,
            });
        }
    }

    Ok(())
}

/// Handle JSON-RPC request
pub async fn handle_rpc(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    let request_id = payload.get("id").cloned();

    if let Err(error) = validate_request(&payload) {
        return (
            StatusCode::OK,
            Json(json!(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(error),
                id: request_id,
            })),
        );
    }

    let response = state.rpc_handler.handle_request(&payload.to_string()).await;
    
    match response {
        Some(resp) => {
            match serde_json::from_str::<Value>(&resp) {
                Ok(json_value) => (StatusCode::OK, Json(json_value)),
                Err(_) => (
                    StatusCode::OK,
                    Json(json!(JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: error_codes::INTERNAL_ERROR,
                            message: "Internal Error: failed to parse response".to_string(),
                            data: None,
                        }),
                        id: request_id,
                    })),
                ),
            }
        }
        None => (
            StatusCode::OK,
            Json(json!(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: error_codes::INVALID_REQUEST,
                    message: "Invalid Request: could not process request".to_string(),
                    data: None,
                }),
                id: request_id,
            })),
        ),
    }
}