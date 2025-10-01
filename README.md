# GOS - gRPC Health Service

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![gRPC](https://img.shields.io/badge/gRPC-4285F4?style=for-the-badge&logo=google&logoColor=white)

A lightweight, high-performance gRPC health check service built with Rust.
Perfect for microservice architectures, container orchestration, and service discovery.

## 🚀 Features

- **Fast & Lightweight**: Built with Rust for maximum performance and minimal resource usage
- **gRPC Native**: Full gRPC support with protocol buffers
- **Health Checks**: Implements standard gRPC health checking protocol
- **Container Ready**: Docker support with minimal image size
- **Observable**: Structured logging with tracing support
- **Production Ready**: Comprehensive error handling and graceful shutdown

## 📋 Quick Start

### Prerequisites

- Rust 1.70+ (edition 2021)
- Protocol Buffers compiler (`protoc`)

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protobuf compiler
brew install protobuf  # macOS
# or apt-get install protobuf-compiler  # Ubuntu/Debian
```

### Running the Service

```bash
# Build the project
cargo build

# Start the server (default: 0.0.0.0:50051)
cargo run

# Or with custom address
GOS_ADDR=127.0.0.1:8080 cargo run
```

### Testing the Health Check

```bash
# Using grpcurl
grpcurl -plaintext \
  -import-path proto \
  -proto proto/gos/v1/gos.proto \
  -d '{}' \
  localhost:50051 gos.v1.GOS/Health
```

Expected response:

```json
{
  "ok": true,
  "message": "GOS is alive"
}
```

## 🏗️ Architecture

GOS follows a simple but robust architecture:

```text
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   gRPC Client   │───▶│   GOS Service   │───▶│   Health Logic  │
│                 │    │                 │    │                 │
│ • Load Balancer │    │ • tonic/tokio   │    │ • Status Check  │
│ • Kubernetes    │    │ • Logging       │    │ • Future: DB    │
│ • Monitor       │    │ • Metrics       │    │ • Future: Cache │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|----------|
| `GOS_ADDR` | Server bind address | `0.0.0.0:50051` |
| `RUST_LOG` | Log level configuration | `gos=info` |

### Docker Usage

```bash
# Build image
docker build -t gos .

# Run container
docker run -p 50051:50051 gos

# With custom configuration
docker run -p 8080:8080 -e GOS_ADDR=0.0.0.0:8080 gos
```

## 🛠️ Development

### Build & Test

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Run all quality checks
./scripts/check-all.sh
```

### Pre-commit Hooks

Install pre-commit hooks for automatic code quality checks:

```bash
pre-commit install
```

### gRPC Schema

The service implements the following protobuf schema:

```protobuf
syntax = "proto3";
package gos.v1;

import "google/protobuf/empty.proto";

message HealthRes {
  bool ok = 1;
  string message = 2;
}

service GOS {
  rpc Health(google.protobuf.Empty) returns (HealthRes);
}
```

## 📚 API Reference

The main components of GOS are:

- [`GosService`] - The main gRPC service implementation
- [`pb`] - Generated protobuf code and message types

## 🔍 Examples

### Custom Health Logic

```rust,no_run
use gos::{GosService, pb::{HealthRes, gos_server::Gos}};
use tonic::{Request, Response, Status};

#[derive(Default, Clone)]
pub struct CustomGosService {
    // Add your custom health check logic here
}

#[tonic::async_trait]
impl Gos for CustomGosService {
    async fn health(&self, _req: Request<()>) -> Result<Response<HealthRes>, Status> {
        // Custom health check logic
        let is_healthy = check_database().await && check_cache().await;
        
        Ok(Response::new(HealthRes {
            ok: is_healthy,
            message: if is_healthy { "All systems operational" } else { "Degraded" }.into(),
        }))
    }
}

async fn check_database() -> bool { true } // Implement your DB check
async fn check_cache() -> bool { true }    // Implement your cache check
```

## 🤝 Contributing

We welcome contributions! Please see our development guide:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Run quality checks: `./scripts/check-all.sh`
5. Commit your changes: `git commit -m 'Add amazing feature'`
6. Push to the branch: `git push origin feature/amazing-feature`
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/bailey-medics/GOS/blob/main/LICENSE) file for details.

## 🏷️ Version Information

Current version: **0.1.0**

- **Stability**: Beta
- **MSRV**: Rust 1.70+
- **gRPC Version**: Compatible with gRPC specification
