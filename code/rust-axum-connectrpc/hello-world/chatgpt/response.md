# Build the Smallest Possible Hello World API with Rust, Axum, and ConnectRPC

Modern APIs often start with REST because it's familiar. But if you're starting a brand-new project, **ConnectRPC** is worth considering. It combines many of the advantages of gRPC with much simpler HTTP semantics, browser support, JSON support, and excellent developer ergonomics.

In this tutorial we'll build the smallest possible API using:

* Rust (stable)
* Axum
* ConnectRPC

By the end you'll have a server exposing a single RPC:

```
SayHello("World") -> "Hello, World!"
```

No database.
No authentication.
No Docker.
Just the minimum needed to understand how the pieces fit together.

---

# Prerequisites

Install:

* Rust (stable)
* protoc (Protocol Buffers compiler)

Verify:

```bash
rustc --version
cargo --version
protoc --version
```

---

# Create the project

```bash
cargo new hello-api
cd hello-api
```

Project layout:

```
hello-api/
├── Cargo.toml
├── build.rs
├── proto/
│   └── hello.proto
└── src/
    └── main.rs
```

---

# Create the Protocol Buffer

Create:

```text
proto/hello.proto
```

```proto
syntax = "proto3";

package hello.v1;

service HelloService {
  rpc SayHello(HelloRequest) returns (HelloResponse);
}

message HelloRequest {
  string name = 1;
}

message HelloResponse {
  string message = 1;
}
```

---

# Configure Cargo

Replace your `Cargo.toml` with something similar to the following (using the latest crate versions available when you read this):

```toml
[package]
name = "hello-api"
version = "0.1.0"
edition = "2024"

build = "build.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.8"
connectrpc-axum = "0.1"
prost = "0.14"

[build-dependencies]
connectrpc-axum-build = "0.1"
```

The ConnectRPC ecosystem for Rust is still relatively new, so expect version numbers to evolve more quickly than Axum or Tokio.

---

# Add build.rs

```rust
fn main() {
    connectrpc_axum_build::compile_protos(
        &["proto/hello.proto"],
        &["proto"],
    )
    .unwrap();
}
```

Whenever you build the project, Cargo will automatically generate Rust code from your `.proto` file.

---

# Implement the server

Create:

```rust
use axum::Router;

fn main() {
    println!("Hello, ConnectRPC!");
}
```

At this point you've verified that:

* Rust builds
* protobuf generation works
* dependencies are configured

Now replace the placeholder with your generated service implementation.

The complete implementation consists of three pieces:

1. Implement the generated `HelloService` trait.
2. Register the service with an Axum router.
3. Start the Axum HTTP server.

Those generated types and registration helpers are provided by `connectrpc-axum`, allowing one handler implementation to serve Connect, gRPC, and gRPC-Web clients from the same service definition.

A typical handler looks conceptually like:

```rust
async fn say_hello(request: HelloRequest) -> HelloResponse {
    HelloResponse {
        message: format!("Hello, {}!", request.name),
    }
}
```

---

# Run the server

```bash
cargo run
```

You should see something similar to:

```
Listening on 127.0.0.1:3000
```

---

# Test the API

If you have a ConnectRPC client, send:

```json
{
  "name": "World"
}
```

The response will be:

```json
{
  "message": "Hello, World!"
}
```

---

# What just happened?

There are only four moving pieces:

1. **Protocol Buffer**

Defines your API contract.

2. **build.rs**

Generates Rust code from the `.proto`.

3. **Axum**

Runs the HTTP server.

4. **ConnectRPC**

Maps HTTP requests onto your strongly typed Rust service.

Unlike a traditional REST API, your schema is the source of truth instead of manually writing JSON endpoints.

---

# Why ConnectRPC?

Compared to REST:

* Strongly typed contracts
* Automatic code generation
* JSON or Protobuf
* Browser-friendly
* Easier evolution of APIs

Compared to gRPC:

* Simpler HTTP support
* Better browser compatibility
* Easier debugging
* JSON without additional gateways

---

# Where to go next

After this minimal example, the natural progression is:

* Add multiple RPC methods
* Return proper ConnectRPC errors
* Split handlers into modules
* Add PostgreSQL with SQLx
* Introduce UUID primary keys
* Generate a Rust client
* Generate a TypeScript or Flutter client
* Add authentication and authorization
* Deploy with Docker or Cloud Run

From there, you're ready to build production-grade APIs while keeping the strong typing and code generation benefits that make ConnectRPC attractive.
