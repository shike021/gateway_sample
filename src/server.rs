//! 服务器模块
//!
//! 负责配置和启动 REST API 服务器。

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use jsonrpc_core::{IoHandler, Params, Value};
use jsonrpc_http_server::ServerBuilder;
use serde_json::json;
use tonic::transport::Server;
use crate::handlers::grpc_helloworld::GreeterService;
use crate::protos::helloworld::greeter_server::GreeterServer;

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

    // 启动REST API服务器
    let rest_server = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });

    // 启动JSON-RPC服务器（监听4000端口）
    let rpc_addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let mut io = IoHandler::new();
    
    // 添加用户信息相关的RPC方法
    io.add_method("get_user_info", |_params| async move {
        Ok(json!({
            "name": "John Doe",
            "age": 30,
            "email": "john@example.com",
            "status": "active"
        }))
    });
    
    io.add_method("update_user_info", |params: Params| async move {
        let params: Value = params.parse().unwrap_or_else(|_| json!({}));
        let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let age = params.get("age").and_then(|v| v.as_u64()).unwrap_or(0);
        Ok(json!({
            "success": true,
            "message": format!("用户信息已更新: {} ({}岁)", name, age)
        }))
    });
    
    io.add_method("verify_credentials", |params: Params| async move {
        let params: Value = params.parse().unwrap_or_else(|_| json!({}));
        let username = params.get("username").and_then(|v| v.as_str()).unwrap_or("");
        let password = params.get("password").and_then(|v| v.as_str()).unwrap_or("");
        Ok(json!({
            "authenticated": username == "admin" && password == "123456",
            "token": "sample-jwt-token"
        }))
    });
    
    tracing::info!("Starting JSON-RPC server on {}", rpc_addr);
    let rpc_server = ServerBuilder::new(io)
        .start_http(&rpc_addr)
        .expect("Unable to start RPC server");
    
    // 启动GRPC服务器（监听5000端口）
    let grpc_addr = "[::1]:5000".parse().unwrap();
    let grpc_server = Server::builder()
        .add_service(GreeterServer::new(GreeterService::default()))
        .serve(grpc_addr);
    
    tracing::info!("Starting GRPC server on {}", grpc_addr);
    let grpc_server = tokio::spawn(async move {
        grpc_server.await?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    });
    
    // 等待三个服务器完成
    let _ = tokio::try_join!(rest_server, async { rpc_server.wait(); Ok(()) }, grpc_server);
    
    Ok(())
}
