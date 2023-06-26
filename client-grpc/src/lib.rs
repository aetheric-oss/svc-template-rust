#![doc = include_str!("../README.md")]

pub mod service;
pub use client::*;
pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};

use lib_common::log_macros;
use tonic::async_trait;
use tonic::transport::Channel;

pub mod client {
    //! Client Library: Client Functions, Structs, Traits
    #![allow(unused_qualifications)]
    include!("grpc.rs");

    use super::*;

    pub use rpc_service_client::RpcServiceClient;
    cfg_if::cfg_if! {
        if #[cfg(feature = "stub_backends")] {
            use svc_template_rust::grpc::server::{RpcServiceServer, ServerImpl};
            lib_common::grpc_mock_client!(RpcServiceClient, RpcServiceServer, ServerImpl);
            super::log_macros!("grpc", "app::client::mock::template_rust");
        } else {
            lib_common::grpc_client!(RpcServiceClient);
            super::log_macros!("grpc", "app::client::template_rust");
        }
    }
}

#[cfg(not(feature = "stub_client"))]
#[async_trait]
impl crate::service::Client<RpcServiceClient<Channel>> for GrpcClient<RpcServiceClient<Channel>> {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_info!("(is_ready) {} client.", self.get_name());
        grpc_debug!("(is_ready) request: {:?}", request);
        self.get_client().await?.is_ready(request).await
    }
}
#[cfg(feature = "stub_client")]
#[async_trait]
impl crate::service::Client<RpcServiceClient<Channel>> for GrpcClient<RpcServiceClient<Channel>> {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
        grpc_debug!("(is_ready MOCK) request: {:?}", request);
        Ok(tonic::Response::new(ReadyResponse { ready: true }))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Client as ServiceClient;

    use super::*;
    use tonic::transport::Channel;

    #[tokio::test]
    #[cfg(not(feature = "stub_client"))]
    async fn test_client_connect() {
        let name = "template_rust";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let connection = client.get_client().await;
        println!("{:?}", connection);
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn test_client_is_ready_request() {
        let name = "template_rust";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let result = client.is_ready(ReadyRequest {}).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner().ready, true);
    }
}
