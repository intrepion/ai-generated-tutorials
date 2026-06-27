# Building a Minimal Hello World API with Rust, Axum, and ConnectRPC

If you're looking to build a type-safe, contract-driven API in Rust, the combination of **Axum** and **ConnectRPC** is a powerful choice. ConnectRPC is a modern RPC framework that serves Connect, gRPC, and gRPC-Web clients over HTTP with binary or JSON protobuf messages. In this tutorial, we'll build the most minimal "Hello World" API using the latest versions of these tools.

## What We're Using

- **Rust** 1.88+ (MSRV for connectrpc)
- **Axum** 0.8.x — a web framework focused on ergonomics and modularity
- **connectrpc** 0.6.x — a Tower-based Rust implementation of ConnectRPC
- **protoc-gen-connect-rust** — the protoc plugin that generates service traits and clients

## Prerequisites

Before we start, make sure you have:

- Rust installed (1.88 or later)
- `protoc` (the Protocol Buffer compiler) installed on your system
- A basic understanding of Protocol Buffers and async Rust

## Step 1: Create a New Project

Let's start by creating a new Rust binary project:

```bash
cargo new hello-connect --edition 2024
cd hello-connect
```

## Step 2: Add Dependencies

Open `Cargo.toml` and add the following dependencies:

```toml
[package]
name = "hello-connect"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
connectrpc = "0.6"
connectrpc-axum = "0.0.7"    # Axum integration for ConnectRPC
http = "1"
bytes = "1"

[build-dependencies]
connectrpc-build = "0.6"     # Build-time code generation
```

> **Note:** connectrpc is currently pre-1.0, but it's production-quality and passes the full ConnectRPC conformance suite.

## Step 3: Create the Protocol Buffer Definition

Create a `proto` directory and add a `hello.proto` file:

```protobuf
syntax = "proto3";

package hello;

service HelloWorldService {
    rpc SayHello(HelloRequest) returns (HelloResponse);
}

message HelloRequest {
    string name = 1;
}

message HelloResponse {
    string message = 1;
}
```

This defines a simple unary RPC: you send a name, and you get a greeting back.

## Step 4: Set Up Code Generation

Create a `build.rs` file in your project root:

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell cargo to rerun this script if the proto file changes
    println!("cargo:rerun-if-changed=proto/hello.proto");

    // Generate Rust code from the proto definition
    connectrpc_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/hello.proto"], &["proto"])?;

    Ok(())
}
```

This will generate the Rust types and service traits at compile time.

## Step 5: Write the Server

Now let's write the actual server code in `src/main.rs`:

```rust
use axum::Router;
use connectrpc_axum::ConnectRequest;
use std::net::SocketAddr;

// Include the generated protobuf code
mod hello {
    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
}

// Import the generated service trait
use hello::hello_world_service_server::HelloWorldService;

// Our handler implementation
#[derive(Clone)]
struct HelloWorldServer;

impl HelloWorldService for HelloWorldServer {
    async fn say_hello(
        &self,
        _req: ConnectRequest<hello::HelloRequest>,
    ) -> Result<hello::HelloResponse, connectrpc::Error> {
        let name = _req.into_inner().name;
        let greeting = format!("Hello, {}!", if name.is_empty() { "World" } else { &name });
        
        Ok(hello::HelloResponse {
            message: greeting,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an instance of our service
    let service = HelloWorldServer;

    // Build the router with the ConnectRPC service
    let router = Router::new()
        .merge(
            hello::hello_world_service_server::HelloWorldServiceServer::new(service)
                .into_router(),
        );

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
```

Let's break down what's happening here:

1. **Generated Code**: The `build.rs` script generates a `hello` module containing our protobuf types and service traits.

2. **Service Implementation**: We define a `HelloWorldServer` struct and implement the `HelloWorldService` trait. The `say_hello` method takes a `ConnectRequest<HelloRequest>` and returns a `Result<HelloResponse, connectrpc::Error>`.

3. **Router Setup**: We create an Axum `Router` and merge in the ConnectRPC service router. The `into_router()` method handles all the routing details for us.

4. **Server**: We bind to port 3000 and serve the router using Axum's server.

## Step 6: Run the Server

Build and run your server:

```bash
cargo run
```

You should see:
```
Server listening on http://127.0.0.1:3000
```

## Step 7: Test the API

You can test your API using `curl` with a JSON payload (ConnectRPC supports both JSON and binary protobuf):

```bash
curl -X POST http://localhost:3000/hello.HelloWorldService/SayHello \
  -H "Content-Type: application/json" \
  -d '{"name": "Rustacean"}'
```

You should get back:
```json
{"message":"Hello, Rustacean!"}
```

If you omit the name or send an empty string:
```bash
curl -X POST http://localhost:3000/hello.HelloWorldService/SayHello \
  -H "Content-Type: application/json" \
  -d '{"name": ""}'
```

You'll get:
```json
{"message":"Hello, World!"}
```

## What Makes This "Minimal"?

This implementation is minimal in several ways:

- **No manual routing** — the generated service handles all path mapping
- **No manual serialization** — protobuf messages are handled automatically
- **No middleware** — we're keeping it simple, but ConnectRPC integrates seamlessly with Tower middleware when you need it
- **Single RPC** — just one unary method, nothing more

## Adding a Simple Test Client

If you want to test from Rust as well, here's a minimal client (add this to a separate binary or an integration test):

```rust
use connectrpc::Client;
use http::Uri;

#[tokio::test]
async fn test_say_hello() {
    let client = hello::hello_world_service_client::HelloWorldServiceClient::new(
        "http://localhost:3000".parse::<Uri>().unwrap(),
    );

    let request = hello::HelloRequest {
        name: "World".to_string(),
    };

    let response = client
        .say_hello(connectrpc::Request::new(request))
        .await
        .unwrap();

    assert_eq!(response.into_inner().message, "Hello, World!");
}
```

## What's Next?

From here, you can:

- **Add more RPC methods** — just extend your proto file and implement the new methods
- **Add Axum extractors** — use any Axum extractor before the `ConnectRequest` body
- **Add streaming** — ConnectRPC supports server streaming out of the box
- **Add Tower middleware** — for logging, tracing, rate limiting, etc.
- **Enable gRPC compatibility** — serve Connect and gRPC on the same port with the `tonic` feature flag

## Troubleshooting

**"protoc not found"** — Make sure you have Protocol Buffers installed. On macOS: `brew install protobuf`. On Ubuntu: `apt-get install protobuf-compiler`.

**Compilation errors about missing types** — Make sure your `build.rs` is running and the `OUT_DIR` is correctly set. The generated code should appear in `target/debug/build/hello-connect-*/out/`.

**Connection refused** — Ensure your server is running and you're using the correct port.

## Conclusion

In about 50 lines of code (plus the proto definition), we've built a fully type-safe RPC API with Rust, Axum, and ConnectRPC. The combination gives you:

- **Type safety** — compile-time guarantees that your server and client match
- **Performance** — built on Axum's high-performance stack (hyper, tokio, tower)
- **Flexibility** — support for Connect, gRPC, and gRPC-Web clients
- **Ergonomics** — idiomatic Rust with Axum's familiar patterns

The code is available as a complete project — you can clone it and start building your own ConnectRPC services today!

*Happy coding! 🦀*
