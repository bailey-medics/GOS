//! Binary entry point for the GOS gRPC health service.
//!
//! This binary starts the gRPC server using the [`gos`] library.
//! For library documentation and API reference, see the main [`gos`] crate.

use std::net::SocketAddr;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use gos::{pb::gos_server::GosServer, GosService};

/// Entry point for the GOS gRPC server.
///
/// Sets up structured logging with tracing, parses the listen address from
/// the `GOS_ADDR` environment variable (defaults to `0.0.0.0:50051`),
/// and starts the gRPC server with the health service.
///
/// # Environment Variables
///
/// * `GOS_ADDR` - The address to bind the server to (optional, defaults to `0.0.0.0:50051`)
/// * `RUST_LOG` - Controls log level (optional, defaults to `gos=info`)
///
/// # Errors
///
/// Returns an error if:
/// * The address parsing fails
/// * The server fails to start
/// * Logging configuration is invalid
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive("gos=info".parse()?))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // listen addr (env or default)
    let addr: SocketAddr = std::env::var("GOS_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50051".into())
        .parse()?;

    tracing::info!("Starting GOS gRPC on {}", addr);

    let svc = GosService;
    Server::builder()
        .add_service(GosServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
