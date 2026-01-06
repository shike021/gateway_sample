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
- **Subscription support**:
  - JSON-RPC WebSocket subscriptions for real-time updates
  - gRPC streaming subscriptions for continuous data streams

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
  
  # Subscribe to user updates (WebSocket)
  # Use a WebSocket client to connect to ws://localhost:4000
  # Send: {"jsonrpc":"2.0","method":"subscribe_user_updates","params":{"user_id":1,"interval_seconds":2},"id":1}
  # You will receive periodic updates in the format:
  # {"jsonrpc":"2.0","method":"subscribe_user_updates","params":{"subscription":<id>,"result":{...}}}
  ```

#### JSON-RPC WebSocket Subscriptions

The JSON-RPC server supports WebSocket connections for real-time subscriptions:

**Subscription Method**: `subscribe_user_updates`
**Parameters**:
- `user_id` (integer): The user ID to subscribe to updates for
- `interval_seconds` (integer): Update interval in seconds (default: 2)

**Example using WebSocket**:
```bash
# Run the example WebSocket client
cargo run --example test_jsonrpc_ws
```

**Response Format**:
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe_user_updates",
  "params": {
    "subscription": 8860627584088728,
    "result": {
      "user_id": 1,
      "name": "User 1",
      "email": "user1@example.com",
      "age": 31,
      "update_type": "activity_update",
      "timestamp": 1767714685,
      "counter": 1
    }
  }
}
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
  - **UserService**: User management service with CRUD operations and streaming subscriptions
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
  
  # UserService - SubscribeUserUpdates (streaming subscription)
  grpcurl -plaintext -d '{"user_id":1,"interval_seconds":2}' localhost:5000 user.UserService/SubscribeUserUpdates
  ```

#### gRPC Streaming Subscriptions

The UserService supports streaming RPC for real-time updates:

**Streaming Method**: `SubscribeUserUpdates`
**Request Parameters**:
- `user_id` (int32): The user ID to subscribe to updates for
- `interval_seconds` (int32): Update interval in seconds (default: 2)

**Example using gRPC client**:
```bash
# Run the example gRPC subscription client
cargo run --example test_grpc_subscription
```

**Response Format**:
```protobuf
message UserUpdate {
  User user = 1;
  string update_type = 2;
  int64 timestamp = 3;
}
```

The streaming subscription continuously sends `UserUpdate` messages with:
- User information (id, name, email, age)
- Update type (profile_update, activity_update, status_update)
- Unix timestamp
- Counter for tracking updates

## Subscription Comparison

The gateway supports two different subscription mechanisms:

| Feature | JSON-RPC WebSocket | gRPC Streaming |
|---------|-------------------|----------------|
| Protocol | WebSocket over HTTP | gRPC streaming RPC |
| Transport | JSON text | Protocol Buffers binary |
| Connection | Single bidirectional connection | Persistent streaming connection |
| Message Format | JSON-RPC 2.0 | Protocol Buffers |
| Use Case | Web applications, browser clients | High-performance services, microservices |
| Example Client | `cargo run --example test_jsonrpc_ws` | `cargo run --example test_grpc_subscription` |

Both subscription types provide:
- Real-time updates at configurable intervals
- User-specific data streams
- Automatic reconnection handling
- Efficient message delivery

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
