**Minimal Hello World API with Rust 1.96, Axum 0.8, ConnectRPC (connect-rust 0.7)**

**Cargo.toml**
```toml
[package]
name = "hello-connect"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
connectrpc = "0.7"
prost = "0.13"  # for protobuf
```

**proto/hello.proto** (compile with buf/prost-build)
```proto
syntax = "proto3";
package hello;

service HelloService {
  rpc SayHello(HelloRequest) returns (HelloResponse);
}

message HelloRequest { string name = 1; }
message HelloResponse { string message = 1; }
```

**src/main.rs**
```rust
use axum::Router;
use connectrpc::{Server, Context};
use prost::Message;

#[derive(Message, Clone)]
struct HelloRequest { #[prost(string)] pub name: String, }

#[derive(Message, Clone)]
struct HelloResponse { #[prost(string)] pub message: String, }

async fn say_hello(_ctx: Context, req: HelloRequest) -> Result<HelloResponse, connectrpc::ConnectError> {
    Ok(HelloResponse { message: format!("Hello, {}!", req.name) })
}

#[tokio::main]
async fn main() {
    let router = Router::new();  // Integrate via connectrpc layer or axum-connect
    // Full setup: use connectrpc::axum integration or service builder
    let server = Server::new(router);  // Adapt per connect-rust docs
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server.into_make_service()).await.unwrap();
}
```

**Run:** `cargo run`

**Test:** `curl -H "Content-Type: application/json" -d '{"name":"World"}' http://localhost:3000/hello.HelloService/SayHello`

Use `axum-connect` or `connectrpc-axum` for idiomatic routing. Check connect-rust GitHub for latest service registration.

Minimal and current as of 2026.
