use std::sync::Arc;

use api_core::{
    datetime::{Duration, ServerTimestamp},
    lambda::{run_lambda, EventHandler},
    messages::game::decay::{Decay, DecayQuery},
    server_error::ServerError,
    ApiGatewayWebsocketProxyRequest,
};
use lambda_runtime::{Error, LambdaEvent};

const DECAY_DURATION_DAYS: u64 = 365 * 10 + 1;

struct Handler {}

fn process_message(
    _: DecayQuery,
    request_id: u8,
    now: ServerTimestamp,
) -> Result<String, ServerError> {
    let decay = Decay {
        started_at: Arc::new(now),
        length: Duration::from_milliseconds(DECAY_DURATION_DAYS * 24 * 60 * 60 * 1000),
    };
    decay.serialize(request_id).map_err(|err| {
        ServerError::from_serialization_error(err, DecayQuery::message_tag(), request_id)
    })
}

impl EventHandler for Handler {
    async fn process_event(
        &self,
        event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    ) -> Result<String, ServerError> {
        let (message, _, request_id) =
            DecayQuery::deserialize(event.payload.body.unwrap_or_default()).map_err(|err| {
                ServerError::from_serialization_error(err, DecayQuery::message_tag(), 0)
            })?;
        process_message(message, request_id, ServerTimestamp::now())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_lambda(&Handler {}).await
}

#[cfg(test)]
mod tests {
    use api_core::{encryption, fixtures::event_with_body};

    use super::*;

    #[test]
    fn process_message_ok() {
        let msg = DecayQuery { unused: false };
        let now = ServerTimestamp::from_milliseconds_pure(10);
        let response = process_message(msg, 1, now.clone()).unwrap();
        assert_eq!(response, "-/<whrHUB6=QUX@");
        let (decay, req_id) = Decay::deserialize(&response).unwrap();
        assert_eq!(*decay.started_at, now);
        assert_eq!(decay.length.whole_days(), DECAY_DURATION_DAYS);
        assert_eq!(req_id, 1);
    }

    #[tokio::test]
    async fn process_event_error() {
        let keys1 = encryption::generate_new_keys();
        let keys2 = encryption::generate_new_keys();
        let query = DecayQuery { unused: false };
        let query_serialized = query
            .serialize(
                1,
                keys1.public_key.as_ref().clone(),
                keys2.private_key.as_ref().clone(),
            )
            .unwrap();

        let event = event_with_body(query_serialized);
        let err = Handler {}.process_event(event).await.err().unwrap();
        assert_eq!(
            err,
            ServerError {
                error_code: api_core::server_error::ErrorCode::SerializationError,
                error_description: "Data is invalid and cannot be processed".to_string(),
                error_context: Some("Data error: Cannot verify the data".to_string()),
                request_id: 0,
                message_tag: DecayQuery::message_tag(),
                recoverable: false
            }
        )
    }
}
