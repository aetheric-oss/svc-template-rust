//! Integration Tests

#[tokio::test]
async fn test_client_requests_and_logs() {
    use logtest::Logger;

    use svc_template_rust_client_grpc::service::Client as ServiceClient;
    use svc_template_rust_client_grpc::*;
    use tonic::transport::Channel;

    let name = "template_rust";
    let (server_host, server_port) =
        lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

    let client: GrpcClient<TemplateRustClient<Channel>> =
        GrpcClient::new_client(&server_host, server_port, name);

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let result = client.is_ready(tonic::Request::new(ReadyRequest {})).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        // Search for the expected log message
        assert!(logger.any(|log| {
            let message = log.args();
            println!("{:?}", message);
            log.args() == format!("(is_ready) {} client MOCK.", name)
        }));
    }
}
