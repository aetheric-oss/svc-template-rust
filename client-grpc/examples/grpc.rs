//! gRPC client implementation

///module svc_template generated from svc-template-grpc.proto
// use std::time::SystemTime;
use svc_template_client_grpc::client::{template_rpc_client::TemplateRpcClient, QueryIsReady};

/// Example svc-template-client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let port = env!("GRPC_PORT");
    let mut client = TemplateRpcClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(QueryIsReady {
        // No arguments
    });

    let response = client.is_ready(request).await?;

    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}
