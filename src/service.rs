//! Service implementation for the GOS gRPC health check service.
//!
//! This module contains the core service logic for handling health check requests.

use tonic::{Request, Response, Status};

/// Generated protobuf code for the GOS service.
///
/// This module contains the auto-generated gRPC service definitions
/// and message types from the protobuf schema defined in `proto/gos/v1/gos.proto`.
pub mod pb {
    tonic::include_proto!("gos.v1"); // package gos.v1;
}

use pb::{gos_server::Gos, HealthRes};

/// A gRPC service implementation for GOS health checking.
///
/// This service provides health status information and can be used
/// for service discovery, load balancer health checks, and monitoring purposes.
/// The service is stateless and can be safely cloned.
#[derive(Default, Clone)]
pub struct GosService;

#[tonic::async_trait]
impl Gos for GosService {
    /// Checks the health status of the GOS service.
    ///
    /// This endpoint always returns a successful response indicating that
    /// the service is alive and ready to handle requests. It can be used
    /// by load balancers, monitoring systems, and orchestrators to determine
    /// if the service instance is healthy.
    ///
    /// # Arguments
    ///
    /// * `_req` - An empty request (unit type representing `google.protobuf.Empty`)
    ///
    /// # Returns
    ///
    /// Returns a `HealthRes` response containing:
    /// * `ok: true` - Indicates the service is healthy
    /// * `message: "GOS is alive"` - A human-readable status message
    ///
    /// # Errors
    ///
    /// This method currently always succeeds but returns a `Result` to conform
    /// to the gRPC service interface. Future implementations might include
    /// actual health checks that could fail.
    async fn health(&self, _req: Request<()>) -> Result<Response<HealthRes>, Status> {
        Ok(Response::new(HealthRes {
            ok: true,
            message: "GOS is alive".into(),
        }))
    }
}
