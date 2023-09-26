//! Integration Tests
use lib_common::log_macros;
use svc_template_rust::*;

log_macros!("it", "test");

fn get_log_string(function: &str, name: &str) -> String {
    #[cfg(feature = "stub_server")]
    return format!("({} MOCK) {} server.", function, name);

    #[cfg(not(feature = "stub_server"))]
    return format!("({}) {} server.", function, name);
}

#[tokio::test]
async fn test_server_requests_and_logs() {
    use logtest::Logger;
    use svc_template_rust::grpc::server::*;

    let name = "template_rust";

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let imp = ServerImpl::default();
        let result = imp.is_ready(tonic::Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

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

/// Example integration test with logging output to test log target
#[tokio::test]
async fn it_1234() {
    it_info!("Testing server start.");

    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let (_, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(async move {
        match tonic::transport::Server::builder()
            .add_service(health_service)
            .serve_with_shutdown(
                "0.0.0.0:50051".parse().unwrap(),
                shutdown_signal("grpc", Some(shutdown_rx)),
            )
            .await
        {
            Ok(()) => it_debug!("Server started"),
            Err(e) => it_error!("Unable to start server: {}", e),
        }
    });

    // Send server the shutdown request
    shutdown_tx.send(()).expect("Could not stop server.");
    it_info!("Test success.");
}
