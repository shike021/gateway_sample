# Axum Gateway

A multi-protocol gateway server built with Rust and the Axum framework, providing REST API, JSON-RPC, and gRPC interfaces.

## Features

- Built with Rust and the Axum Web framework
- Supports multiple protocols: REST API, JSON-RPC, gRPC
- Modular architecture with separated routing and business logic
- CORS support
- Easy to extend and maintain
- Modern async JSON-RPC implementation with jsonrpsee
- Multiple gRPC services (GreeterService and UserService)
- Health check endpoint for monitoring
- OpenAPI/Swagger documentation support

## Project Structure

```
axum_gateway/
├── src/
│   ├── main.rs              # Program entry point
│   ├── server.rs            # Server startup and configuration module
│   ├── config.rs            # Configuration management
│   ├── routes/              # Route definition module
│   │   ├── mod.rs           # Route module entry
│   │   ├── rest.rs          # REST API routes (renamed from grid.rs)
│   │   ├── json_rpc.rs      # JSON-RPC routes
│   │   └── health.rs        # Health check routes
│   ├── handlers/            # Business logic module
│   │   ├── mod.rs           # Business logic module entry
│   │   ├── grid.rs          # Grid business logic
│   │   ├── user_info.rs     # User info handling (JSON-RPC)
│   │   ├── grpc_helloworld.rs # gRPC Greeter service implementation
│   │   └── grpc_user.rs     # gRPC UserService implementation
│   ├── errors.rs            # Error handling
│   └── protos/              # Protobuf generated code
├── Cargo.toml               # Project dependencies configuration
└── README.md                # Project documentation
```

## Supported API Protocols

### REST API
- **Port**: 3000
- **Functionality**: Provides basic RESTful interfaces
- **Example calls**:
  ```bash
  # Get all grid data
  curl -X GET http://localhost:3000/grid
  
  # Get specific grid item
  curl -X GET http://localhost:3000/grid/1
  
  # Create new grid item
  curl -X POST http://localhost:3000/grid \
    -H "Content-Type: application/json" \
    -d '{"name":"Test Grid","rows":10,"columns":10}'
  
  # Update grid item
  curl -X PUT http://localhost:3000/grid/1 \
    -H "Content-Type: application/json" \
    -d '{"name":"Updated Grid","rows":20,"columns":20}'
  
  # Delete grid item
  curl -X DELETE http://localhost:3000/grid/1
  ```

### JSON-RPC API
- **Port**: 4000
- **Functionality**: Provides JSON-RPC 2.0 compliant interfaces using jsonrpsee
- **Example calls**:
  ```bash
  # Get user info
  curl -X POST http://localhost:4000 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"get_user_info","params":{},"id":1}'
  
  # Update user info
  curl -X POST http://localhost:4000 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"update_user_info","params":{"name":"John Doe","email":"john@example.com","age":30,"status":"active"},"id":2}'
  
  # Verify credentials
  curl -X POST http://localhost:4000 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"verify_credentials","params":{"username":"testuser","password":"testpass"},"id":3}'
  ```

#### Implementation Details

**Current Implementation**:
- Uses `jsonrpsee` library for modern async JSON-RPC support
- Runs on a dedicated port (4000) separate from REST API
- Provides better type safety and async support compared to jsonrpc-core
- Uses RpcModule for registering RPC methods with proper error handling

**Key Features**:
- Full JSON-RPC 2.0 specification compliance
- Async method support with tokio
- Better error handling with ErrorObjectOwned
- Subscription support (available but not currently used)
- High-performance async implementation

### gRPC API
- **Port**: 5000
- **Functionality**: Provides high-performance gRPC interfaces with two services
- **Services**:
  - **GreeterService**: Basic greeting service with `say_hello` and `echo` methods
  - **UserService**: User management service with CRUD operations
- **Example call** (using grpcurl):
  ```bash
  # GreeterService - SayHello
  grpcurl -plaintext -d '{"name":"World"}' localhost:5000 helloworld.Greeter/SayHello
  
  # GreeterService - Echo
  grpcurl -plaintext -d '{"message":"Hello"}' localhost:5000 helloworld.Greeter/Echo
  
  # UserService - CreateUser
  grpcurl -plaintext -d '{"name":"Alice","email":"alice@example.com","age":25}' localhost:5000 user.UserService/CreateUser
  
  # UserService - GetUser
  grpcurl -plaintext -d '{"user_id":1}' localhost:5000 user.UserService/GetUser
  
  # UserService - UpdateUser
  grpcurl -plaintext -d '{"user_id":1,"name":"Alice Updated","email":"alice.updated@example.com","age":26}' localhost:5000 user.UserService/UpdateUser
  
  # UserService - DeleteUser
  grpcurl -plaintext -d '{"user_id":1}' localhost:5000 user.UserService/DeleteUser
  ```

## Quick Start

1. Ensure Rust development environment is installed
2. Clone the project code
3. Run `cargo run` to start the server
4. The server will start three services simultaneously:
   - REST API: `http://localhost:3000`
   - JSON-RPC: `http://localhost:4000`
   - gRPC: `http://localhost:5000`

## Configuration

The server can be configured via `config` file (TOML format):

```toml
[server]
# REST API server address
rest_host = "127.0.0.1"
rest_port = 3000

# JSON-RPC server address
jsonrpc_host = "127.0.0.1"
jsonrpc_port = 4000

# gRPC server address
grpc_host = "[::1]"
grpc_port = 5000

[logging]
# Log level: trace, debug, info, warn, error
level = "debug"
```

## Dependencies

- [Axum](https://crates.io/crates/axum) - Web framework
- [Tokio](https://crates.io/crates/tokio) - Async runtime
- [Serde](https://crates.io/crates/serde) - Serialization/deserialization library
- [Tonic](https://crates.io/crates/tonic) - gRPC framework
- [jsonrpsee](https://crates.io/crates/jsonrpsee) - Modern async JSON-RPC implementation
- [tower-http](https://crates.io/crates/tower-http) - HTTP middleware (CORS, etc.)
- [tracing](https://crates.io/crates/tracing) - Logging and tracing
- [config](https://crates.io/crates/config) - Configuration management
- [utoipa](https://crates.io/crates/utoipa) - OpenAPI documentation generation
- [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) - Swagger UI integration

## Testing

Run tests with:
```bash
cargo test
```

## Building

Build the project with:
```bash
cargo build --release
```
