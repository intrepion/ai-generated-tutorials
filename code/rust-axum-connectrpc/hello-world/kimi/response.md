I'll create a comprehensive tutorial blog post for you. Let me first search for the latest information on connect-rust and Axum to ensure everything is up-to-date.


---

# The Most Minimal Hello World API with Rust, Axum, and ConnectRPC

> **June 2026** | Rust 1.85+, Axum 0.8, ConnectRPC (connect-rust) 0.6

If you're looking for the absolute smallest possible ConnectRPC server running on Axum — the kind you can read in one sitting and actually understand — this is it. No boilerplate. No ceremony. Just a working gRPC/Connect endpoint in under 50 lines of Rust.

---

## What We're Building

A single `Greet` method that accepts a name and returns a greeting. It speaks three protocols simultaneously:

- **Connect** (JSON over HTTP/1.1 — the modern REST replacement)
- **gRPC** (binary over HTTP/2 — the standard)
- **gRPC-Web** (for browser clients)

All from one Axum server on port `8080`.

---

## Prerequisites

```bash
# Rust (latest stable)
rustc --version   # 1.85+ recommended

# Protocol Buffers compiler
protoc --version  # 3.21+

# The ConnectRPC code generator
cargo install --locked connectrpc-codegen
```

---

## Step 1: Define the Service

Create `proto/greet.proto`:

```protobuf
syntax = "proto3";

package greet.v1;

service GreetService {
  rpc Greet(GreetRequest) returns (GreetResponse);
}

message GreetRequest {
  string name = 1;
}

message GreetResponse {
  string greeting = 1;
}
```

---

## Step 2: Generate Rust Code

Create `build.rs`:

```rust
fn main() {
    connectrpc_build::Config::new()
        .files(&["proto/greet.proto"])
        .includes(&["proto/"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
```

And in `Cargo.toml`:

```toml
[package]
name = "hello-connect"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
connectrpc = "0.6"

[build-dependencies]
connectrpc-build = "0.6"
```

Run generation once:

```bash
cargo build
```

This creates `src/generated/` with your message types and service stubs.

---

## Step 3: Implement the Server

`src/main.rs` — the entire application:

```rust
use std::sync::Arc;
use axum::{Router, routing::get};
use connectrpc::{Router as ConnectRouter, Context, ConnectError};

// 1. Include the generated code
pub mod proto {
    connectrpc::include_generated!();
}

use proto::greet::v1::{GreetRequestView, GreetResponse};

// 2. Implement the service trait
struct MyGreetService;

impl proto::greet::v1::GreetService for MyGreetService {
    async fn greet(
        &self,
        ctx: Context,
        request: connectrpc::buffa::OwnedView<GreetRequestView<'static>>,
    ) -> Result<(GreetResponse, Context), ConnectError> {
        // Zero-copy: `request.name` is borrowed `&str` from the wire buffer
        let response = GreetResponse {
            greeting: format!("Hello, {}!", request.name),
            ..Default::default()
        };
        Ok((response, ctx))
    }
}

#[tokio::main]
async fn main() {
    // 3. Wrap in Arc and register with ConnectRPC router
    let service = Arc::new(MyGreetService);
    let connect = service.register(ConnectRouter::new());

    // 4. Mount into Axum with a fallback for all RPC routes
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .fallback_service(connect.into_axum_service());

    // 5. Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
```

That's it. **43 lines of real code.** 

---

## Step 4: Test It

### Using Connect (JSON over HTTP/1.1)

```bash
curl -X POST http://localhost:8080/greet.v1.GreetService/Greet \
  -H "Content-Type: application/json" \
  -d '{"name": "World"}'
```

Response:
```json
{"greeting": "Hello, World!"}
```

### Using gRPC

```bash
grpcurl -plaintext -d '{"name": "World"}' \
  localhost:8080 greet.v1.GreetService/Greet
```

Response:
```json
{
  "greeting": "Hello, World!"
}
```

Both protocols hit the exact same handler. The framework negotiates the wire format from the `Content-Type` header automatically. 

---

## What's Happening Under the Hood

| Component | Role |
|-----------|------|
| **`buffa`** | Zero-copy Protobuf views — your `request.name` is a `&str` borrowed directly from the HTTP body, no heap allocation |
| **`ConnectRouter`** | Owns all your service stubs and routes RPC calls by method name |
| **`into_axum_service()`** | Converts the Connect router into an Axum `Service`, mounted as a fallback |
| **`Arc`** | Shared ownership of your service state — safe for concurrent requests |

The `..Default::default()` in `GreetResponse` fills in any unset fields (like unknown future fields) without you having to list them. 

---

## Why This Stack?

- **Rust** — Memory safety without GC, zero-cost abstractions
- **Axum 0.8** — Native async traits (no `#[async_trait]` macro), `{id}` path syntax, built by the Tokio team 
- **ConnectRPC** — One protocol for browsers, mobile, and backend services. No Envoy sidecar needed for gRPC-Web. JSON when you want it, binary when you need it.

---

## Next Steps

From here, you can add:

- **Streaming** — `rpc StreamGreet(stream GreetRequest) returns (stream GreetResponse)`
- **Middleware** — Tower layers for auth, logging, compression
- **Client code** — `connectrpc::client::HttpClient` with connection pooling
- **TLS** — Swap `HttpClient::plaintext()` for `.with_tls()`

The full API surface is documented at the [connect-rust GitHub repo](https://github.com/anthropics/connect-rust). 

---

**Happy building.** 🦀
