//! Lambda which accepts empty `Ping` message and returns `ping::ServerStatus` with additional info like server timestamp
//! Intended to be called every N seconds by all the clients to sync time and ensure connection stays open

use api_core::datetime::ServerTimestamp;
use api_core::lambda::{run_lambda, EventHandler};
use api_core::server_error::ServerError;
use api_core::ApiGatewayWebsocketProxyRequest;
use api_core::{common::health::healthy_status, messages::common::ping::Ping};
use lambda_runtime::{Error, LambdaEvent};

struct Handler {}

fn process_message(_: Ping, request_id: u8, now: ServerTimestamp) -> Result<String, ServerError> {
    healthy_status(now)
        .serialize(0)
        .map_err(|err| ServerError::from_serialization_error(err, Ping::message_tag(), request_id))
}

impl EventHandler for Handler {
    async fn process_event(
        &self,
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> Result<String, ServerError> {
        let (message, request_id) = Ping::deserialize(event.payload.body.unwrap_or_default())
            .map_err(|err| ServerError::from_serialization_error(err, Ping::message_tag(), 0))?;
        process_message(message, request_id, ServerTimestamp::now())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_lambda(&Handler {}).await
}

#[cfg(test)]
mod tests {
    use api_core::{fixtures::event_with_body, messages::common::ping::ServerStatus};

    use super::*;

    #[test]
    fn process_message_ok() {
        let now = ServerTimestamp::from_milliseconds_pure(1726219252123);
        let response = process_message(Ping { unused: false }, 0, now.clone()).unwrap();
        assert_eq!(response, "-.'r4aJ:tuv)|T{7");
        let (data, req_id) = ServerStatus::deserialize(&response).unwrap();
        assert_eq!(*data.timestamp, now);
        assert_eq!(req_id, 0);
    }

    #[tokio::test]
    async fn process_event_error() {
        let event = event_with_body("bad_data".to_string());
        let err = Handler {}.process_event(event).await.err().unwrap();
        assert_eq!(
            err,
            ServerError {
                error_code: api_core::server_error::ErrorCode::SerializationError,
                error_description: "Data is invalid and cannot be processed".to_string(),
                error_context: Some("Data error: No json_prefix and json_suffix found".to_string()),
                request_id: 0,
                message_tag: Ping::message_tag(),
                recoverable: false
            }
        )
    }
}
