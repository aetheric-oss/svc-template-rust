//! gRPC server implementation

use std::env;

#[allow(unused_qualifications)]
mod grpc;
use grpc::template_rpc_server::{TemplateRpc, TemplateRpcServer};
use grpc::{QueryIsReady, ReadyResponse};
use tonic::{transport::Server, Request, Response, Status};

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct TemplateImpl {}

#[tonic::async_trait]
impl TemplateRpc for TemplateImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<QueryIsReady>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

fn get_grpc_addr_port() -> (String, String) {
    //parse socket address from env variable or take default value
    let address = match env::var("GRPC_ADDR") {
        Ok(val) => val,
        Err(_) => "0.0.0.0".to_string(), // default value
    };

    let port = match env::var("GRPC_PORT") {
        Ok(val) => val,
        Err(_) => "8000".to_string(), // default value
    };

    (address, port)
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (grpc_address, grpc_port) = get_grpc_addr_port();
    let full_grpc_addr = format!("{grpc_address}:{grpc_port}").parse()?;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<TemplateRpcServer<TemplateImpl>>()
        .await;

    let grpc_client = TemplateImpl::default();
    //start server
    Server::builder()
        .add_service(health_service)
        .add_service(TemplateRpcServer::new(grpc_client))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
