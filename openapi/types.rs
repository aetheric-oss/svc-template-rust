/// Types used for REST communication with the svc-template-rust server
use lib_common::time::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Example Request Body Information Type
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct ExampleRequest {
    /// Itinerary UUID to Cancel
    pub id: String,

    /// The time of the request
    pub timestamp: DateTime<Utc>,
}

/// Confirm itinerary Operation Status
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ExampleStatus {
    /// Unauthorized request
    #[schema(example = "Unauthorized request.")]
    Unauthorized(String),

    /// Unavailable Service
    Unavailable,
}
