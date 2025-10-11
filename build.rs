fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use the API crate's proto directory as canonical for repository-level tasks.
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(
            &["crates/api/proto/vpr/v1/vpr.proto"],
            &["crates/api/proto"],
        )?;
    println!("cargo:rerun-if-changed=crates/api/proto/vpr/v1/vpr.proto");
    println!("cargo:rerun-if-changed=crates/api/proto");
    Ok(())
}
