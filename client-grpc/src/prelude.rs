//! Re-export of used objects

pub use super::client as template_rust;
pub use super::service::Client as TemplateRustServiceClient;
pub use template_rust::TemplateRustClient;

pub use lib_common::grpc::Client;
