//! gRPC client implementation

use std::env;
#[allow(unused_qualifications, missing_docs)]
use svc_template_rust_client_grpc::client::{rpc_service_client::RpcServiceClient, ReadyRequest};

/// Provide endpoint url to use
pub fn get_grpc_endpoint() -> String {
    //parse socket address from env variable or take default value
    let address = match env::var("SERVER_HOSTNAME") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(), // default value
    };

    let port = match env::var("SERVER_PORT_GRPC") {
        Ok(val) => val,
        Err(_) => "50051".to_string(), // default value
    };

    format!("http://{}:{}", address, port)
}

/// Example svc-template-client-grpc
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_endpoint = get_grpc_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    let mut client = RpcServiceClient::connect(grpc_endpoint).await?;

    println!("Client created");

    let response = client
        .is_ready(tonic::Request::new(ReadyRequest {}))
        .await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
