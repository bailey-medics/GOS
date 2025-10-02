fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The API crate owns the proto files now; reference them from there so both
    // crates use the same canonical schema located in crates/api/proto.
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(&["../api/proto/gos/v1/gos.proto"], &["../api/proto"])?;
    println!("cargo:rerun-if-changed=../api/proto/gos/v1/gos.proto");
    println!("cargo:rerun-if-changed=../api/proto");
    Ok(())
}
