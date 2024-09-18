//! Lambda which accepts empty `Ping` message and returns `ping::ServerStatus` with additional info like server timestamp
//! Intended to be called every N seconds by all the clients to sync time and ensure connection stays open

use api_core::common::health::healthy_status;
use api_core::datetime::ServerTimestamp;
use lambda_http::{run, service_fn, Error, Request, Response};

fn data(now: ServerTimestamp) -> String {
    healthy_status(now)
        .serialize()
        .expect("Health data should be always serializable")
}

async fn handler(_: Request) -> Result<Response<String>, Error> {
    Response::builder()
        .status(200)
        .body(data(ServerTimestamp::now()))
        .map_err(Error::from)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use api_core::messages::common::ping::ServerStatus;

    use super::*;

    #[test]
    fn response() {
        let now = ServerTimestamp::from_milliseconds_pure(1726219252123);
        let response = data(now.clone());
        assert_eq!(response, "-.#QT;|ls+7m9J+");
        let data = ServerStatus::deserialize(&response).unwrap();
        assert_eq!(*data.timestamp, now);
    }
}
