//! Rest API implementations
/// openapi generated rest types
pub mod rest_types {
    include!("../../../openapi/types.rs");
}

pub use rest_types::*;

use crate::grpc::client::GrpcClients;
use axum::{extract::Extension, Json};
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
pub async fn health_check(
    Extension(grpc_clients): Extension<GrpcClients>,
) -> Result<(), StatusCode> {
    rest_debug!("(health_check) entry.");

    let mut ok = true;

    // FIXME - update/ uncomment this with the right dependencies.
    // This health check is to verify that ALL dependencies of this
    // microservice are running.
    if grpc_clients
        .storage
        .adsb
        .is_ready(ReadyRequest {})
        .await
        .is_err()
    {
        let error_msg = "svc-storage adsb unavailable.".to_string();
        rest_error!("(health_check) {}.", &error_msg);
        ok = false;
    }

    match ok {
        true => {
            rest_debug!("(health_check) healthy, all dependencies running.");
            Ok(())
        }
        false => {
            rest_error!("(health_check) unhealthy, 1+ dependencies down.");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Example REST API function
#[utoipa::path(
    post,
    path = "/template/example",
    tag = "svc-template-rust",
    request_body = ExampleRequest,
    responses(
        (status = 200, description = "Request successful.", body = String),
        (status = 500, description = "Request unsuccessful."),
    )
)]
pub async fn example(
    Extension(mut _grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<ExampleRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("(query_vertiports) entry.");

    // Example request to outside gRPC client

    // // Build request
    // let degree_range: f32 = 2.0;
    // let filter = AdvancedSearchFilter::search_between(
    //     "latitude".to_owned(),
    //     (payload.latitude + degree_range).to_string(),
    //     (payload.latitude - degree_range).to_string(),
    // )
    // .and_between(
    //     "longitude".to_owned(),
    //     (payload.longitude + degree_range).to_string(),
    //     (payload.longitude - degree_range).to_string(),
    // );
    // let request = tonic::Request::new(filter);

    // // Get client
    // let result = grpc_clients.storage.get_client().await;
    // let Some(mut client) = result else {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(query_vertiports) {}", &error_msg);
    //     return Err(StatusCode::SERVICE_UNAVAILABLE);
    // };

    // // Make request, process response
    // let response = client.search(request).await;
    // match response {
    //     Ok(response) => {
    //         rest_info!("(example) response: {:?}", response);
    //         Ok(Json(format!("{}!", payload.id)))
    //     }
    //     Err(e) => {
    //         rest_error!("(example) error: {}", e);
    //         Err(StatusCode::INTERNAL_SERVER_ERROR)
    //     }
    // }

    Ok(Json(format!("{}!", payload.id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check_success() {
        crate::get_log_handle().await;
        ut_info!("(test_health_check_success) Start.");

        // Mock the GrpcClients extension
        let config = crate::Config::try_from_env().unwrap_or_default();
        let grpc_clients = GrpcClients::default(config); // Replace with your own mock implementation

        // Call the health_check function
        let result = health_check(Extension(grpc_clients)).await;

        // Assert the expected result
        println!("{:?}", result);
        assert!(result.is_ok());

        ut_info!("(test_health_check_success) Success.");
    }
}
