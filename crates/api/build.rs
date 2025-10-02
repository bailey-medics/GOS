fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the crate-local proto directory as the single source for this crate.
    // This keeps the protobuf files colocated with the API implementation.
    let crate_proto = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &[crate_proto.join("gos/v1/gos.proto")],
            &[crate_proto.as_path()],
        )?;
    println!("cargo:rerun-if-changed=proto/gos/v1/gos.proto");
    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
