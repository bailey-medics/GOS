// Re-export the generated protobuf module. The generated code will be placed
// into OUT_DIR at build time by the build script.
pub mod pb {
    tonic::include_proto!("gos.v1");
}

pub use pb::*;
