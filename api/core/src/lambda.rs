//! Helpers for AWS lambda

use std::sync::Arc;

use aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use logic::{
    encryption::PublicKey,
    messages::{ClientPlayerMessage, ClientPublicMessage},
    server_error::ServerError,
};
use serde_json::{json, Value};

/// Event handler for events that are public and not require authentication
pub trait PublicEventHandler<T>
where
    T: ClientPublicMessage,
{
    /// Process message
    async fn process_message(&self, message: T, request_id: u8) -> Result<String, ServerError>;
}

/// Event handler for events that requires authentication
pub trait PlayerEventHandler<T>
where
    T: ClientPlayerMessage,
{
    /// Process message
    async fn process_message(
        &self,
        message: T,
        public_key: Arc<PublicKey>,
        request_id: u8,
    ) -> Result<String, ServerError>;
}

/// Run public event handler using AWS Lambda which does not require authentication. Handler may be reused many
/// times in case of warm start. In case of error the returned ServerError will be serialized as a server message
/// so it can be processed by WebSocket clients
pub async fn run_public_handler<T>(handler: &impl PublicEventHandler<T>) -> Result<(), Error>
where
    T: ClientPublicMessage,
{
    tracing::init_default_subscriber();
    run(service_fn(
        |event: LambdaEvent<ApiGatewayWebsocketProxyRequest>| async move {
            Result::<Value, Error>::Ok(to_json_response(process_public_event(event, handler).await))
        },
    ))
    .await
}

/// Run player event handler using AWS Lambda which requires player authentication. Handler may be reused many
/// times in case of warm start. In case of error the returned ServerError will be serialized as a server message
/// so it can be processed by WebSocket clients
pub async fn run_player_handler<T>(handler: &impl PlayerEventHandler<T>) -> Result<(), Error>
where
    T: ClientPlayerMessage,
{
    tracing::init_default_subscriber();
    run(service_fn(
        |event: LambdaEvent<ApiGatewayWebsocketProxyRequest>| async move {
            Result::<Value, Error>::Ok(to_json_response(process_player_event(event, handler).await))
        },
    ))
    .await
}

async fn process_public_event<T>(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    handler: &impl PublicEventHandler<T>,
) -> String
where
    T: ClientPublicMessage,
{
    let (msg, request_id) = match T::deserialize(event.payload.body.unwrap_or_default()) {
        Ok(msg) => msg,
        Err(err) => {
            return ServerError::from_serialization_error(err, T::tag(), 0)
                .serialize(0)
                .expect("Failed to serialize an error");
        }
    };
    match handler.process_message(msg, request_id).await {
        Ok(output) => output,
        Err(err) => err
            .serialize(request_id)
            .expect("Failed to serialize an error"),
    }
}

async fn process_player_event<T>(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    handler: &impl PlayerEventHandler<T>,
) -> String
where
    T: ClientPlayerMessage,
{
    let (msg, public_key, request_id) =
        match T::deserialize(event.payload.body.clone().unwrap_or_default()) {
            Ok(msg) => msg,
            Err(err) => {
                return ServerError::from_serialization_error(err, T::tag(), 0)
                    .serialize(0)
                    .expect("Failed to serialize an error");
            }
        };
    match handler.process_message(msg, public_key, request_id).await {
        Ok(output) => output,
        Err(err) => err
            .serialize(request_id)
            .expect("Failed to serialize an error"),
    }
}

/// Converts our custom string response to format that AWS API Gateway expects
fn to_json_response(response: String) -> Value {
    json!({
        "statusCode": 200,
        "body": response,
    })
}

#[cfg(test)]
mod tests {
    use logic::{
        encryption,
        messages::{common::ping::Ping, game::decay::DecayQuery},
    };

    use crate::fixtures::event_with_body;

    use super::*;

    struct HandlerSuccess {}
    impl PublicEventHandler<Ping> for HandlerSuccess {
        async fn process_message(&self, _: Ping, request_id: u8) -> Result<String, ServerError> {
            Ok(format!("Ping={}", request_id))
        }
    }
    impl PlayerEventHandler<DecayQuery> for HandlerSuccess {
        async fn process_message(
            &self,
            _: DecayQuery,
            public_key: Arc<PublicKey>,
            request_id: u8,
        ) -> Result<String, ServerError> {
            Ok(format!(
                "Query={}, pub_key_len={}",
                request_id,
                public_key.as_string().len()
            ))
        }
    }

    struct HandlerError {}
    impl PublicEventHandler<Ping> for HandlerError {
        async fn process_message(&self, _: Ping, request_id: u8) -> Result<String, ServerError> {
            Err(ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "error_description".to_string(),
                error_context: Some("error_context".to_string()),
                request_id,
                message_tag: Ping::tag(),
                recoverable: false,
            })
        }
    }

    impl PlayerEventHandler<DecayQuery> for HandlerError {
        async fn process_message(
            &self,
            _: DecayQuery,
            _: Arc<PublicKey>,
            request_id: u8,
        ) -> Result<String, ServerError> {
            Err(ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "error_description".to_string(),
                error_context: Some("error_context".to_string()),
                request_id,
                message_tag: DecayQuery::tag(),
                recoverable: false,
            })
        }
    }

    #[tokio::test]
    async fn public_handler_success() {
        let request_id = 1;
        let event = event_with_body(Ping {}.serialize(request_id).unwrap());
        let response = process_public_event(event, &HandlerSuccess {}).await;
        assert_eq!(response, "Ping=1");
    }

    #[tokio::test]
    async fn public_handler_error() {
        let request_id = 1;
        let event = event_with_body(Ping {}.serialize(request_id).unwrap());
        let response = process_public_event(event, &HandlerError {}).await;
        assert_eq!(response, "-0#3x]*4qc;lAjfA_`* x$/+fV,P|OaI2nLRj1&RQ*$L^a+J");
        let error = ServerError::deserialize(&response).unwrap();
        assert_eq!(error.1, request_id);
        assert_eq!(
            error.0,
            ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "error_description".to_string(),
                error_context: Some("error_context".to_string()),
                request_id,
                message_tag: Ping::tag(),
                recoverable: false,
            }
        )
    }

    #[tokio::test]
    async fn public_handler_bad_data() {
        let event = event_with_body("bad_data".to_string());
        let response = process_public_event(event, &HandlerError {}).await;
        assert_eq!(response, "-0@[7v#9hc8,szzmMvunH:XN*XpO]'7z8ZJDP6e%^L8|mIjh|#{(.hecR03!-ar@9ka$5:^[9I7y9=MZ=a#fK-7F+,YXa$x!B=`C+clVaArPOT:O)V8]C");
        let error = ServerError::deserialize(&response).unwrap();
        assert_eq!(error.1, 0);
        assert_eq!(
            error.0,
            ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "Data is invalid and cannot be processed".to_string(),
                error_context: Some("Data error: No json_prefix and json_suffix found".to_string()),
                request_id: 0,
                message_tag: Ping::tag(),
                recoverable: false
            }
        )
    }

    #[tokio::test]
    async fn player_handler_success() {
        let request_id = 1;
        let keys = encryption::generate_new_keys();
        let event = event_with_body(
            DecayQuery {}
                .serialize(
                    request_id,
                    keys.public_key.as_ref().clone(),
                    keys.private_key.as_ref().clone(),
                )
                .unwrap(),
        );
        let response = process_player_event(event, &HandlerSuccess {}).await;
        assert_eq!(response, "Query=1, pub_key_len=40");
    }

    #[tokio::test]
    async fn player_handler_error() {
        let request_id = 1;
        let keys = encryption::generate_new_keys();
        let event = event_with_body(
            DecayQuery {}
                .serialize(
                    request_id,
                    keys.public_key.as_ref().clone(),
                    keys.private_key.as_ref().clone(),
                )
                .unwrap(),
        );
        let response = process_player_event(event, &HandlerError {}).await;
        assert_eq!(response, "-0#3x]*4qc;lAjfA_`* x$/+fV,P|OaI2nLRj1&RQ*$L^hR]");
        let error = ServerError::deserialize(&response).unwrap();
        assert_eq!(error.1, request_id);
        assert_eq!(
            error.0,
            ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "error_description".to_string(),
                error_context: Some("error_context".to_string()),
                request_id,
                message_tag: DecayQuery::tag(),
                recoverable: false,
            }
        )
    }

    #[tokio::test]
    async fn player_handler_bad_data() {
        let event = event_with_body("bad_data".to_string());
        let response = process_player_event(event, &HandlerError {}).await;
        assert_eq!(response, "-0@[7v#9hc8,szzmMvunH:XN*XpO]'7z8ZJDP6e%^L8|mIjh|#{(.hecR03!-ar@9ka$5:^[9I7y9=MZ=a#fK-7F+,YXa$x!B=`C+clVaArPOT:O)V@%U");
        let error = ServerError::deserialize(&response).unwrap();
        assert_eq!(error.1, 0);
        assert_eq!(
            error.0,
            ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "Data is invalid and cannot be processed".to_string(),
                error_context: Some("Data error: No json_prefix and json_suffix found".to_string()),
                request_id: 0,
                message_tag: DecayQuery::tag(),
                recoverable: false
            }
        )
    }

    #[tokio::test]
    async fn player_handler_bad_signature() {
        let request_id = 1;
        let keys1 = encryption::generate_new_keys();
        let keys2 = encryption::generate_new_keys();
        let event = event_with_body(
            DecayQuery {}
                .serialize(
                    request_id,
                    keys1.public_key.as_ref().clone(),
                    keys2.private_key.as_ref().clone(),
                )
                .unwrap(),
        );
        let response = process_player_event(event, &HandlerError {}).await;
        assert_eq!(response, "-06:F$[V5MA6J\u{7f}xER-+EoB`ouyt.m>f\u{7f}yo/E@.{9=bBiq(!8bT.P .RcE54*f{b&o4,wRHd'sOA#kj8@,B8`gr&c ;4)WuiCf5ux");
        let error = ServerError::deserialize(&response).unwrap();
        assert_eq!(error.1, 0);
        assert_eq!(
            error.0,
            ServerError {
                error_code: logic::server_error::ErrorCode::SerializationError,
                error_description: "Data is invalid and cannot be processed".to_string(),
                error_context: Some("Data error: Cannot verify the data".to_string()),
                request_id: 0,
                message_tag: DecayQuery::tag(),
                recoverable: false
            }
        )
    }
}
