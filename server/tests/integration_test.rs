//! Integration Tests

#[tokio::test]
async fn test_server_requests_and_logs() {
    use logtest::Logger;

    use svc_template_rust::grpc::server::*;

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let imp = GrpcServerImpl::default();
        let result = imp.is_ready(tonic::Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

        assert!(logger.any(|log| {
            let message = log.args();
            println!("{:?}", message);
            log.args() == format!("(is_ready) {} server MOCK.", "template_rust")
        }));
    }
}
