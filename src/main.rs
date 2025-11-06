//! Axum Grid Server 主入口点
//!
//! 这个文件只负责启动和配置服务，不包含具体的业务逻辑。

mod handlers;
mod routes;
mod server;

use server::start_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动服务器
    start_server().await
}
