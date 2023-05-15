//! Rest server implementation

use super::api;
use crate::config::Config;
use crate::grpc::client::GrpcClients;
use crate::shutdown_signal;
use axum::{extract::Extension, routing, Router};
use std::net::SocketAddr;

/// Starts the REST API server for this microservice
#[cfg(not(tarpaulin_include))]
pub async fn rest_server(config: Config) -> Result<(), ()> {
    rest_info!("(rest_server) entry.");
    let rest_port = config.docker_port_rest;
    let full_rest_addr: SocketAddr = match format!("[::]:{}", rest_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            rest_error!("(rest_server) invalid address: {:?}, exiting.", e);
            return Err(());
        }
    };

    //
    // Extensions
    //
    // GRPC Clients
    let grpc_clients = GrpcClients::default(config.clone());

    //
    // Create Server
    //
    let app = Router::new()
        .route("/health", routing::get(api::health_check)) // MUST HAVE
        .route("/template/example", routing::post(api::example))
        .layer(Extension(grpc_clients)); // Extension layer must be last

    //
    // Bind to address
    //
    match axum::Server::bind(&full_rest_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("rest"))
        .await
    {
        Ok(_) => {
            rest_info!("(rest_server) hosted at: {}.", full_rest_addr);
            Ok(())
        }
        Err(e) => {
            rest_error!("(rest_server) could not start server: {}", e);
            Err(())
        }
    }
}
