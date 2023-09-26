//! Integration Tests

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

    use svc_template_rust_client_grpc::prelude::*;

    let name = "template_rust";
    let (server_host, server_port) =
        lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

    let client = TemplateRustClient::new_client(&server_host, server_port, name);

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let result = client.is_ready(template_rust::ReadyRequest {}).await;
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
