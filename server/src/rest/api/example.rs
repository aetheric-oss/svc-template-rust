//! Rest API implementations of example operations
/// openapi generated rest types
pub use super::rest_types::*;
use crate::grpc::client::GrpcClients;
use axum::{extract::Extension, Json};
use hyper::StatusCode;

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
#[axum::debug_handler]
pub async fn example(
    Extension(_grpc_clients): Extension<GrpcClients>,
    Json(payload): Json<ExampleRequest>,
) -> Result<Json<String>, StatusCode> {
    rest_debug!("entry.");

    // Example request

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

    // // Make request, process response
    // let response = grpc_clients.storage.adsb.search(request).await;
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

    Ok(Json(payload.id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_success() {
        lib_common::logger::get_log_handle().await;
        ut_info!("start");

        // Mock the GrpcClients extension
        let config = crate::Config::default();
        let grpc_clients = GrpcClients::default(config); // Replace with your own mock implementation

        // Mock the payload
        let payload = ExampleRequest {
            id: lib_common::uuid::Uuid::new_v4().to_string(),
            timestamp: lib_common::time::Utc::now(),
        };

        let id = example(Extension(grpc_clients), Json(payload.clone()))
            .await
            .unwrap()
            .0;

        assert_eq!(id, payload.id);

        ut_info!("success");
    }
}
