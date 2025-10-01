use std::net::SocketAddr;
use tonic::transport::Server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod service;
use service::pb::gos_server::GosServer;
use service::GosService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("gos=info".parse()?))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // listen addr (env or default)
    let addr: SocketAddr = std::env::var("GOS_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50051".into())
        .parse()?;

    tracing::info!("Starting GOS gRPC on {}", addr);

    let svc = GosService::default();
    Server::builder()
        .add_service(GosServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
