fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(&["proto/gos/v1/gos.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/gos/v1/gos.proto");
    println!("cargo:rerun-if-changed=proto");
    Ok(())
}
