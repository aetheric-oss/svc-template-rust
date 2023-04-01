//! Rest API implementations

use crate::grpc::client::GrpcClients;
use axum::{extract::Extension, Json};
use hyper::StatusCode;

/// openapi generated rest types
pub mod rest_types {
    include!("../../../openapi/types.rs");
}

// GRPC client types
// use svc_scheduler_client_grpc::grpc::{
//     ConfirmItineraryRequest, Id, Itinerary as SchedulerItinerary, QueryFlightPlan,
// };

// REST types the caller will use
pub use rest_types::ExampleRequest;

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
    Extension(mut _grpc_clients): Extension<GrpcClients>,
) -> Result<(), StatusCode> {
    rest_debug!("(health_check) entry.");

    let ok = true;

    // FIXME - uncomment this when you have a dependency
    // This health check is to verify that ALL dependencies of this
    // microservice are running.

    // let result = grpc_clients.storage.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     rest_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

    // let result = grpc_clients.pricing.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-pricing unavailable.".to_string();
    //     rest_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

    // let result = grpc_clients.scheduler.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-scheduler unavailable.".to_string();
    //     rest_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

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

    // Example request to outside GRPC client

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
