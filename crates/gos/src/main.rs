//! Binary entry point for the GOS gRPC health service.
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use gos::{pb::gos_server::GosServer, GosService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive("gos=info".parse()?))
        .with(tracing_subscriber::fmt::layer())
        .init();

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
