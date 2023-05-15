#![doc = include_str!("../README.md")]

pub mod service;
pub use client::*;

use lib_common::log_macros;

#[cfg(not(any(feature = "mock_client", test)))]
pub mod client {
    //! Client Library: Client Functions, Structs, Traits
    #![allow(unused_qualifications)]
    #[cfg(not(tarpaulin_include))]
    // no_coverage: Generated file, includes functions which are not being used.
    include!("grpc.rs");

    use tonic::async_trait;
    use tonic::transport::Channel;

    pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
    pub use rpc_service_client::RpcServiceClient as TemplateRustClient;

    use lib_common::grpc_client;
    grpc_client!(TemplateRustClient);
    super::log_macros!("grpc", "app::client::template_rust");

    #[async_trait]
    impl super::service::Client<TemplateRustClient<Channel>>
        for GrpcClient<TemplateRustClient<Channel>>
    {
        type ReadyRequest = ReadyRequest;
        type ReadyResponse = ReadyResponse;

        async fn is_ready(
            &self,
            request: tonic::Request<Self::ReadyRequest>,
        ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
            grpc_info!("(is_ready) {}.", self.get_name());
            grpc_debug!("(is_ready) request: {:?}", request);
            self.get_client().await?.is_ready(request).await
        }
    }
}

#[cfg(any(feature = "mock_client", test))]
pub mod client {
    //! Client Library: Client Functions, Structs, Traits
    #![allow(unused_qualifications)]
    #[cfg(not(tarpaulin_include))]
    // no_coverage: Generated file, includes functions which are not being used.
    include!("grpc.rs");

    use tonic::async_trait;
    use tonic::transport::Channel;

    pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
    pub use rpc_service_client::RpcServiceClient as TemplateRustClient;

    use lib_common::grpc_mock_client;
    use svc_template_rust::grpc::server::{GrpcServerImpl, RpcServiceServer};
    grpc_mock_client!(TemplateRustClient, RpcServiceServer, GrpcServerImpl);
    super::log_macros!("grpc", "app::client::mock_template_rust");

    #[async_trait]
    impl super::service::Client<TemplateRustClient<Channel>>
        for GrpcClient<TemplateRustClient<Channel>>
    {
        type ReadyRequest = ReadyRequest;
        type ReadyResponse = ReadyResponse;

        async fn is_ready(
            &self,
            request: tonic::Request<Self::ReadyRequest>,
        ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
            grpc_warn!("(is_ready) {} client MOCK.", self.get_name());
            grpc_debug!("(is_ready) request: {:?}", request);
            Ok(tonic::Response::new(ReadyResponse { ready: true }))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Client as ServiceClient;

    use super::*;
    use tonic::transport::Channel;

    #[tokio::test]
    async fn test_client_connect() {
        let name = "template_rust";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<TemplateRustClient<Channel>> =
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

        let client: GrpcClient<TemplateRustClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let result = client.is_ready(tonic::Request::new(ReadyRequest {})).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner().ready, true);
    }
}
