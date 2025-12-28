//! 服务器模块
//!
//! 负责配置和启动 REST API 服务器。

use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tonic::transport::Server;
use crate::config::Config;
use crate::handlers::grpc_helloworld::GreeterService;
use crate::protos::helloworld::greeter_server::GreeterServer;

// 引入路由模块
use crate::routes;

/// 应用状态
pub type AppState = routes::grid::AppState;

/// 启动服务器
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::from_file("config")
        .unwrap_or_else(|_| {
            tracing::warn!("Failed to load config file, using defaults");
            Config::default()
        });

    // 初始化日志
    let log_level = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| config.logging.level.clone());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(&log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 初始化应用状态
    let state = AppState {
        grid_items: Arc::new(RwLock::new(Vec::new())),
    };

    // 构建应用路由
    let app = routes::app_routes()
        .with_state(state)
        .layer(CorsLayer::permissive());

    // 获取 REST 服务器地址
    let rest_addr = config.rest_addr()?;
    tracing::info!("Starting REST API server on {}", rest_addr);

    // 启动REST API服务器
    let rest_server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(rest_addr).await?;
        axum::serve(listener, app).await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });

    // 获取 gRPC 服务器地址
    let grpc_addr = config.grpc_addr()?;
    let grpc_server = Server::builder()
        .add_service(GreeterServer::new(GreeterService::default()))
        .serve(grpc_addr);
    
    tracing::info!("Starting GRPC server on {}", grpc_addr);
    let grpc_server = tokio::spawn(async move {
        grpc_server.await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });
    
    // 等待两个服务器完成
    let _ = tokio::try_join!(rest_server, grpc_server);
    
    Ok(())
}
