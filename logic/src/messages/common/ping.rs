//! Clients periodically sends Ping messages while server replies with ServerStatus

use std::sync::Arc;

use messages_macro::{client_message, server_message};

use crate::{
    datetime::{ServerTimestamp, Timestamp},
    messages::SerializationError,
};

/// Current server status
#[derive(Debug, Clone, PartialEq, bincode::Decode, bincode::Encode, uniffi::Enum)]
pub enum Status {
    /// Everything is fine
    OK,
}

/// Unix timestamp with milliseconds precision
#[derive(Debug, PartialEq, bincode::Decode, bincode::Encode)]
pub struct MyTimestamp(u64);

/// Server status message with common info like current time for time synchronization
#[server_message(1)]
pub struct ServerStatus {
    /// Current server timestamp, UTC
    pub timestamp: Arc<ServerTimestamp>,
    /// Current server status
    pub status: Status,
}

/// Client ping message
#[client_message(1)]
pub struct Ping {
    ts: Arc<Timestamp>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize() {
        // let s = Status::OK;
        // let data = bincode::encode_to_vec(&s, bincode::config::standard()).unwrap();
        // println!("{:?}", data);
        // let ss: Status = bincode::decode_from_slice(&data, bincode::config::standard()).unwrap().0;
        // assert_eq!(s, ss);

        let msg = ServerStatus {
            timestamp: Arc::new(ServerTimestamp::from_milliseconds_pure(1726219252123)),
            status: Status::OK,
        };

        // NO ARC: [253, 155, 113, 175, 234, 145, 1, 0, 0, 0]
        //    ARC: [253, 155, 113, 175, 234, 145, 1, 0, 0, 0]
        //    MYT: [253, 155, 113, 175, 234, 145, 1, 0, 0, 0]
        //    TES: [253, 155, 113, 175, 234, 145, 1, 0, 0, 0]
        let data = bincode::encode_to_vec(&msg, bincode::config::standard()).unwrap();
        println!("{:?}", data);
        let ss: ServerStatus = bincode::decode_from_slice(&data, bincode::config::standard())
            .unwrap()
            .0;
        assert_eq!(ss, msg);
    }
}
