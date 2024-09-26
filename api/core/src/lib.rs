//! Core services library for all API functionality

#![allow(async_fn_in_trait)] // Ignore it as we plan to use async trait only in our code which is fine
#![deny(missing_docs)] // It's used by all API services and should be well documented

// Re-export some of the functionality to simplify dependency for API lambdas
pub use aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
pub use logic::*;

pub mod common;
pub mod entities;
pub mod fixtures;
pub mod lambda;
pub mod storage;
