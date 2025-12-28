//! 配置管理模块
//!
//! 负责从配置文件加载和管理应用程序配置。

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub rest_host: String,
    pub rest_port: u16,
    pub grpc_host: String,
    pub grpc_port: u16,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

impl Config {
    /// 从配置文件加载配置
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;

        Ok(settings.try_deserialize()?)
    }

    /// 获取 REST 服务器地址
    pub fn rest_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.server.rest_host, self.server.rest_port);
        addr.parse().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    /// 获取 gRPC 服务器地址
    pub fn grpc_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        let addr = format!("{}:{}", self.server.grpc_host, self.server.grpc_port);
        addr.parse().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                rest_host: "127.0.0.1".to_string(),
                rest_port: 3000,
                grpc_host: "[::1]".to_string(),
                grpc_port: 5000,
            },
            logging: LoggingConfig {
                level: "debug".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.server.rest_host, "127.0.0.1");
        assert_eq!(config.server.rest_port, 3000);
        assert_eq!(config.server.grpc_host, "[::1]");
        assert_eq!(config.server.grpc_port, 5000);
        assert_eq!(config.logging.level, "debug");
    }

    #[test]
    fn test_config_rest_addr() {
        let config = Config::default();
        let addr = config.rest_addr().unwrap();
        assert_eq!(addr.port(), 3000);
    }

    #[test]
    fn test_config_grpc_addr() {
        let config = Config::default();
        let addr = config.grpc_addr().unwrap();
        assert_eq!(addr.port(), 5000);
    }
}
