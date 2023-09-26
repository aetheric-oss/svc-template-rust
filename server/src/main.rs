//! Main function starting the server and initializing dependencies.

use log::info;
use svc_template_rust::*;

/// Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env().unwrap_or_default();

    // Try to load log configuration from the provided log file.
    // Will default to stdout debug logging if the file can not be loaded.
    load_logger_config_from_file(config.log_config.as_str()).await?;

    info!("(main) Server startup.");
    // --------------------------------------------------
    // START REST SECTION
    // This section should be removed if there is no REST interface
    // --------------------------------------------------

    // Allow option to only generate the spec file to a given location
    // locally: cargo run -- --api ./out/$(PACKAGE_NAME)-openapi.json
    // or `make rust-openapi` and `make rust-validate-openapi`
    let args = Cli::parse();
    if let Some(target) = args.openapi {
        return rest::generate_openapi_spec(&target);
    }

    tokio::spawn(rest::server::rest_server(config.clone(), None));
    // --------------------------------------------------
    // END REST SECTION
    // --------------------------------------------------

    tokio::spawn(grpc::server::grpc_server(config, None)).await?;

    info!("(main) Server shutdown.");

    // Make sure all log message are written/ displayed before shutdown
    log::logger().flush();

    Ok(())
}
