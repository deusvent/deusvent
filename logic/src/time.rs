//! Time related structs and functionality

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Unix timestamp with second precision, supports time until 2106 year
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(u32);

impl Timestamp {
    /// Returns current timestamp, UTC
    pub fn now() -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self(now.unix_timestamp() as u32)
    }

    /// Creates a new timestamp with given amount of seconds
    pub fn new(seconds: u32) -> Self {
        Self(seconds)
    }

    /// Returns absolute duration from the second timestamp
    pub fn diff(&self, other: &Timestamp) -> Duration {
        let diff = self.0.abs_diff(other.0);
        Duration(diff)
    }

    /// Returns timestamp number value as string
    pub fn as_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for Timestamp {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u32 = s.parse()?;
        Ok(Timestamp(value))
    }
}

/// Wrapper type for timestamp that was created on a server, meaning it could be trusted
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerTimestamp(Timestamp);

impl ServerTimestamp {
    #[cfg(feature = "server")]
    /// Creates a new server timestamp for current time, available only in server context
    pub fn now() -> Self {
        Self(Timestamp::now())
    }

    /// Creates a new server timestamp with given amount of seconds
    pub fn new(seconds: u32) -> Self {
        Self(Timestamp::new(seconds))
    }

    /// Returns server timestamp number value as string
    pub fn as_string(&self) -> String {
        self.0.as_string()
    }
}

impl FromStr for ServerTimestamp {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timestamp: Timestamp = s.parse()?;
        Ok(ServerTimestamp(timestamp))
    }
}

/// Time duration with second precision, string representation is in [HH:MM::SS] format
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Duration(u32);

impl Duration {
    /// Create new duration from passed amount of seconds
    pub fn new(seconds: u32) -> Self {
        Self(seconds)
    }

    /// Number of a whole minutes in a duration
    pub fn whole_minutes(&self) -> u32 {
        self.0 / 60
    }

    /// Number of whole hours in a duration
    pub fn whole_hours(&self) -> u32 {
        self.whole_minutes() / 60
    }

    /// Number of whole 24-hours days in a duration
    pub fn whole_days(&self) -> u32 {
        self.whole_hours() / 24
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.whole_hours();
        let minutes = self.whole_minutes() - hours * 60;
        let seconds = self.0 - hours * 60 * 60 - minutes * 60;
        f.write_fmt(format_args!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_durations() {
        // Minutes
        assert_eq!(Duration::new(1).whole_minutes(), 0);
        assert_eq!(Duration::new(60).whole_minutes(), 1);
        assert_eq!(Duration::new(61).whole_minutes(), 1);
        // Hours
        assert_eq!(Duration::new(1).whole_hours(), 0);
        assert_eq!(Duration::new(60 * 60).whole_hours(), 1);
        assert_eq!(Duration::new(3 * 60 * 60 + 1).whole_hours(), 3);
        // Days
        assert_eq!(Duration::new(1).whole_days(), 0);
        assert_eq!(Duration::new(24 * 60 * 60).whole_days(), 1);
        assert_eq!(Duration::new(2 * 24 * 60 * 60 + 1).whole_days(), 2);
        // Min/Max
        assert_eq!(Duration::new(0).whole_days(), 0);
        assert_eq!(Duration::new(u32::MAX).whole_days(), 49_710);
    }

    #[test]
    fn test_display() {
        assert_eq!(Duration::new(0).to_string(), "00:00:00");
        assert_eq!(Duration::new(83).to_string(), "00:01:23");
        assert_eq!(Duration::new(5 * 60 * 60 + 83).to_string(), "05:01:23");
        assert_eq!(Duration::new(u32::MAX).to_string(), "1193046:28:15");
    }

    #[test]
    fn test_timestamp() {
        assert_eq!(Timestamp::new(1).diff(&Timestamp::new(2)), Duration::new(1));
        let t1 = Timestamp::new(0);
        let t2 = Timestamp::new(u32::MAX);
        // Order doesn't matter
        assert_eq!(t1.diff(&t2), Duration::new(u32::MAX));
        assert_eq!(t2.diff(&t1), Duration::new(u32::MAX));
    }

    #[test]
    fn test_now() {
        // Just a compilation test to ensure getting current time works on a target platform
        let now = Timestamp::now();
        assert!(now.0 > 0);
    }
}
