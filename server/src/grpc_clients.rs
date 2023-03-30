// FIXME import other microservices' GRPC clients
// pub use svc_storage_client_grpc::adsb::rpc_service_client::RpcServiceClient as AdsbClient;

use futures::lock::Mutex;
use std::sync::Arc;
pub use tonic::transport::Channel;

/// Writes an info! message to the app::grpc logger
macro_rules! grpc_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::grpc", $($arg)+);
    };
}

/// Writes an error! message to the app::grpc logger
macro_rules! grpc_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::grpc", $($arg)+);
    };
}

/// Writes a debug! message to the app::grpc logger
macro_rules! grpc_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::grpc", $($arg)+);
    };
}

#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct GrpcClients {
    // FIXME clients here
    // pub adsb: GrpcClient<AdsbClient<Channel>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GrpcClient<T> {
    inner: Arc<Mutex<Option<T>>>,
    address: String,
}

/// Returns a string in http://host:port format from provided
/// environment variables
/// #[allow(dead_code)]
#[allow(dead_code)]
fn get_grpc_endpoint(env_host: &str, env_port: &str) -> String {
    grpc_debug!("(get_grpc_endpoint) entry");
    let port = match std::env::var(env_port) {
        Ok(s) => s,
        Err(_) => {
            grpc_error!("(env) {} undefined.", env_port);
            "".to_string()
        }
    };
    let host = match std::env::var(env_host) {
        Ok(s) => s,
        Err(_) => {
            grpc_error!("(env) {} undefined.", env_host);
            "".to_string()
        }
    };

    let full = format!("http://{host}:{port}");
    grpc_info!("(get_grpc_endpoint) full address: {}", full);
    full
}

impl<T> GrpcClient<T> {
    #[allow(dead_code)]
    pub async fn invalidate(&mut self) {
        let arc = Arc::clone(&self.inner);
        let mut client = arc.lock().await;
        *client = None;
    }

    #[allow(dead_code)]
    pub fn new(env_host: &str, env_port: &str) -> Self {
        let opt: Option<T> = None;
        GrpcClient {
            inner: Arc::new(Mutex::new(opt)),
            address: get_grpc_endpoint(env_host, env_port),
        }
    }
}

#[allow(unused_macros)]
macro_rules! grpc_client {
    ( $client: ident, $name: expr ) => {
        impl GrpcClient<$client<Channel>> {
            pub async fn get_client(&mut self) -> Option<$client<Channel>> {
                grpc_debug!("(get_client) storage::{} entry", $name);

                let arc = Arc::clone(&self.inner);

                // if already connected, return the client
                let client = arc.lock().await;
                if client.is_some() {
                    return client.clone();
                }

                grpc_debug!(
                    "(grpc) connecting to {} server at {}",
                    $name,
                    self.address.clone()
                );
                let result = $client::connect(self.address.clone()).await;
                match result {
                    Ok(client) => {
                        grpc_info!(
                            "(grpc) success: connected to {} server at {}",
                            $name,
                            self.address.clone()
                        );
                        Some(client)
                    }
                    Err(e) => {
                        grpc_error!(
                            "(grpc) couldn't connect to {} server at {}; {}",
                            $name,
                            self.address,
                            e
                        );
                        None
                    }
                }
            }
        }
    };
}

// FIXME - add other clients here
// grpc_client!(AdsbClient, "adsb");

impl GrpcClients {
    pub fn default() -> Self {
        GrpcClients {
            // FIXME - add other clients here
            // adsb: GrpcClient::<AdsbClient<Channel>>::new("STORAGE_HOST_GRPC", "STORAGE_PORT_GRPC"),
        }
    }
}
