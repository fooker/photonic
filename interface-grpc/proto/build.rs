fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .build_transport(true)
        .server_mod_attribute("photonic", "#[cfg(feature = \"server\")]")
        .client_mod_attribute("photonic", "#[cfg(feature = \"client\")]")
        .compile_protos(&["proto/photonic.proto"], &["proto/"])?;

    return Ok(());
}
