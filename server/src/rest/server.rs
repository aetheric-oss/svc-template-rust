//! Rest server implementation

use super::api;
use crate::config::Config;
use crate::grpc::client::GrpcClients;
use crate::shutdown_signal;
use axum::{extract::Extension, routing, Router};

/// Starts the REST API server for this microservice
#[cfg(not(tarpaulin_include))]
pub async fn rest_server(config: Config) {
    use std::net::SocketAddr;

    rest_debug!("(rest_server) entry.");
    let grpc_clients = GrpcClients::default();
    let rest_port = config.docker_port_rest;
    let full_rest_addr: SocketAddr = match format!("[::]:{}", rest_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            rest_error!("Failed to parse REST address: {}", e);
            return;
        }
    };

    let app = Router::new()
        .route("/health", routing::get(api::health_check)) // MUST HAVE
        .route("/template/example", routing::post(api::example))
        .layer(Extension(grpc_clients)); // Extension layer must be last

    rest_info!("(rest) hosted at {:?}", full_rest_addr);
    match axum::Server::bind(&full_rest_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest"))
        .await
    {
        Ok(_) => rest_info!("REST server running at: {}.", full_rest_addr),
        Err(e) => {
            rest_error!("could not start REST server: {}", e);
        }
    };
}
