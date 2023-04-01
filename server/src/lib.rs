#![doc = include_str!("../README.md")]

use clap::Parser;

pub mod config;
pub mod grpc;

// --------------------------------------------------
// START REST SECTION
// This section should be removed if there is no REST interface
// --------------------------------------------------
/// rest implementation module
pub mod rest;

/// struct holding cli configuration options
#[derive(Parser, Debug)]
pub struct Cli {
    /// Target file to write the OpenAPI Spec
    #[arg(long)]
    pub openapi: Option<String>,
}
// --------------------------------------------------
// END REST SECTION
// --------------------------------------------------

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// This signal handler can be used in our [`axum::Server`] method `with_graceful_shutdown`
/// and in our [`tonic::transport::Server`] method `serve_with_shutdown`.
///
/// # Examples
///
/// ## axum
/// ```
/// use svc_template_rust::shutdown_signal;
/// pub async fn server() {
///     let app = axum::Router::new();
///     axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
///         .serve(app.into_make_service())
///         .with_graceful_shutdown(shutdown_signal("rest"));
/// }
/// ```
///
/// ## tonic
/// ```
/// use svc_template_rust::shutdown_signal;
/// pub async fn server() {
///     let (_, health_service) = tonic_health::server::health_reporter();
///     tonic::transport::Server::builder()
///         .add_service(health_service)
///         .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc"));
/// }
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn shutdown_signal(server: &str) {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    rest_warn!("({}) shutdown signal", server);
}
