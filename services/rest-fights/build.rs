use tonic_build::Config;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    let inclodes: &[String] = &[];
    // prost_build::compile_protos();
    // tonic_build::compile_protos("./proto/locationservice-v1.proto")?;

    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        // .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .compile_protos(&["./proto/locationservice-v1.proto"],inclodes)?;
    Ok(())
}
