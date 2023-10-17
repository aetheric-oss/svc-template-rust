//! # Config
//!
//! Define and implement config options for module

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// struct holding configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// port to be used for gRPC server
    pub docker_port_grpc: u16,
    /// port to be used for REST server
    pub docker_port_rest: u16,
    /// host of storage server
    pub storage_host_grpc: String,
    /// port of storage server
    pub storage_port_grpc: u16,
    /// path to log configuration YAML file
    pub log_config: String,
    /// Rate limit - requests per second for REST requests
    pub rest_request_limit_per_second: u8,
    /// Enforces a limit on the concurrent number of requests the underlying service can handle
    pub rest_concurrency_limit_per_service: u8,
    /// Full url (including port number) to be allowed as request origin for
    /// REST requests
    pub rest_cors_allowed_origin: String,
}

impl Default for Config {
    fn default() -> Self {
        log::warn!("(default) Creating Config object with default values.");
        Self::new()
    }
}

impl Config {
    /// Default values for Config
    pub fn new() -> Self {
        Config {
            docker_port_grpc: 50051,
            docker_port_rest: 8000,
            storage_port_grpc: 50051,
            storage_host_grpc: String::from("svc-storage"),
            log_config: String::from("log4rs.yaml"),
            rest_request_limit_per_second: 2,
            rest_concurrency_limit_per_service: 5,
            rest_cors_allowed_origin: String::from("http://localhost:3000"),
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();
        let default_config = Config::default();

        config::Config::builder()
            .set_default("docker_port_grpc", default_config.docker_port_grpc)?
            .set_default("docker_port_rest", default_config.docker_port_rest)?
            .set_default("log_config", default_config.log_config)?
            .set_default(
                "rest_concurrency_limit_per_service",
                default_config.rest_concurrency_limit_per_service,
            )?
            .set_default(
                "rest_request_limit_per_seconds",
                default_config.rest_request_limit_per_second,
            )?
            .set_default(
                "rest_cors_allowed_origin",
                default_config.rest_cors_allowed_origin,
            )?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[tokio::test]
    async fn test_config_from_default() {
        crate::get_log_handle().await;
        ut_info!("(test_config_from_default) Start.");

        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.docker_port_rest, 8000);
        assert_eq!(config.storage_port_grpc, 50051);
        assert_eq!(config.storage_host_grpc, String::from("svc-storage"));
        assert_eq!(config.log_config, String::from("log4rs.yaml"));
        assert_eq!(config.rest_concurrency_limit_per_service, 5);
        assert_eq!(config.rest_request_limit_per_second, 2);
        assert_eq!(
            config.rest_cors_allowed_origin,
            String::from("http://localhost:3000")
        );

        ut_info!("(test_config_from_default) Success.");
    }

    #[tokio::test]
    async fn test_config_from_env() {
        crate::get_log_handle().await;
        ut_info!("(test_config_from_env) Start.");

        std::env::set_var("DOCKER_PORT_GRPC", "6789");
        std::env::set_var("DOCKER_PORT_REST", "9876");
        std::env::set_var("STORAGE_HOST_GRPC", "test_host_grpc");
        std::env::set_var("STORAGE_PORT_GRPC", "12345");
        std::env::set_var("LOG_CONFIG", "config_file.yaml");
        std::env::set_var("REST_CONCURRENCY_LIMIT_PER_SERVICE", "255");
        std::env::set_var("REST_REQUEST_LIMIT_PER_SECOND", "255");
        std::env::set_var(
            "REST_CORS_ALLOWED_ORIGIN",
            "https://allowed.origin.host:443",
        );

        let config = Config::try_from_env();
        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.docker_port_grpc, 6789);
        assert_eq!(config.docker_port_rest, 9876);
        assert_eq!(config.storage_port_grpc, 12345);
        assert_eq!(config.storage_host_grpc, String::from("test_host_grpc"));
        assert_eq!(config.log_config, String::from("config_file.yaml"));
        assert_eq!(config.rest_concurrency_limit_per_service, 255);
        assert_eq!(config.rest_request_limit_per_second, 255);
        assert_eq!(
            config.rest_cors_allowed_origin,
            String::from("https://allowed.origin.host:443")
        );

        ut_info!("(test_config_from_env) Success.");
    }
}
