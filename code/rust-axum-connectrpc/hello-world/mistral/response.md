# Minimal Hello World API with Rust, Axum, and ConnectRPC

*By Oliver Forral*

---

## Introduction

In this tutorial, we’ll build the **most minimal "Hello World" API** using the latest versions of:
- **Rust** (stable)
- **Axum** (web framework)
- **ConnectRPC** (`connect-rust`)

This guide assumes you have:
- Rust installed (`rustup`)
- Basic familiarity with Rust and `cargo`

---

## Step 1: Create a New Rust Project

```bash
cargo new minimal-hello-world-api
cd minimal-hello-world-api
```

---

## Step 2: Add Dependencies

Edit your `Cargo.toml`:

```toml
[package]
name = "minimal-hello-world-api"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1", features = ["full"] }

# ConnectRPC
connect-rust = "0.13"
prost = "0.12"
prost-types = "0.12"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## Step 3: Define the Protobuf Schema

Create a file `proto/hello.proto`:

```protobuf
syntax = "proto3";

package hello.v1;

service Greeter {
  rpc SayHello (HelloRequest) returns (HelloResponse);
}

message HelloRequest {
  string name = 1;
}

message HelloResponse {
  string message = 1;
}
```

---

## Step 4: Generate Rust Code from Protobuf

Add a `build.rs` file to your project root:

```rust
fn main() {
    let proto_file = "proto/hello.proto";
    tonic_build::compile_protos(proto_file).unwrap();
}
```

Add `tonic-build` as a build dependency in `Cargo.toml`:

```toml
[build-dependencies]
tonic-build = "0.10"
```

Run:
```bash
cargo build
```

This generates Rust code in `src/hello.v1.rs`.

---

## Step 5: Implement the gRPC Service

Create `src/lib.rs`:

```rust
pub mod hello {
    tonic::include_proto!("hello.v1");
}

use hello::greeter_server::Greeter;
use hello::{HelloRequest, HelloResponse};

#[derive(Default)]
pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
        let response = HelloResponse {
            message: format!("Hello, {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(response))
    }
}
```

---

## Step 6: Set Up Axum and ConnectRPC

Edit `src/main.rs`:

```rust
mod lib;
use lib::GreeterService;

use axum::{
    routing::get,
    Router,
};
use connect_rust::{
    proto::PathService,
    router::Router as ConnectRouter,
};
use hello::greeter_server::GreeterServer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a ConnectRPC router
    let mut connect_router = ConnectRouter::new();
    connect_router.add_service(
        PathService::new(GreeterServer::new(GreeterService::default())),
    );

    // Create an Axum router
    let app = Router::new()
        .nest_service("/connect", connect_router)
        .route("/", get(|| async { "Hello, World!" }));

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

## Step 7: Run the Server

```bash
cargo run
```

Visit:
- **HTTP**: [http://localhost:3000](http://localhost:3000)
- **gRPC**: Use a tool like `grpcurl` or `bloomrpc` to call the `SayHello` endpoint.

---

## Step 8: Test the gRPC Endpoint

Install `grpcurl`:
```bash
brew install grpcurl
```

Call the endpoint:
```bash
grpcurl -plaintext -d '{"name": "World"}' localhost:3000 connect.hello.v1.Greeter/SayHello
```

Expected output:
```json
{
  "message": "Hello, World!"
}
```

---

## Conclusion

You now have a **minimal "Hello World" API** using:
- **Axum** for HTTP
- **ConnectRPC** for gRPC

This setup is **lightweight, fast, and production-ready** for microservices.

---

## Next Steps

- Add authentication (e.g., JWT)
- Explore ConnectRPC’s [documentation](https://connect.build/docs/rust)
- Integrate with databases (e.g., `sqlx`)

---

*Questions? Suggestions? Open an issue or PR on [GitHub](https://github.com/your-repo).*