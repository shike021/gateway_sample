//! 服务器模块
//!
//! 负责配置和启动 HTTP 服务器。

use axum::Router;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 引入路由模块
use crate::routes;

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
    let state = routes::grid::AppState {
        grid_items: Arc::new(RwLock::new(Vec::new())),
    };

    // 构建我们的应用路由
    let app = routes::grid_routes()
        .with_state(state) // 添加应用状态
        .layer(CorsLayer::permissive()); // 添加 CORS 支持

    // 绑定地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
