//! Lambda which accepts empty `Ping` message and returns `ping::ServerStatus` with additional info like server timestamp
//! Intended to be called every N seconds by all the clients to sync time and ensure connection stays open

use api_core::common::health::healthy_status;
use api_core::datetime::ServerTimestamp;
use api_core::messages::Message;
use lambda_http::{run, service_fn, Error, Request, Response};

fn data(now: ServerTimestamp) -> String {
    let health = healthy_status(now);
    let data = health
        .serialize()
        .expect("Health data should be always serializable");
    String::from_utf8(data).expect("Health data should be always a string")
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
    use super::*;

    #[test]
    fn response() {
        let now = ServerTimestamp::new(1726219252);
        let response = data(now);
        assert_eq!(
            response,
            r#"{"type":"common.serverStatus","timestamp":1726219252,"status":"OK"}"#
        );
    }
}
