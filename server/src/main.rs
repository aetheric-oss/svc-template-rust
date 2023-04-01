//! Main function starting the server and initializing dependencies.

use clap::Parser;
use dotenv::dotenv;
use log::info;
use svc_template_rust::config::Config;
use svc_template_rust::grpc;
use svc_template_rust::rest;
use svc_template_rust::Cli;

/// Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Will use default config settings if no environment vars are found.
    let config = Config::from_env().unwrap_or_default();

    dotenv().ok();
    {
        let log_cfg: &str = config.log_config.as_str();
        if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
            panic!("(logger) could not parse {}. {}", log_cfg, e);
        }
    }

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

    tokio::spawn(rest::server::rest_server(config.clone()));
    // --------------------------------------------------
    // END REST SECTION
    // --------------------------------------------------

    let _ = tokio::spawn(grpc::server::grpc_server(config)).await;

    info!("Server shutdown.");
    Ok(())
}
