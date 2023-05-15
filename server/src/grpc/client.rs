//! gRPC client helpers implementation

pub use lib_common::grpc::ClientConnect;
use lib_common::grpc::{Client, GrpcClient};
use tonic::transport::Channel;

// FIXME: import other microservices' GRPC clients instead, this is just an example.
use svc_template_rust_client_grpc::client::TemplateRustClient;
//use svc_storage_client_grpc::get_clients as get_storage_clients;
//use svc_storage_client_grpc::Clients;

/// Struct to hold all gRPC client connections
#[derive(Clone, Debug)]
pub struct GrpcClients {
    /// FIXME: add the correct clients here
    pub template_rust: GrpcClient<TemplateRustClient<Channel>>,
    //pub storage: Clients,
}

impl GrpcClients {
    /// Create new GrpcClient with defaults
    pub fn default(config: crate::config::Config) -> Self {
        let template_rust = GrpcClient::<TemplateRustClient<Channel>>::new_client(
            &config.template_rust_host_grpc,
            config.template_rust_port_grpc,
            "template_rust",
        );

        //let storage_clients = get_storage_clients(config.storage_host_grpc, config.storage_port_grpc);

        GrpcClients {
            template_rust,
            //storage: storage_clients,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grpc_client_default() {
        let config = crate::config::Config::default();
        let clients = GrpcClients::default(config);
        assert_eq!(clients.template_rust.get_name(), "template_rust")
    }
}
