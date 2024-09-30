fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_transport(true)
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile_protos(&["proto/photonic.proto"], &["proto/"])?;

    return Ok(());
}
