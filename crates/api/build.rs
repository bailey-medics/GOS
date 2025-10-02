fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the repository-level proto directory as the single source of truth.
    // The crate's CARGO_MANIFEST_DIR is crates/api, so the repo root is two levels up.
    let repo_proto = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &[repo_proto.join("gos/v1/gos.proto")],
            &[repo_proto.as_path()],
        )?;
    println!("cargo:rerun-if-changed=../proto/gos/v1/gos.proto");
    println!("cargo:rerun-if-changed=../../proto");
    Ok(())
}
