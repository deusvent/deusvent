uniffi::include_scaffolding!("logic");

use std::sync::Arc;
use std::sync::Mutex;

pub use logic::datetime::ServerTimestamp;
pub use logic::datetime::Timestamp;

// Wrap logic::datetime::SyncedTimestamp as we can't expose mutable functions
#[derive(Default)]
pub struct SyncedTimestamp {
    timestamp: Arc<Mutex<logic::datetime::SyncedTimestamp>>,
}

impl SyncedTimestamp {
    pub fn new() -> Self {
        Self {
            timestamp: Arc::new(Mutex::new(logic::datetime::SyncedTimestamp::new())),
        }
    }

    pub fn adjust(
        &self,
        server_time: &ServerTimestamp,
        sent_at: &Timestamp,
        received_at: &Timestamp,
    ) {
        let mut timestamp = self.timestamp.lock().expect("Cannot lock synced timestamp");
        timestamp.adjust(server_time, sent_at, received_at)
    }

    pub fn now(&self) -> Arc<Timestamp> {
        let timestamp = self.timestamp.lock().expect("Cannot lock synced timestamp");
        Arc::new(timestamp.now())
    }
}
