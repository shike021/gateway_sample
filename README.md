# Axum Grid Server

一个基于 Rust 和 Axum 框架构建的网格服务器，提供 RESTful API 接口。

## 功能特性

- 基于 Rust 语言和 Axum Web 框架
- RESTful API 设计
- 模块化架构，路由与业务逻辑分离
- 易于扩展和维护

## 项目结构

```
axum_gateway/
├── src/
│   ├── main.rs          # 程序入口点
│   ├── server.rs        # 服务启动和配置模块
│   ├── routes/          # 路由定义模块
│   │   └── mod.rs       # 路由模块入口
│   └── handlers/        # 业务处理模块
│       └── mod.rs       # 业务处理模块入口
├── Cargo.toml           # 项目依赖配置
└── README.md            # 项目说明文档
```

## API 接口

### 获取网格数据
```http
GET /grid
```

### 获取特定网格项
```http
GET /grid/:id
```

### 创建新的网格项
```http
POST /grid
```

### 更新网格项
```http
PUT /grid/:id
```

### 删除网格项
```http
DELETE /grid/:id
```

## 快速开始

1. 确保已安装 Rust 开发环境
2. 克隆项目代码
3. 运行 `cargo run` 启动服务器
4. 服务器将在 `http://localhost:3000` 上运行

## 依赖

- [Axum](https://crates.io/crates/axum) - Web 框架
- [Tokio](https://crates.io/crates/tokio) - 异步运行时
- [Serde](https://crates.io/crates/serde) - 序列化/反序列化库
