use std::sync::Arc;

use api_core::{
    datetime::{Duration, ServerTimestamp},
    encryption::PublicKey,
    lambda::{run_player_handler, PlayerEventHandler},
    messages::{
        game::decay::{Decay, DecayQuery},
        ClientPlayerMessage,
    },
    server_error::ServerError,
};
use lambda_runtime::Error;

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
    decay
        .serialize(request_id)
        .map_err(|err| ServerError::from_serialization_error(err, DecayQuery::tag(), request_id))
}

impl PlayerEventHandler<DecayQuery> for Handler {
    async fn process_message(
        &self,
        message: DecayQuery,
        _: Arc<PublicKey>,
        request_id: u8,
    ) -> Result<String, ServerError> {
        process_message(message, request_id, ServerTimestamp::now())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_player_handler(&Handler {}).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_message_ok() {
        let msg = DecayQuery {};
        let now = ServerTimestamp::from_milliseconds_pure(10);
        let response = process_message(msg, 1, now.clone()).unwrap();
        assert_eq!(response, "-/-.+8GP/R@<_.Ht");
        let (decay, req_id) = Decay::deserialize(&response).unwrap();
        assert_eq!(*decay.started_at, now);
        assert_eq!(decay.length.whole_days(), DECAY_DURATION_DAYS);
        assert_eq!(req_id, 1);
    }
}
