//! #![doc = include_str!("../README.md")]

mod grpc_clients;
///module generated from proto/svc-template-rust-grpc.proto
pub mod svc_template_rust {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
}

use dotenv::dotenv;
use grpc_clients::GrpcClients;
use log::{debug, error, info, warn};
use tonic::{transport::Server, Request, Response, Status};

use svc_template_rust::template_rust_rpc_server::{TemplateRustRpc, TemplateRustRpcServer};
use svc_template_rust::{QueryIsReady, ReadyResponse};

//-------------------------------------------------------------
// START REST SECTION
// This section should be removed if there is no REST interface
// Also remove the openapi/ directory
// Also remove rest_api.rs
// Also remove axum and utoipa dependencies from Cargo.toml
//-------------------------------------------------------------
mod rest_api;
use axum::{extract::Extension, routing, Router};
use clap::Parser;
use utoipa::OpenApi;

#[derive(Parser, Debug)]
struct Cli {
    /// Target file to write the OpenAPI Spec
    #[arg(long)]
    openapi: Option<String>,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // rest_api::query_flight
    ),
    components(
        schemas(
            rest_api::rest_types::ExampleRequest
        )
    ),
    tags(
        (name = "svc-template-rust", description = "svc-template-rust REST API")
    )
)]
struct ApiDoc;

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
///
/// # Examples
///
/// ```
/// Server::bind(&"0.0.0.0:8000".parse().unwrap())
/// .serve(app.into_make_service())
/// .with_graceful_shutdown(shutdown_signal())
/// .await
/// .unwrap();
/// ```
#[cfg(not(tarpaulin_include))]
async fn shutdown_signal(server: &str) {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    warn!("({}) shutdown signal", server);
}

/// Starts the REST API server for this microservice
#[cfg(not(tarpaulin_include))]
pub async fn rest_server(grpc_clients: GrpcClients) {
    debug!("(rest_server) entry.");
    let rest_port = std::env::var("DOCKER_PORT_REST")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let app = Router::new()
        .route("/health", routing::get(rest_api::health_check)) // MUST HAVE
        .route("/template/example", routing::post(rest_api::example))
        .layer(Extension(grpc_clients)); // Extension layer must be last

    let address = format!("[::]:{rest_port}").parse().unwrap();
    info!("(rest) hosted at {:?}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest"))
        .await
        .unwrap();
}

/// Create OpenAPI3 Specification File
fn generate_openapi_spec(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = ApiDoc::openapi()
        .to_pretty_json()
        .expect("(ERROR) unable to write openapi specification to json.");

    std::fs::write(target, output).expect("(ERROR) unable to write json string to file.");

    Ok(())
}

//-----------------------------------------------------------
// END REST SECTION
//-----------------------------------------------------------

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct TemplateRustImpl {}

#[tonic::async_trait]
impl TemplateRustRpc for TemplateRustImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<QueryIsReady>,
    ) -> Result<Response<ReadyResponse>, Status> {
        debug!("(grpc is_ready) entry.");
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

#[cfg(not(tarpaulin_include))]
async fn grpc_server() {
    debug!("(grpc_server) entry.");

    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let Ok(full_grpc_addr) = format!("[::]:{}", grpc_port).parse() else {
        error!("Failed to parse gRPC address");
        return;
    };

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    let imp = TemplateRustImpl::default();
    health_reporter
        .set_serving::<TemplateRustRpcServer<TemplateRustImpl>>()
        .await;

    //start server
    info!("Starting gRPC server at: {}", full_grpc_addr);
    let _ = Server::builder()
        .add_service(health_service)
        .add_service(TemplateRustRpcServer::new(imp))
        .serve(full_grpc_addr)
        .await;
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
#[cfg(not(tarpaulin_include))]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    {
        let log_cfg: &str = "log4rs.yaml";
        if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
            error!("(logger) could not parse {}. {}", log_cfg, e);
            panic!();
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
        return generate_openapi_spec(&target);
    }

    let grpc_clients = GrpcClients::default();
    tokio::spawn(rest_server(grpc_clients));

    // --------------------------------------------------
    // END REST SECTION
    // --------------------------------------------------

    let _ = tokio::spawn(grpc_server()).await;

    Ok(())
}
