//! gRPC server implementation

///module generated from proto/svc-template-rust-grpc.proto
pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}
pub use grpc_server::rpc_service_server::RpcServiceServer;

use grpc_server::rpc_service_server::RpcService;
use grpc_server::{ReadyRequest, ReadyResponse};

use crate::config::Config;
use crate::shutdown_signal;

use std::fmt::Debug;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// struct to implement the gRPC server functions
#[derive(Debug, Default, Copy, Clone)]
pub struct GrpcServerImpl {}

#[tonic::async_trait]
#[cfg(not(any(feature = "mock_server", test)))]
impl RpcService for GrpcServerImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_info!("(is_ready) template_rust.");
        grpc_debug!("(is_ready) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}

/// Starts the grpc servers for this microservice using the provided configuration
///
/// # Example:
/// ```
/// use svc_template_rust::grpc::server::grpc_server;
/// use svc_template_rust::config::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(grpc_server(config)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn grpc_server(config: Config) {
    grpc_debug!("(grpc_server) entry.");

    // Grpc Server
    let grpc_port = config.docker_port_grpc;
    let full_grpc_addr: SocketAddr = match format!("[::]:{}", grpc_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            grpc_error!("Failed to parse gRPC address: {}", e);
            return;
        }
    };

    let imp = GrpcServerImpl::default();
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<RpcServiceServer<GrpcServerImpl>>()
        .await;

    //start server
    grpc_info!("Starting gRPC services on: {}.", full_grpc_addr);
    match Server::builder()
        .add_service(health_service)
        .add_service(RpcServiceServer::new(imp))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc"))
        .await
    {
        Ok(_) => grpc_info!("gRPC server running at: {}.", full_grpc_addr),
        Err(e) => {
            grpc_error!("could not start gRPC server: {}", e);
        }
    };
}

#[tonic::async_trait]
#[cfg(any(feature = "mock_server", test))]
impl RpcService for GrpcServerImpl {
    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_warn!("(is_ready) template_rust server MOCK.");
        grpc_debug!("(is_ready) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }
}
