use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("gos.v1");
}

use pb::{gos_server::Gos, HealthRes};

#[derive(Default, Clone)]
pub struct GosService;

#[tonic::async_trait]
impl Gos for GosService {
    async fn health(&self, _req: Request<()>) -> Result<Response<HealthRes>, Status> {
        Ok(Response::new(HealthRes {
            ok: true,
            message: "GOS is alive".into(),
        }))
    }
}
