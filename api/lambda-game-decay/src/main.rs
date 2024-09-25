use std::sync::Arc;

use api_core::{
    datetime::{Duration, ServerTimestamp},
    messages::game::decay::{Decay, DecayQuery},
};
use lambda_http::{run, service_fn, Error, Request, Response};

const DECAY_DURATION_DAYS: u64 = 365 * 10 + 1;

// TODO This is just a test handler with no error handling or anything, just to check authentication
//      We need a common error handling for backend, which will be handled next as a separate task
//      Actually this whole logic around Decay should be either moved to api-core or better yet fully
//      reworked with more general mechanism which will return such resource
fn process_request(req: String, now: ServerTimestamp) -> String {
    let (_, player_id, request_id) = DecayQuery::deserialize(req).unwrap();
    println!("Got decay request for UserId: {0}", player_id.as_string());
    let decay = Decay {
        started_at: Arc::new(now),
        length: Duration::from_milliseconds(DECAY_DURATION_DAYS * 24 * 60 * 60 * 1000),
    };
    decay.serialize(request_id).unwrap()
}

async fn handler(req: Request) -> Result<Response<String>, Error> {
    match req.into_body() {
        lambda_http::Body::Text(data) => Response::builder()
            .status(200)
            .body(process_request(data, ServerTimestamp::now()))
            .map_err(Error::from),
        _ => todo!("Error handling..."),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use api_core::encryption;

    use super::*;

    #[test]
    fn test_request() {
        let keys = encryption::generate_new_keys();
        let private_key = (*keys.private_key).clone();
        let public_key = (*keys.public_key).clone();
        let data = DecayQuery { unused: false };
        let req = data.serialize(1, public_key, private_key).unwrap();
        let now = ServerTimestamp::from_milliseconds_pure(10);
        let response = process_request(req, now.clone());
        assert_eq!(response, "-/<whrHUB6=QUX@");
        let (decay, req_id) = Decay::deserialize(&response).unwrap();
        assert_eq!(*decay.started_at, now);
        assert_eq!(decay.length.whole_days(), DECAY_DURATION_DAYS);
        assert_eq!(req_id, 1);
    }
}
