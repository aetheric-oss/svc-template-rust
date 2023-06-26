//! Integration Tests
use lib_common::grpc::get_endpoint_from_env;
use svc_template_rust_client_grpc::service::Client as ServiceClient;
use svc_template_rust_client_grpc::*;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

pub(crate) static CLIENT: OnceCell<GrpcClient<RpcServiceClient<Channel>>> = OnceCell::const_new();

pub async fn get_client() -> &'static GrpcClient<RpcServiceClient<Channel>> {
    CLIENT
        .get_or_init(|| async move {
            let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
            GrpcClient::<RpcServiceClient<Channel>>::new_client(&host, port, "template_rust")
        })
        .await
}

fn get_log_string(function: &str, name: &str) -> String {
    #[cfg(feature = "stub_client")]
    return format!("({} MOCK) {} client.", function, name);

    #[cfg(not(feature = "stub_client"))]
    cfg_if::cfg_if! {
        if #[cfg(feature = "stub_backends")] {
            return format!("({} MOCK) {} server.", function, name);
        } else {
            return format!("({}) {} client.", function, name);
        }
    }
}

#[tokio::test]
async fn test_client_requests_and_logs() {
    use logtest::Logger;

    let name = "template_rust";
    let client = get_client().await;

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let result = client.is_ready(ReadyRequest {}).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        // Search for the expected log message
        let expected = get_log_string("is_ready", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| {
            if log.target().contains("app::") {
                println!("{}", log.target());
                let message = log.args();
                println!("{:?}", message);
                log.args() == expected
            } else {
                false
            }
        }));
    }
}
