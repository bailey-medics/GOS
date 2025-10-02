use tonic::{Request, Response, Status};

pub mod pb {
    tonic::include_proto!("gos.v1");
}

use pb::{gos_server::Gos, CreatePatientReq, CreatePatientRes, HealthRes, Patient};

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

        let base = std::env::var("PATIENT_DATA_DIR").unwrap_or_else(|_| "/patient_data".into());
        let data_dir = Path::new(&base).join("patients");
        if let Err(e) = fs::create_dir_all(&data_dir) {
            tracing::error!("failed to create data dir: {}", e);
            return Err(Status::internal("failed to create storage directory"));
        }

        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        #[derive(Serialize, Deserialize)]
        struct StoredPatient {
            id: String,
            first_name: String,
            last_name: String,
            created_at: String,
        }

        let patient = StoredPatient {
            id: id.clone(),
            first_name: first.to_string(),
            last_name: last.to_string(),
            created_at: created_at.clone(),
        };

        let filename = data_dir.join(format!("{}.json", id));
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
