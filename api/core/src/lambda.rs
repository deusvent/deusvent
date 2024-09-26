//! Helpers for AWS lambda

use aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use logic::server_error::ServerError;

/// Main trait for processing ApiGatewayWebsocket events
pub trait EventHandler {
    /// Process event
    async fn process_event(
        &self,
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> Result<String, ServerError>;
}

/// Run event handler using AWS Lambda. Handler may be reused many times in case of warm start. In case of error
/// the returned ServerError will be serialized as a server message so it can be processed by WebSocket clients
pub async fn run_lambda<E: EventHandler>(handler: &E) -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(|event| async move {
        handler.process_event(event).await.or_else(|err| {
            Result::<String, Error>::Ok(err.serialize(0).expect("Failed to serialize an error"))
        })
    }))
    .await
}
