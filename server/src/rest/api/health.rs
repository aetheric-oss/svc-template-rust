//! Rest API implementations for health checks
/// openapi generated rest types
pub use super::rest_types::*;
use crate::grpc::client::GrpcClients;
use axum::extract::Extension;
use hyper::StatusCode;

use svc_storage_client_grpc::prelude::*;

// gRPC client types
// use svc_scheduler_client_grpc::prelude::*;
// ...

/// Provides a way to tell a caller if the service is healthy.
/// Checks dependencies, making sure all connections can be made.
#[utoipa::path(
    get,
    path = "/health",
    tag = "svc-template-rust",
    responses(
        (status = 200, description = "Service is healthy, all dependencies running."),
        (status = 503, description = "Service is unhealthy, one or more dependencies unavailable.")
    )
)]
#[axum::debug_handler]
pub async fn health_check(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<(), StatusCode> {
    rest_debug!("entry.");

    let mut ok = true;

    // This health check is to verify that ALL dependencies of this
    // microservice are running.

    ok &= grpc_clients
        .storage
        .adsb
        .is_ready(ReadyRequest {})
        .await
        .map_err(|e| {
            rest_error!("svc-storage user unavailable: {}.", e);
        })
        .is_ok();

    // others here

    if !ok {
        rest_error!("unhealthy, 1+ dependencies down.");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    rest_debug!("healthy, all dependencies running.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_success() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        // Mock the GrpcClients extension
        let config = crate::Config::default();
        let grpc_clients = GrpcClients::default(config); // Replace with your own mock implementation

        // Call the health_check function
        let result = health_check(Extension(grpc_clients)).await;

        // Assert the expected result
        println!("{:?}", result);
        assert!(result.is_ok());

        ut_info!("success");
    }
}
