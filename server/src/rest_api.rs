pub mod rest_types {
    include!("../../openapi/types.rs");
}

use crate::grpc_clients::GrpcClients;
use axum::{extract::Extension, Json};
use hyper::StatusCode;

// GRPC client types
// use svc_scheduler_client_grpc::grpc::{
//     ConfirmItineraryRequest, Id, Itinerary as SchedulerItinerary, QueryFlightPlan,
// };

// REST types the caller will use
pub use rest_types::ExampleRequest;

/// Writes an info! message to the app::req logger
macro_rules! req_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::req", $($arg)+);
    };
}

/// Writes an error! message to the app::req logger
macro_rules! req_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::req", $($arg)+);
    };
}

/// Writes a debug! message to the app::req logger
macro_rules! req_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::req", $($arg)+);
    };
}

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
    req_debug!("(health_check) entry.");

    let ok = true;

    // FIXME - uncomment this when you have a dependency
    // This health check is to verify that ALL dependencies of this
    // microservice are running.

    // let result = grpc_clients.storage.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-storage unavailable.".to_string();
    //     req_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

    // let result = grpc_clients.pricing.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-pricing unavailable.".to_string();
    //     req_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

    // let result = grpc_clients.scheduler.get_client().await;
    // if result.is_none() {
    //     let error_msg = "svc-scheduler unavailable.".to_string();
    //     req_error!("(health_check) {}", &error_msg);
    //     ok = false;
    // };

    match ok {
        true => {
            req_info!("(health_check) healthy, all dependencies running.");
            Ok(())
        }
        false => {
            req_error!("(health_check) unhealthy, 1+ dependencies down.");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

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
    req_debug!("(query_vertiports) entry.");

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
    //     req_error!("(query_vertiports) {}", &error_msg);
    //     return Err(StatusCode::SERVICE_UNAVAILABLE);
    // };

    // // Make request, process response
    // let response = client.search(request).await;
    // match response {
    //     Ok(response) => {
    //         req_info!("(example) response: {:?}", response);
    //         Ok(Json(format!("{}!", payload.id)))
    //     }
    //     Err(e) => {
    //         req_error!("(example) error: {}", e);
    //         Err(StatusCode::INTERNAL_SERVER_ERROR)
    //     }
    // }

    Ok(Json(format!("{}!", payload.id)))
}
