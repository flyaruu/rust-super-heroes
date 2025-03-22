fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inclodes: &[String] = &[];
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile_protos(&["./proto/locationservice-v1.proto"], inclodes)?;
    Ok(())
}
