#![doc = include_str!("../README.md")]

#[cfg(test)]
#[macro_use]
pub mod test_util;

pub mod config;
pub mod grpc;

pub use crate::config::Config;

pub use clap::Parser;
/// rest implementation module
pub mod rest;

/// struct holding cli configuration options
#[derive(Parser, Debug, Clone)]
pub struct Cli {
    /// Target file to write the OpenAPI Spec
    #[arg(long)]
    pub openapi: Option<String>,
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// This signal handler can be used in our [`axum::serve()`] method `with_graceful_shutdown`
/// and in our [`tonic::transport::Server`] method `serve_with_shutdown`.
///
/// # Examples
///
/// ## axum
/// ```
/// use svc_template_rust::shutdown_signal;
/// pub async fn server() {
///     let app = axum::Router::new();
///     let full_rest_addr: std::net::SocketAddr = "0.0.0.0:8000".parse().unwrap();
///     //
///     // Bind to address
///     //
///     let listener = tokio::net::TcpListener::bind(&full_rest_addr).await.unwrap();
///
///     //
///     // Start serving
///     //
///     axum::serve(listener, app)
///         .with_graceful_shutdown(shutdown_signal("rest", None))
///         .await.unwrap();
/// }
/// ```
///
/// ## tonic
/// ```
/// use svc_template_rust::shutdown_signal;
/// pub async fn server() {
///     let (_, health_service) = tonic_health::server::health_reporter();
///
///     tonic::transport::Server::builder()
///         .add_service(health_service)
///         .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc", None));
/// }
/// ```
///
/// ## using a shutdown signal channel
/// ```
/// use svc_template_rust::shutdown_signal;
/// pub async fn server() {
///     let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
///     let (_, health_service) = tonic_health::server::health_reporter();
///     tokio::spawn(async move {
///         tonic::transport::Server::builder()
///             .add_service(health_service)
///             .serve_with_shutdown("0.0.0.0:50051".parse().unwrap(), shutdown_signal("grpc", Some(shutdown_rx)))
///             .await;
///     });
///
///     // Send server the shutdown request
///     shutdown_tx.send(()).expect("Could not stop server.");
/// }
/// ```
pub async fn shutdown_signal(
    server: &str,
    shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>,
) {
    match shutdown_rx {
        Some(receiver) => receiver
            .await
            .expect("(shutdown_signal) expect tokio signal oneshot Receiver"),
        None => tokio::signal::ctrl_c()
            .await
            .expect("(shutdown_signal) expect tokio signal ctrl-c"),
    }

    log::warn!("(shutdown_signal) server shutdown for [{}]", server);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_shutdown() {
        ut_info!("start");

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let (_, health_service) = tonic_health::server::health_reporter();
        tokio::spawn(async move {
            let _ = tonic::transport::Server::builder()
                .add_service(health_service)
                .serve_with_shutdown(
                    "0.0.0.0:50051".parse().unwrap(),
                    shutdown_signal("grpc", Some(shutdown_rx)),
                )
                .await;
        });

        // Send server the shutdown request
        assert!(shutdown_tx.send(()).is_ok());

        ut_info!("success");
    }
}
