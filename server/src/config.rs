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
    /// host of template_rust gRPC server
    pub template_rust_host_grpc: String,
    /// port of template_rust gRPC server
    pub template_rust_port_grpc: u16,
    /// host of template_rust rest server
    pub template_rust_host_rest: String,
    /// port of template_rust rest server
    pub template_rust_port_rest: u16,
    /// path to log configuration YAML file
    pub log_config: String,
}

impl Default for Config {
    fn default() -> Self {
        log::warn!("Creating Config object with default values.");
        Self::new()
    }
}

impl Config {
    /// Default values for Config
    pub fn new() -> Self {
        println!(
            "Could not find all required values in the environment. Using default config values."
        );
        Config {
            docker_port_grpc: 50051,
            docker_port_rest: 8000,
            template_rust_port_grpc: 50051,
            template_rust_host_grpc: "localhost".to_owned(),
            template_rust_port_rest: 8000,
            template_rust_host_rest: "localhost".to_owned(),
            log_config: String::from("/log4rs.yaml"),
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();

        config::Config::builder()
            .set_default("docker_port_grpc", 50051)?
            .set_default("docker_port_rest", 8000)?
            .set_default("log_config", "./log4rs.yaml")?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_default() {
        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.docker_port_rest, 8000);
        assert_eq!(config.template_rust_port_grpc, 50051);
        assert_eq!(config.template_rust_host_grpc, String::from("localhost"));
        assert_eq!(config.template_rust_port_rest, 8000);
        assert_eq!(config.template_rust_host_rest, String::from("localhost"));
        assert_eq!(config.log_config, String::from("log4rs.yaml"));
    }

    #[tokio::test]
    async fn test_config_from_env() {
        async move {
            std::env::set_var("DOCKER_PORT_GRPC", "6789");
            std::env::set_var("DOCKER_PORT_REST", "9876");
            std::env::set_var("TEMPLATE_RUST_HOST_GRPC", "test_host_grpc");
            std::env::set_var("TEMPLATE_RUST_PORT_GRPC", "12345");
            std::env::set_var("TEMPLATE_RUST_HOST_REST", "test_host_rest");
            std::env::set_var("TEMPLATE_RUST_PORT_REST", "54321");
            std::env::set_var("LOG_CONFIG", "config_file.yaml");

            let config = Config::try_from_env();
            assert!(config.is_ok());
            let config = config.unwrap();

            assert_eq!(config.docker_port_grpc, 6789);
            assert_eq!(config.docker_port_rest, 9876);
            assert_eq!(config.template_rust_port_grpc, 12345);
            assert_eq!(
                config.template_rust_host_grpc,
                String::from("test_host_grpc")
            );
            assert_eq!(config.template_rust_port_rest, 54321);
            assert_eq!(
                config.template_rust_host_rest,
                String::from("test_host_rest")
            );
            assert_eq!(config.log_config, String::from("config_file.yaml"));
        }
        .await
    }
}
