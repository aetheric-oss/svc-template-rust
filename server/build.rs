//! build script to generate .rs from .proto

///generates .rs files in src directory
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_config = tonic_build::configure()
        .type_attribute("QueryIsReady", "#[derive(Eq, Copy)]")
        .type_attribute("ReadyResponse", "#[derive(Eq, Copy)]");
    let client_config = server_config.clone();

    client_config
        .build_server(false)
        .out_dir("../client-grpc/src/")
        .compile(&["../proto/svc-template-grpc.proto"], &["../proto"])?;

    // Build the Server
    server_config
        .build_client(false)
        .out_dir("src/")
        .compile(&["../proto/svc-template-grpc.proto"], &["../proto"])?;

    Ok(())
}
