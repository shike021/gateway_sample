# Axum Gateway

一个基于 Rust 和 Axum 框架构建的多协议网关服务器，提供 REST API、JSON-RPC 和 gRPC 接口。

## 功能特性

- 基于 Rust 语言和 Axum Web 框架
- 支持多种协议：REST API、JSON-RPC、gRPC
- 模块化架构，路由与业务逻辑分离
- CORS 支持
- 易于扩展和维护

## 项目结构

```
axum_gateway/
├── src/
│   ├── main.rs              # 程序入口点
│   ├── server.rs            # 服务启动和配置模块
│   ├── routes/              # 路由定义模块
│   │   ├── mod.rs           # 路由模块入口
│   │   ├── grid.rs          # Grid 相关路由
│   │   └── json_rpc.rs      # JSON-RPC 路由
│   ├── handlers/            # 业务处理模块
│   │   ├── mod.rs           # 业务处理模块入口
│   │   ├── grid.rs          # Grid 业务逻辑
│   │   ├── user_info.rs     # 用户信息处理
│   │   └── grpc_helloworld.rs # gRPC 服务实现
│   └── protos/              # Protobuf 生成的代码
├── Cargo.toml               # 项目依赖配置
└── README.md                # 项目说明文档
```

## 支持的 API 协议

### REST API
- **端口**: 3000
- **功能**: 提供基础的 RESTful 接口
- **示例调用**:
  ```bash
  # 获取所有网格数据
  curl -X GET http://localhost:3000/grid
  
  # 获取特定网格项
  curl -X GET http://localhost:3000/grid/1
  
  # 创建新的网格项
  curl -X POST http://localhost:3000/grid \
    -H "Content-Type: application/json" \
    -d '{"name":"Test Grid","rows":10,"columns":10}'
  
  # 更新网格项
  curl -X PUT http://localhost:3000/grid/1 \
    -H "Content-Type: application/json" \
    -d '{"name":"Updated Grid","rows":20,"columns":20}'
  
  # 删除网格项
  curl -X DELETE http://localhost:3000/grid/1
  ```

### JSON-RPC API
- **端口**: 3000（集成在 REST API 中）
- **路径**: `/jsonrpc`
- **功能**: 提供 JSON-RPC 2.0 规范的接口
- **示例调用**:
  ```bash
  # 获取用户信息
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"get_user_info","params":["user123"],"id":1}'
  
  # 更新用户信息
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"update_user_info","params":["user123",{"name":"John Doe"}],"id":2}'
  
  # 验证凭据
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"verify_credentials","params":["user123","password123"],"id":3}'
  ```

### gRPC API
- **端口**: 5000
- **功能**: 提供高性能的 gRPC 接口
- **示例调用** (使用 grpcurl):
  ```bash
  grpcurl -plaintext -d '{"name":"World"}' localhost:5000 helloworld.Greeter/SayHello
  ```

## 快速开始

1. 确保已安装 Rust 开发环境
2. 克隆项目代码
3. 运行 `cargo run` 启动服务器
4. 服务器将同时启动三个服务：
   - REST API 和 JSON-RPC: `http://localhost:3000`
   - gRPC: `http://localhost:5000`

## 依赖

- [Axum](https://crates.io/crates/axum) - Web 框架
- [Tokio](https://crates.io/crates/tokio) - 异步运行时
- [Serde](https://crates.io/crates/serde) - 序列化/反序列化库
- [Tonic](https://crates.io/crates/tonic) - gRPC 框架
- [jsonrpc-core](https://crates.io/crates/jsonrpc-core) - JSON-RPC 实现
- [tower-http](https://crates.io/crates/tower-http) - HTTP 中间件（CORS 等）
- [tracing](https://crates.io/crates/tracing) - 日志和追踪
