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
        Config {
            docker_port_grpc: 50051,
            docker_port_rest: 8000,
            template_rust_port_grpc: 50051,
            template_rust_host_grpc: "localhost".to_owned(),
            template_rust_port_rest: 8000,
            template_rust_host_rest: "localhost".to_owned(),
            log_config: String::from("log4rs.yaml"),
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();

        config::Config::builder()
            .set_default("docker_port_grpc", 50051)?
            .set_default("docker_port_rest", 8000)?
            .set_default("log_config", String::from("log4rs.yaml"))?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}
