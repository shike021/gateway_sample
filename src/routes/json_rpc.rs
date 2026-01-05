//! JSON-RPC route handling
//!
//! Handles JSON-RPC related requests using jsonrpsee native server.
//!
//! Copyright © 2025 imshike@gmail.com
//! SPDX-License-Identifier: Apache-2.0
//! Author: imshike@gmail.com

use jsonrpsee::server::RpcModule;
use jsonrpsee::types::ErrorObjectOwned;
use serde_json::Value;

use crate::handlers::user_info as rpc;

/// Create and configure JSON-RPC module
pub fn create_rpc_module() -> RpcModule<()> {
    let mut module = RpcModule::new(());

    module
        .register_async_method("get_user_info", |_params, _subscription, _ctx| async move {
            rpc::get_user_info().await.map_err(|e: ErrorObjectOwned| e)
        })
        .unwrap();

    module
        .register_async_method("update_user_info", |params, _subscription, _ctx| async move {
            let value: Value = params.parse().unwrap_or_default();
            rpc::update_user_info(value).await.map_err(|e: ErrorObjectOwned| e)
        })
        .unwrap();

    module
        .register_async_method("verify_credentials", |params, _subscription, _ctx| async move {
            let value: Value = params.parse().unwrap_or_default();
            rpc::verify_credentials(value).await.map_err(|e: ErrorObjectOwned| e)
        })
        .unwrap();

    module
}
