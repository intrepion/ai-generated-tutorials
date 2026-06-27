# Minimal Hello World API: Rust + ConnectRPC (Anthropic's Implementation)

**Audience:** Rust generalists who know lifetimes and cargo, but may be new to async frameworks and RPC patterns.

**Stack versions (June 2026):**
- Rust 1.88+ (MSRV; uses 2024 edition)
- connectrpc 0.7, released March 2026, pre-1.0 under Apache-2.0
- `buffa` (Anthropic's zero-copy protobuf, built-in)
- Tokio 1.40+
- Axum 0.7+ (optional; connectrpc supports both built-in hyper server and axum)

---

## Why Anthropic's ConnectRPC?

Anthropic open-sourced connectrpc, a Tower-based ConnectRPC implementation that speaks Connect, gRPC, and gRPC-Web on the same handlers, already in production at Anthropic.

This is different from the tonic/prost gRPC stack:

**Traditional gRPC** (tonic + prost):
- HTTP/2 mandatory → harder to debug with curl
- prost allocates for every string field
- Ceremony-heavy but battle-tested

**ConnectRPC** (Anthropic's connect-rust + buffa):
- HTTP/1.1 + HTTP/2 + gRPC protocol negotiation (automatic)
- Readable JSON for debugging, binary for efficiency  
- buffa offers zero-copy message views (first-class editions support) and reduces allocator CPU
- Passed all 3,600 ConnectRPC server conformance tests across Connect, gRPC, and gRPC-Web protocols

**The production reality:** Green conformance runs were necessary but far from sufficient—production issues around resource bounds (gzip bombs, unbounded decompression, TLS handshake timeouts) were surfaced only during security review.

**Confidence:** 85% on API stability (pre-1.0 means minor versions can shift), 90% on correctness (passed full conformance).

---

## Part 1: The Schema (Protocol Buffers)

Create `proto/hello.proto`:

```protobuf
syntax = "proto3";

package hello.v1;

// Greeting service: learns your name, says hello.
service GreetService {
  rpc SayHello(HelloRequest) returns (HelloReply) {}
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
  int32 request_id = 2;
}
```

**What's happening:**
- `package hello.v1` → Rust module path (hint at versioning; you can evolve to v2 later)
- `SayHello` → A unary RPC (one request, one response; not streaming)
- Messages → Serializable types with tagged fields (the `= 1` is critical for schema evolution)

**Why this matters for Rust generalists:** Protobuf is *not* JSON serialization. The numbered fields allow old clients to talk to new servers (server ignores unknown fields). That's why gRPC scales.

---

## Part 2: Generate Rust Code

Add to `Cargo.toml`:

```toml
[package]
name = "hello-connectrpc"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"

[dependencies]
connectrpc = { version = "0.7", features = ["axum"] }
tokio = { version = "1.40", features = ["macros", "rt-multi-thread", "net"] }
axum = "0.7"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[build-dependencies]
connectrpc-build = "0.7"
```

**Explanation for newcomers:**
- `connectrpc` → Anthropic's Tower-based ConnectRPC implementation
- `connectrpc-build` → Compile-time code generator (runs in `build.rs`)
- `buffa` (built-in) → Zero-copy protobuf; message types are generated views into the wire buffer
- We're adding the `axum` feature because we're hosting over HTTP

Add `build.rs` in the project root:

```rust
fn main() {
    connectrpc_build::Config::new()
        .files(&["proto/hello.proto"])
        .includes(&["proto/"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
```

**Read this:** On build, Cargo runs `build.rs` *before* compiling your crate. The `connectrpc_build::Config` generates:
1. Message types from the proto file (buffa's zero-copy views)
2. A trait for your service (trait `GreetService`)
3. A client stub (for making calls to this service)

All output goes to `OUT_DIR` and is included via `connectrpc::include_generated!()` in your code. Buffa generates one `.rs` file per proto package, with optional companion files for complex oneof/view structures.

Run:

```bash
cargo build
```

The generated code lands in `src/gen/` (or wherever the build script points). For now, trust it exists. We'll use it next.

---

## Part 3: Implement the Server

Create `src/main.rs`:

```rust
use connectrpc::{RequestContext, Response, ResponseHeader, ResponseTrailer, Router, ServiceRequest, ServiceResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// Include generated code: message types + service trait from proto/hello.proto
pub mod proto {
    connectrpc::include_generated!();
}

use proto::hello::v1::{GreetService, HelloRequest, HelloReply};

// Application state (shared across requests)
#[derive(Clone)]
struct AppState {
    request_count: Arc<RwLock<i32>>,
}

// Implement the generated GreetService trait
#[derive(Clone)]
struct GreetServiceImpl {
    state: AppState,
}

impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        // Increment counter (req.name is a &str borrowed from request buffer—no allocation!)
        let mut count = self.state.request_count.write().await;
        *count += 1;
        let request_id = *count;

        // Build response
        let message = format!("Hello, {}!", req.name);
        
        info!(
            request_id,
            name = req.name,
            "Processed greeting request"
        );

        Ok(Response::new(HelloReply {
            message,
            request_id,
            ..Default::default()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let state = AppState {
        request_count: Arc::new(RwLock::new(0)),
    };

    let service = Arc::new(GreetServiceImpl {
        state: state.clone(),
    });

    // Register the service with the router
    let router = service.register(Router::new());

    // Convert to Axum app
    let app = router.into_axum_router();

    // Bind and serve
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server listening on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;
    Ok(())
}
```

**Key insights for Rust generalists:**

1. **The trait comes from the proto:** `GreetService` is generated. You implement it. That's the contract.

2. **`ServiceRequest<'_, T>` is a view:** It borrows from the request buffer, so `req.name` is `&str` without allocation. String fields are zero-copy—this is buffa's superpower.

3. **`RequestContext` carries metadata:** Headers, deadlines, peer certs, negotiated protocol. You read it but don't modify it.

4. **`Response<B>` is the happy path:** Wrap your message in `Response::new(msg)` and return `Ok(Response)`. For errors, return `Err(ConnectError)`.

5. **`.register()` and `.into_axum_router()`:** The service trait is registered into a ConnectRPC `Router`, then lifted into Axum's type system with one method call.

---

## Part 4: Test It (Curl)

ConnectRPC automatically negotiates protocol. The route format is `/package.Service/Method`:

```bash
# JSON (easiest to debug)
curl -X POST http://localhost:8080/hello.v1.GreetService/SayHello \
  -H "content-type: application/json" \
  -d '{"name": "World"}'

# Expected response (JSON):
# {"message":"Hello, World!","requestId":1}
```

**Why this works:** ConnectRPC checks the `content-type` header. With `application/json`, it parses the request as JSON and encodes the response as JSON. Swap to `application/proto` (binary) and it uses protobuf wire format instead. Same handler, different codec. That's the beauty of negotiation.

**Verify it's working:**

```bash
# Terminal 1
cargo run

# Terminal 2
curl -X POST http://localhost:8080/hello.v1.GreetService/SayHello \
  -H "content-type: application/json" \
  -d '{"name": "Rust Generalist"}' | jq

# Output:
# {
#   "message": "Hello, Rust Generalist!",
#   "requestId": 1
# }
```

---

## Part 5: Add a Rust Client

ConnectRPC generates a client stub. Create `src/bin/client.rs`:

```rust
use connectrpc::client::ClientOptions;
use std::error::Error;

pub mod proto {
    connectrpc::include_generated!();
}

use proto::hello::v1::{greet_service_client::GreetServiceClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client (HTTP/1.1 by default)
    let options = ClientOptions::new("http://127.0.0.1:8080".parse()?);
    let client = GreetServiceClient::new(options);

    // Make a request
    let request = HelloRequest {
        name: "Rust Generalist".into(),
        ..Default::default()
    };

    let response = client.say_hello(request).await?;
    println!("Server replied: {}", response.message);
    println!("Request ID: {}", response.request_id);

    Ok(())
}
```

**How it works:**
- `GreetServiceClient` is generated from your proto service
- `ClientOptions::new()` configures the HTTP client (protocol, TLS, compression, timeouts)
- `.say_hello()` is an async method matching your proto RPC
- Errors are `Result<Response, ConnectError>`

**Run both:**

```bash
# Terminal 1
cargo run

# Terminal 2
cargo run --bin client
```

**Output:**
```
Server replied: Hello, Rust Generalist!
Request ID: 1
```

**What's happening under the hood:** The client is making an HTTP POST to `/hello.v1.GreetService/SayHello` with a JSON body (by default). ConnectRPC negotiates the protocol with the server. If the server doesn't support JSON, it can negotiate to binary protobuf or gRPC.

---

## Part 6: Understanding the Async Pattern

This is where Rust generalists often get tripped up. Let's unpack:

```rust
#[tokio::main]
async fn main() { }
```

**What's happening:**
- `#[tokio::main]` is a macro that creates a Tokio runtime (the async executor)
- Without it, `async fn` is just a function that *returns a Future*—it doesn't run anything
- The macro sets up the runtime, runs your async code, and cleans up

**In your service handler:**

```rust
async fn say_hello(&self, req: HelloRequest) -> Result<HelloReply, connect::Error>
```

The `async` keyword means:
- Inside this function, you can use `await` to pause and resume
- When you `await` on something (like a lock or network call), the runtime can run *other tasks* on the same thread
- This is why Rust can handle thousands of concurrent requests on a few OS threads

**Why it matters:** Unlike Go (which spawns lightweight goroutines), Rust forces you to think about concurrency explicitly. The `.await` site is where you're *allowing* other tasks to run. That's a feature, not a bug—it makes reasoning about race conditions easier.

---

## Part 7: Understanding the Generated Code and Buffa Views

After `cargo build`, generated code lands in `OUT_DIR` and is included via `connectrpc::include_generated!()`. You'll see:

```rust
// Generated message types (buffa, zero-copy views)
#[derive(Clone, Debug)]
pub struct HelloRequest<'a> {
    name: &'a str,  // ← Borrows from the wire buffer! No allocation.
}

impl<'a> HelloRequest<'a> {
    pub fn name(&self) -> &'a str { self.name }
    pub fn set_name(&mut self, value: impl Into<String>) { /* ... */ }
    
    // Decode from bytes (zero-copy parse)
    pub fn decode(buf: &'a [u8]) -> Result<Self, DecodeError> { /* ... */ }
}

// Generated service trait
pub trait GreetService: Send + Sync {
    async fn say_hello(
        &self,
        ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply>;
}

// Generated client
pub mod greet_service_client {
    pub struct GreetServiceClient { /* ... */ }
    impl GreetServiceClient {
        pub async fn say_hello(&self, req: HelloRequest) -> Result<HelloReply, ConnectError> { /* ... */ }
    }
}
```

**Key insights:**

1. **`HelloRequest<'a>` is a view:** It borrows from the wire buffer. When you call `req.name`, there's no heap allocation. The `&'a str` is valid as long as the request buffer exists.

2. **This is why buffa exists:** Traditional protobuf (prost) copies all strings into `String` types. For a high-concurrency service decoding many string-heavy messages, allocation overhead matters. Benchmarks show 33% throughput improvement on string-heavy payloads.

3. **When to use `.to_owned_message()`:** If you need to move data into `tokio::spawn` or hold it past the request, call `.to_owned_message()` to get an owned struct:

```rust
impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        // req.name is &str (borrowed from request buffer)
        
        // Move into a spawned task? Convert to owned:
        let owned_req = req.to_owned_message();
        tokio::spawn(async move {
            println!("{}", owned_req.name); // Now owned, safe to hold
        });
        
        // Return from handler without `.to_owned()`? Just use the borrowed &str:
        Ok(Response::new(HelloReply {
            message: format!("Hello, {}!", req.name), // Borrows are fine here
            ..Default::default()
        }))
    }
}
```

4. **`.set_name()` allocates:** Views are read-only by default. To build a response, construct the owned message:

```rust
let mut reply = HelloReply {
    message: "Hello!".into(),
    request_id: 1,
    ..Default::default()
};
reply.set_message("Updated".into()); // This allocates if needed
```

---

## Part 8: Where Rust Generalists Stumble with ConnectRPC

### Confusing borrowed views with owned messages

```rust
// ❌ This compiles but surprises people
impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        let name = req.name; // &str (borrowed)
        tokio::spawn(async move {
            println!("{}", name); // ✓ This works; &str is Copy
        });
        Ok(Response::new(HelloReply {
            message: format!("Hello, {}!", name),
            ..Default::default()
        }))
    }
}

// ❌ But this doesn't compile
impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        tokio::spawn(async move {
            println!("{}", req.name); // ✗ Can't move `req` into spawn; borrows don't live that long
        });
        Ok(Response::new(HelloReply { /* ... */ }))
    }
}

// ✓ Fix: convert to owned when spawning
impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        let owned = req.to_owned_message();
        tokio::spawn(async move {
            println!("{}", owned.name); // ✓ Now owned, safe in spawn
        });
        Ok(Response::new(HelloReply { /* ... */ }))
    }
}
```

**Why it matters:** Buffa's views are memory-efficient *because* they borrow from the wire buffer. The downside: you can't hold them across async boundaries without converting to owned. This is intentional—it forces you to think about when allocation happens.

### `RequestContext` is immutable

```rust
// ❌ This doesn't work
async fn say_hello(
    &self,
    mut ctx: RequestContext,  // ← RequestContext is not mutable
    req: ServiceRequest<'_, HelloRequest>,
) -> ServiceResult<HelloReply> {
    // Can't modify ctx
    Ok(Response::new(HelloReply { /* ... */ }))
}

// ✓ Use extensions for state passthrough
async fn say_hello(
    &self,
    ctx: RequestContext,
    req: ServiceRequest<'_, HelloRequest>,
) -> ServiceResult<HelloReply> {
    let user_id = ctx.extensions().get::<UserId>()?;  // Read from extensions
    Ok(Response::new(HelloReply { /* ... */ }))
}
```

**Why:** `RequestContext` is read-only for good reason: it represents the incoming request state, which should not be modified in the handler. Use `ctx.extensions()` to read middleware-injected state (auth, rate limit info, etc.).

### Protocol negotiation surprises

```rust
// Both of these work with the same handler:

// JSON negotiation
curl -X POST http://localhost:8080/hello.v1.GreetService/SayHello \
  -H "content-type: application/json" \
  -d '{"name":"World"}'

// Binary protobuf negotiation
curl -X POST http://localhost:8080/hello.v1.GreetService/SayHello \
  -H "content-type: application/proto" \
  --data-binary @request.bin
```

**Gotcha:** ConnectRPC auto-negotiates, but curl with binary data requires `--data-binary` and pre-encoded protobuf. For testing, stick with JSON.

### Shared mutable state in handlers

```rust
// ❌ This is racy
#[derive(Clone)]
struct BadImpl {
    counter: i32,  // Not atomic or locked
}

impl GreetService for BadImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        self.counter += 1;  // ✗ Data race under concurrent requests
        Ok(Response::new(HelloReply { /* ... */ }))
    }
}

// ✓ Use Arc<RwLock<T>> for shared mutable state
#[derive(Clone)]
struct GoodImpl {
    counter: Arc<RwLock<i32>>,
}

impl GreetService for GoodImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        let mut count = self.counter.write().await;
        *count += 1;
        Ok(Response::new(HelloReply { /* ... */ }))
    }
}
```

**Why:** ConnectRPC's Router runs handlers concurrently (one per request). Your impl needs to be `Send + Sync`. Interior mutability via `Arc<RwLock<T>>` is idiomatic. If you don't need mutation, just make your impl `Clone` with immutable fields.

---

## Part 9: Next Steps (Leveling Up)

You now have a working unary RPC over Connect + JSON. Here's where to go next:

**1. Streaming (server-streaming, client-streaming, bidirectional):**
```protobuf
service GreetService {
  rpc Greet(GreetRequest) returns (GreetResponse) {}
  rpc StreamGreetings(GreetRequest) returns (stream GreetResponse) {}
}
```

Implement streaming with `ServiceStream`:
```rust
async fn stream_greetings(
    &self,
    _ctx: RequestContext,
    req: ServiceRequest<'_, GreetRequest>,
) -> ServiceResult<ServiceStream<GreetResponse>> {
    let stream = futures::stream::iter(vec![
        GreetResponse { greeting: "Hello".into(), ..Default::default() },
        GreetResponse { greeting: "Welcome".into(), ..Default::default() },
    ]);
    Ok(Response::ok(ServiceStream::new(Box::pin(stream))))
}
```

**2. Error handling with ConnectError:**
```rust
use connectrpc::ConnectError;

async fn say_hello(
    &self,
    _ctx: RequestContext,
    req: ServiceRequest<'_, HelloRequest>,
) -> ServiceResult<HelloReply> {
    if req.name.is_empty() {
        return Err(ConnectError::invalid_argument("name cannot be empty"));
    }
    Ok(Response::new(HelloReply { /* ... */ }))
}
```

**3. Tower middleware (logging, auth, rate limiting):**
```rust
use tower::Layer;
use axum::middleware;

let app = router
    .into_axum_router()
    .layer(middleware::from_fn(my_logging_middleware));
```

Middleware can read/inject `http::Extensions` into the `RequestContext`.

**4. TLS (mTLS for client auth):**
```toml
connectrpc = { version = "0.7", features = ["axum", "server-tls", "client-tls"] }
```

See the [`examples/tls`](https://github.com/anthropics/connect-rust/tree/main/examples) directory for certificate setup.

**5. Health checking (liveness/readiness probes):**
```toml
connectrpc-health = "0.7"
```

Register the standard `grpc.health.v1.Health` service:
```rust
use connectrpc_health::{HealthService, ServingStatus};
let health = Arc::new(HealthService::new());
let router = health.register(router);
```

**6. Server reflection (grpcurl, grpcui, Postman):**
```toml
connectrpc-reflection = "0.7"
```

Allows schema-aware clients to introspect your service without knowing the proto upfront.

**7. Testing with `connectrpc`:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use connectrpc::client::ClientOptions;

    #[tokio::test]
    async fn test_greet() {
        let impl_ = Arc::new(GreetServiceImpl { /* ... */ });
        let router = impl_.register(Router::new());
        let app = router.into_axum_router();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(axum::serve(listener, app));

        let client = GreetServiceClient::new(
            ClientOptions::new(format!("http://{}", addr).parse().unwrap())
        );
        let response = client.say_hello(HelloRequest { /* ... */ }).await.unwrap();
        assert_eq!(response.message, "Hello!");
    }
}
```

---

## Part 10: Honest Assessment

**This is great for:**
- Learning RPC fundamentals with modern protocol negotiation
- High-concurrency services where string allocation overhead matters (buffa's zero-copy views)
- Multi-language interop (Connect, gRPC, and gRPC-Web all work on same handlers)
- Anthropic-backed Rust (already in production there)

**This has friction:**
- Pre-1.0, so minor versions can introduce breaking changes (API is settling)
- Zero-copy views require thinking about lifetimes more carefully (`ServiceRequest<'_, T>` borrows)
- Protocol negotiation adds complexity; for simple internal services, REST or gRPC might be overkill
- Community is smaller than tonic/gRPC ecosystem (fewer tutorials, fewer third-party integrations)
- Buffa's benefit only pays off at scale (high concurrency, string-heavy payloads); for simple services, prost + tonic is faster to iterate

**Production readiness:**
- Passed full conformance suite (3,600 ConnectRPC tests + 1,514 TLS tests)
- But: conformance exercises protocol correctness, not resource bounds. Security review surfaced gzip bomb, decompression bomb, and TLS handshake timeout issues post-conformance. *Read the production hardening section in the official guide.*

**When to use ConnectRPC over gRPC (tonic):**
- You have Go/TypeScript services already using ConnectRPC and want Rust peer
- You need protocol negotiation (JSON for debugging, binary for production)
- String-heavy message payloads at high concurrency (buffa's zero-copy wins)
- You're comfortable with pre-1.0 libraries

**When to stick with gRPC (tonic):**
- Mature ecosystem with more examples
- Team comfort (tonic is older, more battle-tested)
- HTTP/2 performance matters more than debuggability (binary-only is fine)
- You need maximum community support

**Confidence levels:**
- ConnectRPC API stability (0.7.x): 85% (pre-1.0, but published roadmap exists)
- Correctness on wire protocol: 95% (passed conformance)
- Production patterns: 70% (Anthropic uses it, but ecosystem is young)
- Buffa zero-copy value: 75% (benchmarks are real, but premature optimization is a trap)

---

## Part 11: Full Working Example (Copy-Paste Ready)

Repository structure:
```
hello-connectrpc/
├── Cargo.toml
├── build.rs
├── proto/
│   └── hello.proto
└── src/
    ├── main.rs
    └── bin/
        └── client.rs
```

**Cargo.toml** (complete):
```toml
[package]
name = "hello-connectrpc"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"

[dependencies]
connectrpc = { version = "0.7", features = ["axum"] }
tokio = { version = "1.40", features = ["macros", "rt-multi-thread", "net"] }
axum = "0.7"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[build-dependencies]
connectrpc-build = "0.7"
```

**proto/hello.proto** (unchanged):
```protobuf
syntax = "proto3";

package hello.v1;

service GreetService {
  rpc SayHello(HelloRequest) returns (HelloReply) {}
}

message HelloRequest {
  string name = 1;
}

message HelloReply {
  string message = 1;
  int32 request_id = 2;
}
```

**build.rs** (complete):
```rust
fn main() {
    connectrpc_build::Config::new()
        .files(&["proto/hello.proto"])
        .includes(&["proto/"])
        .include_file("_connectrpc.rs")
        .compile()
        .unwrap();
}
```

**src/main.rs** (ready to run):
```rust
use connectrpc::{RequestContext, Response, Router, ServiceRequest, ServiceResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub mod proto {
    connectrpc::include_generated!();
}

use proto::hello::v1::{GreetService, HelloRequest, HelloReply};

#[derive(Clone)]
struct AppState {
    request_count: Arc<RwLock<i32>>,
}

#[derive(Clone)]
struct GreetServiceImpl {
    state: AppState,
}

impl GreetService for GreetServiceImpl {
    async fn say_hello(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, HelloRequest>,
    ) -> ServiceResult<HelloReply> {
        let mut count = self.state.request_count.write().await;
        *count += 1;
        let request_id = *count;

        let message = format!("Hello, {}!", req.name);
        
        info!(
            request_id,
            name = req.name,
            "Processed greeting request"
        );

        Ok(Response::new(HelloReply {
            message,
            request_id,
            ..Default::default()
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let state = AppState {
        request_count: Arc::new(RwLock::new(0)),
    };

    let service = Arc::new(GreetServiceImpl {
        state: state.clone(),
    });

    let router = service.register(Router::new());
    let app = router.into_axum_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("Server listening on http://127.0.0.1:8080");

    axum::serve(listener, app).await?;
    Ok(())
}
```

**src/bin/client.rs** (ready to run):
```rust
use connectrpc::client::ClientOptions;
use std::error::Error;

pub mod proto {
    connectrpc::include_generated!();
}

use proto::hello::v1::{greet_service_client::GreetServiceClient, HelloRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = ClientOptions::new("http://127.0.0.1:8080".parse()?);
    let client = GreetServiceClient::new(options);

    let request = HelloRequest {
        name: "Rust Generalist".into(),
        ..Default::default()
    };

    let response = client.say_hello(request).await?;
    println!("Server replied: {}", response.message);
    println!("Request ID: {}", response.request_id);

    Ok(())
}
```

**Run it:**
```bash
# Terminal 1: build and start server
cargo run

# Terminal 2: make a request with client
cargo run --bin client

# Terminal 3: test with curl
curl -X POST http://localhost:8080/hello.v1.GreetService/SayHello \
  -H "content-type: application/json" \
  -d '{"name":"World"}' | jq
```

**Expected output (curl):**
```json
{
  "message": "Hello, World!",
  "requestId": 2
}
```

## Takeaways for Rust Generalists

1. **Protocol negotiation is powerful:** Same handler serves Connect, gRPC, and gRPC-Web clients. JSON for debugging, binary for production. That's leverage.

2. **Buffa views are memory-efficient but require lifetime thinking:** `ServiceRequest<'_, T>` borrows from the request buffer. Call `.to_owned_message()` when you need to move data across async boundaries. This is *intentional friction*—it forces you to think about allocation.

3. **Traits are your contract:** Implement the generated trait, and ConnectRPC handles routing, serialization, error handling, and protocol negotiation. You focus on business logic.

4. **Async isn't free:** Every `.await` is a context switch. ConnectRPC runs handlers concurrently (one per request). Your impl must be `Send + Sync`; use `Arc<RwLock<T>>` for shared mutable state.

5. **Pre-1.0 means breaking changes are possible:** ConnectRPC 0.7 is production-quality at Anthropic, but API surface is still settling. Pin your versions; monitor releases.

6. **Connect ≠ gRPC performance:** ConnectRPC adds protocol negotiation overhead. For internal services where HTTP/2 is guaranteed, traditional gRPC (tonic) may be faster. But interop with Go/TypeScript Connect services is seamless.

7. **Security review surfaced resource bounds issues:** Conformance suite passed, but gzip bomb, decompression bomb, and TLS timeout issues weren't caught by protocol conformance alone. Read the production hardening guide before shipping.

---

## Further Reading

- **[ConnectRPC User Guide](https://github.com/anthropics/connect-rust/blob/main/docs/guide.md)** — Long-form, covers streaming, middleware, TLS, health checking, reflection
- **[Buffa Docs](https://github.com/anthropics/buffa)** — Zero-copy message views, editions support, performance characteristics
- **[Examples Directory](https://github.com/anthropics/connect-rust/tree/main/examples)** — Runnable end-to-end: streaming, tower middleware, TLS, multi-service, browser/wasm
- **[ConnectRPC Conformance](https://github.com/connectrpc/conformance)** — 12,800+ tests; understand what's verified and what's not
- **[Iain McGinniss's Deep Dive](https://dev.to/iainmcgin/zero-copy-protobuf-and-connectrpc-for-rust-1m3e)** — Why Anthropic built this; security lessons from conformance → production

---

**Final confidence:** 85% on this tutorial being correct and complete; 75% on whether you'll choose ConnectRPC over gRPC after building something real (depends on your ecosystem).
