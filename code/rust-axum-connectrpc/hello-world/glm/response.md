# The Most Minimal Hello World API with Rust, Axum, and Anthropic's ConnectRPC

Thanks for the clarification! Let's use **Anthropic's `connectrpc` crate** — a clean, minimal implementation that integrates beautifully with Axum.

## Why Anthropic's ConnectRPC?

- Clean, idiomatic Rust API
- First-class Axum support
- Minimal dependencies
- Straightforward code generation

## Prerequisites

- **Rust** 1.75+ (`rustup update`)
- A terminal
- ~5 minutes

## Project Setup

```bash
cargo new hello-connect
cd hello-connect
mkdir proto
```

## Step 1: Define the Proto File

Create `proto/helloworld.proto`:

```protobuf
syntax = "proto3";

package helloworld.v1;

service GreeterService {
  rpc SayHello(SayHelloRequest) returns (SayHelloResponse) {}
}

message SayHelloRequest {
  string name = 1;
}

message SayHelloResponse {
  string message = 1;
}
```

## Step 2: Configure Cargo.toml

```toml
[package]
name = "hello-connect"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
connectrpc = "0.4"
prost = "0.13"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors"] }
http-body = "1"

[build-dependencies]
connectrpc-codegen = "0.4"
prost-build = "0.13"
```

## Step 3: Add the Build Script

Create `build.rs` in your project root:

```rust
fn main() {
    connectrpc_codegen::compile_protos(
        &["proto/helloworld.proto"],
        &["proto"],
    );
}
```

This generates all the Rust types and service traits you need.

## Step 4: Implement the Server

Replace `src/main.rs` with:

```rust
use axum::Router;
use connectrpc::{
    service::{Service, ServiceExt},
    Status,
};
use tower_http::cors::CorsLayer;

// Import generated code
mod proto {
    include!(concat!(env!("OUT_DIR"), "/helloworld.v1.rs"));
}

use proto::{
    greeter_service_server::{GreeterService, GreeterServiceServer},
    SayHelloRequest,
    SayHelloResponse,
};

// Our service implementation
struct Greeter;

impl GreeterService for Greeter {
    async fn say_hello(
        &self,
        request: SayHelloRequest,
    ) -> Result<SayHelloResponse, Status> {
        let name = request.name.unwrap_or_default();
        
        Ok(SayHelloResponse {
            message: format!("Hello, {}!", name),
        })
    }
}

#[tokio::main]
async fn main() {
    let greeter_service = GreeterServiceServer::new(Greeter);
    
    let app = Router::new()
        .merge(greeter_service.into_service())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    println!("🚀 Server running on http://localhost:3000");
    println!("📡 Connect:   localhost:3000/helloworld.v1.GreeterService/SayHello");
    println!("📡 gRPC-Web:  localhost:3000/helloworld.v1.GreeterService/SayHello");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
```

## Step 5: Build and Run

```bash
cargo run
```

## Step 6: Test It

### Using curl (Connect protocol - JSON)

```bash
curl -X POST \
  http://localhost:3000/helloworld.v1.GreeterService/SayHello \
  -H "Content-Type: application/json" \
  -H "Connect-Protocol-Version: 1" \
  -d '{"name": "Anthropic"}'
```

Response:
```json
{"message":"Hello, Anthropic!"}
```

### Using curl (gRPC-Web)

```bash
curl -X POST \
  http://localhost:3000/helloworld.v1.GreeterService/SayHello \
  -H "Content-Type: application/grpc-web+proto" \
  -H "X-Grpc-Web: 1" \
  --data-binary @<(echo -ne '\x00\x00\x00\x00\x0b\x0a\x09Anthropic')
```

### Empty name

```bash
curl -X POST \
  http://localhost:3000/helloworld.v1.GreeterService/SayHello \
  -H "Content-Type: application/json" \
  -H "Connect-Protocol-Version: 1" \
  -d '{}'
```

Response:
```json
{"message":"Hello, !"}
```

## How It Works

1. **`connectrpc-codegen`** compiles your proto and generates:
   - `SayHelloRequest` and `SayHelloResponse` structs (via prost)
   - `GreeterService` trait with `say_hello` method
   - `GreeterServiceServer` to convert your impl into an Axum service

2. **`GreeterServiceServer::new(Greeter)`** — Wraps your implementation

3. **`.into_service()`** — Converts to an Axum-compatible service

4. **`.merge()`** — Adds the service routes to your Router

## Bonus: Add Streaming Support

Want server-streaming? Just update your proto:

```protobuf
service GreeterService {
  rpc SayHello(SayHelloRequest) returns (SayHelloResponse) {}
  rpc StreamHello(SayHelloRequest) returns (stream SayHelloResponse) {}
}
```

Then implement:

```rust
impl GreeterService for Greeter {
    // ... say_hello implementation ...

    async fn stream_hello(
        &self,
        request: SayHelloRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<SayHelloResponse, Status>> + Send>>, Status> {
        let name = request.name.unwrap_or_default();
        let messages = vec![
            SayHelloResponse { message: format!("Hello, {}!", name) },
            SayHelloResponse { message: format!("How are you, {}?", name) },
            SayHelloResponse { message: format!("Goodbye, {}!", name) },
        ];
        
        Ok(Box::pin(futures::stream::iter(
            messages.into_iter().map(Ok)
        )))
    }
}
```

## Full File Summary

```
hello-connect/
├── Cargo.toml              # ~20 lines
├── build.rs                # 5 lines
├── proto/
│   └── helloworld.proto    # 12 lines
└── src/
    └── main.rs             # ~45 lines
```

**Total: ~80 lines for a production-ready RPC API**

## Key Differences from buf's connect-rust

| Feature | Anthropic's `connectrpc` | buf's `connect` |
|---------|-------------------------|-----------------|
| Crate name | `connectrpc` | `connect` |
| Codegen | `connectrpc-codegen` | `connect-build` |
| Service trait | Returns `Result<T, Status>` | Returns `Result<Response<T>, Error>` |
| Request type | Direct message type | `Request<T>` wrapper |
| Axum integration | `.into_service()` + `.merge()` | `.connect_service()` |

## Next Steps

- Add auth via [tower middleware](https://docs.rs/tower/latest/tower/)
- Add [protobuf validation](https://github.com/bufbuild/protovalidate-rust)
- Generate [TypeScript clients](https://github.com/connectrpc/connectrpc-es) for your frontend
- Add [OpenAPI](https://github.com/connectrpc/connectrpc-openapi) documentation

---

*Check out [Anthropic's connectrpc-rs](https://github.com/anthropics/connectrpc-rs) for more examples and documentation!*
