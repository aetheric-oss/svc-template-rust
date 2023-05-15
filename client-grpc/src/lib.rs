#![doc = include_str!("../README.md")]

pub mod service;
pub use client::*;

pub mod client {
    //! Client Library: Client Functions, Structs, Traits
    #![allow(unused_qualifications)]
    #[cfg(not(tarpaulin_include))]
    // no_coverage: Generated file, includes functions which are not being used.
    include!("grpc.rs");

    use lib_common::log_macros;
    use tonic::async_trait;
    use tonic::transport::Channel;

    pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};
    pub use rpc_service_client::RpcServiceClient as TemplateRustClient;

    log_macros!("grpc", "app::client::template_rust");

    use lib_common::grpc_client;
    grpc_client!(TemplateRustClient);
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
            self.get_client().await?.is_ready(request).await
        }
    }
}
