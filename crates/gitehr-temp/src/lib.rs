use tonic::{Request, Response, Status};

// Use the shared gos-proto crate for generated protobuf types.
pub use gos_proto::pb;
use gos_proto::pb::{gos_server::Gos, CreatePatientReq, CreatePatientRes, HealthRes, Patient};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

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

    async fn create_patient(
        &self,
        req: Request<CreatePatientReq>,
    ) -> Result<Response<CreatePatientRes>, Status> {
        let req = req.into_inner();
        let first = req.first_name.trim();
        let last = req.last_name.trim();
        if first.is_empty() || last.is_empty() {
            return Err(Status::invalid_argument(
                "first_name and last_name are required",
            ));
        }

        // Determine storage directory from environment (matches compose.dev.yml)
        // Store each patient under <PATIENT_DATA_DIR>/<2hex>/<2hex>/<32hex-uuid>/demographics.json
        let base = std::env::var("PATIENT_DATA_DIR").unwrap_or_else(|_| "/patient_data".into());
        let data_dir = Path::new(&base);
        if let Err(e) = fs::create_dir_all(data_dir) {
            tracing::error!("failed to create data dir: {}", e);
            return Err(Status::internal("failed to create storage directory"));
        }

        // generate uuid and a 32-hex form without hyphens for directory naming
        let raw_uuid = Uuid::new_v4().to_string();
        let id = raw_uuid.replace('-', "");
        let created_at = Utc::now().to_rfc3339();

        #[derive(Serialize, Deserialize)]
        struct StoredPatient {
            first_name: String,
            last_name: String,
            created_at: String,
        }

        let patient = StoredPatient {
            first_name: first.to_string(),
            last_name: last.to_string(),
            created_at: created_at.clone(),
        };

        // shard into two-level hex dirs from first 4 chars of the 32-char id
        let id_lower = id.to_lowercase();
        let s1 = &id_lower[0..2];
        let s2 = &id_lower[2..4];
        let patient_dir = data_dir.join(s1).join(s2).join(&id_lower);
        if let Err(e) = fs::create_dir_all(&patient_dir) {
            tracing::error!("failed to create patient dir: {}", e);
            return Err(Status::internal("failed to create patient directory"));
        }

        let filename = patient_dir.join("demographics.json");
        match serde_json::to_string_pretty(&patient) {
            Ok(json) => {
                if let Err(e) = fs::write(&filename, json) {
                    tracing::error!("failed to write patient file: {}", e);
                    return Err(Status::internal("failed to write patient file"));
                }
            }
            Err(e) => {
                tracing::error!("failed to serialize patient: {}", e);
                return Err(Status::internal("failed to serialize patient"));
            }
        }

        let resp = CreatePatientRes {
            filename: filename.display().to_string(),
            patient: Some(Patient {
                id,
                first_name: first.to_string(),
                last_name: last.to_string(),
                created_at,
            }),
        };

        Ok(Response::new(resp))
    }
}

// Re-export the service type for consumers
pub use GosService as GitehrTempGosService;
