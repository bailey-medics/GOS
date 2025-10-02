fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the API crate's proto directory as canonical for repository-level tasks.
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &["crates/api/proto/gos/v1/gos.proto"],
            &["crates/api/proto"],
        )?;
    println!("cargo:rerun-if-changed=crates/api/proto/gos/v1/gos.proto");
    println!("cargo:rerun-if-changed=crates/api/proto");
    Ok(())
}
