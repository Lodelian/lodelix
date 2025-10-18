fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "grpc")]
    tonic_prost_build::configure()
        .build_server(true)
        .compile_protos(&["proto/Status.proto", "proto/Config.proto"], &["proto"])?;

    Ok(())
}
