//! Lambda which accepts empty `Ping` message and returns `ping::ServerStatus` with additional info like server timestamp
//! Intended to be called every N seconds by all the clients to sync time and ensure connection stays open

use api_core::datetime::ServerTimestamp;
use api_core::lambda::{run_public_handler, PublicEventHandler};
use api_core::messages::ClientPublicMessage;
use api_core::server_error::ServerError;
use api_core::{common::health::healthy_status, messages::common::ping::Ping};
use lambda_runtime::Error;

struct Handler {}

fn process_message(_: Ping, request_id: u8, now: ServerTimestamp) -> Result<String, ServerError> {
    healthy_status(now)
        .serialize(0)
        .map_err(|err| ServerError::from_serialization_error(err, Ping::tag(), request_id))
}

impl PublicEventHandler<Ping> for Handler {
    async fn process_message(&self, msg: Ping, request_id: u8) -> Result<String, ServerError> {
        process_message(msg, request_id, ServerTimestamp::now())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_public_handler(&Handler {}).await
}

#[cfg(test)]
mod tests {
    use api_core::messages::common::ping::ServerStatus;

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
}
