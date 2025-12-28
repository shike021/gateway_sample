# Axum Gateway

A multi-protocol gateway server built with Rust and the Axum framework, providing REST API, JSON-RPC, and gRPC interfaces.

## Features

- Built with Rust and the Axum Web framework
- Supports multiple protocols: REST API, JSON-RPC, gRPC
- Modular architecture with separated routing and business logic
- CORS support
- Easy to extend and maintain

## Project Structure

```
axum_gateway/
├── src/
│   ├── main.rs              # Program entry point
│   ├── server.rs            # Server startup and configuration module
│   ├── routes/              # Route definition module
│   │   ├── mod.rs           # Route module entry
│   │   ├── grid.rs          # Grid related routes
│   │   └── json_rpc.rs      # JSON-RPC routes
│   ├── handlers/            # Business logic module
│   │   ├── mod.rs           # Business logic module entry
│   │   ├── grid.rs          # Grid business logic
│   │   ├── user_info.rs     # User info handling
│   │   └── grpc_helloworld.rs # gRPC service implementation
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
- **Port**: 3000 (integrated with REST API)
- **Path**: `/jsonrpc`
- **Functionality**: Provides JSON-RPC 2.0 compliant interfaces
- **Example calls**:
  ```bash
  # Get user info
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"get_user_info","params":["user123"],"id":1}'
  
  # Update user info
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"update_user_info","params":["user123",{"name":"John Doe"}],"id":2}'
  
  # Verify credentials
  curl -X POST http://localhost:3000/jsonrpc \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"verify_credentials","params":["user123","password123"],"id":3}'
  ```

#### Implementation Details

**Current Implementation**:
- Uses `jsonrpc-core` library + Axum RESTful routing
- JSON-RPC requests are received via Axum POST endpoint, then delegated to jsonrpc-core's IoHandler
- Pros: Fully integrated with Axum, shares middleware (CORS, logging, etc.), unified port management, simple deployment
- Cons: Requires creating IoHandler per request (can be optimized with caching), manual JSON serialization handling

**Alternative Implementation Options**:

1. **jsonrpc-v2 + Axum**
   - Better type safety and async support
   - Simpler API, more comprehensive error handling
   - Requires additional dependencies

2. **Standalone jsonrpc-http-server**
   - Out-of-the-box JSON-RPC server
   - Requires separate port and management
   - Cannot share Axum middleware

3. **axum-jsonrpc (if available)**
   - JSON-RPC integration designed specifically for Axum
   - Better Axum ecosystem compatibility

The current choice of jsonrpc-core + Axum is most suitable for this project, as it needs to support REST, JSON-RPC, and gRPC protocols simultaneously. Using Axum uniformly simplifies architecture and deployment.

### gRPC API
- **Port**: 5000
- **Functionality**: Provides high-performance gRPC interfaces
- **Example call** (using grpcurl):
  ```bash
  grpcurl -plaintext -d '{"name":"World"}' localhost:5000 helloworld.Greeter/SayHello
  ```

## Quick Start

1. Ensure Rust development environment is installed
2. Clone the project code
3. Run `cargo run` to start the server
4. The server will start three services simultaneously:
   - REST API and JSON-RPC: `http://localhost:3000`
   - gRPC: `http://localhost:5000`

## Configuration

The server can be configured via `config.toml` file:

```toml
[server]
# REST API and JSON-RPC server address
rest_host = "127.0.0.1"
rest_port = 3000

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
- [jsonrpc-core](https://crates.io/crates/jsonrpc-core) - JSON-RPC implementation
- [tower-http](https://crates.io/crates/tower-http) - HTTP middleware (CORS, etc.)
- [tracing](https://crates.io/crates/tracing) - Logging and tracing
- [config](https://crates.io/crates/config) - Configuration management

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
