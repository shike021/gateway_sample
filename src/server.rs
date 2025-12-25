//! 服务器模块
//!
//! 负责配置和启动 REST API 服务器。

use axum::Router;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use jsonrpc_core::IoHandler;
use jsonrpc_http_server::{ServerBuilder, hyper::server::conn::AddrIncoming};

// 引入路由模块
use crate::routes;

/// 应用状态
pub type AppState = routes::grid::AppState;

/// 启动服务器
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "axum_gateway=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 初始化应用状态
    let state = AppState {
        grid_items: Arc::new(RwLock::new(Vec::new())),
    };

    // 构建应用路由
    let app = routes::app_routes()
        .with_state(state) // 添加应用状态
        .layer(CorsLayer::permissive()); // 添加 CORS 支持

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
