//! Date and time related functionality

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// Date, string format YYYY-MM-DD
#[derive(Debug, PartialEq, Clone)]
pub struct Date(time::Date);

impl Date {
    /// Create new date object from given parameters where month and day starts with 1
    pub fn new(year: u32, month: u8, day: u8) -> Self {
        let month = time::Month::try_from(month).expect("invalid month number");
        let date = time::Date::from_calendar_date(year as i32, month, day).expect("invalid date");
        Date(date)
    }

    /// Returns a new `Date` that is earlier by the specified number of days
    pub fn remove_days(&self, days: usize) -> Date {
        let res = self.0.saturating_add(time::Duration::days(-(days as i64)));
        Self(res)
    }

    /// Returns a new `Date` that is later by the specified number of days
    pub fn add_days(&self, days: usize) -> Date {
        let res = self.0.saturating_add(time::Duration::days(days as i64));
        Self(res)
    }

    /// Returns current year
    pub fn year(&self) -> usize {
        self.0.year() as usize
    }

    /// Returns current month in range from 1 to 12
    pub fn month(&self) -> usize {
        self.0.month() as usize
    }

    /// Returns current day in range from 1 to 31
    pub fn day(&self) -> usize {
        self.0.day() as usize
    }

    /// Returns the zero-indexed number of days from Monday e.g. Tuesday is 1
    pub fn days_from_monday(&self) -> u8 {
        self.0.weekday().number_days_from_monday()
    }

    /// Returns Date of the beginning of the current week
    pub fn as_start_of_week(&self) -> Self {
        self.remove_days(self.days_from_monday().into())
    }

    /// Returns Date of the beginning of the current month
    pub fn as_start_of_month(&self) -> Self {
        self.remove_days(self.day() - 1)
    }

    /// Returns Date of the beginning of the current year
    pub fn as_start_of_year(&self) -> Self {
        Date::new(
            self.year()
                .try_into()
                .expect("Cannot get a year from DateDay"),
            1,
            1,
        )
    }

    /// Returns absolute duration difference between two dates
    pub fn diff(&self, other: &Date) -> Duration {
        let diff = (self.0 - other.0).whole_seconds().unsigned_abs();
        Duration::from_milliseconds(diff * 1000)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:04}-{:02}-{:02}",
            self.0.year(),
            self.0.month() as u8,
            self.0.day()
        ))
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        check_format(s, vec!['d', 'd', 'd', 'd', '-', 'd', 'd', '-', 'd', 'd'])?;
        let year: i32 = parse_number(&s[0..4], 1900, 3000)?;
        let month: u8 = parse_number(&s[5..7], 1, 12)?;
        let month = time::Month::try_from(month).expect("invalid month number");
        let day: u8 = parse_number(&s[8..10], 1, 31)?;
        let date = time::Date::from_calendar_date(year, month, day)
            .map_err(|err| format!("invalid date: {}", err))?;
        Ok(Date(date))
    }
}

/// Unix timestamp with milliseconds precision
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Timestamp(u64);

impl Timestamp {
    /// Returns current timestamp, UTC
    pub fn now() -> Self {
        let now = time::OffsetDateTime::now_utc();
        Self((now.unix_timestamp_nanos() / 1_000_000) as u64)
    }

    /// Creates a new timestamp with given amount of milliseconds
    pub fn from_milliseconds(milliseconds: u64) -> Self {
        Self(milliseconds)
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
        let value: u64 = s.parse()?;
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
    pub fn from_milliseconds(milliseconds: u64) -> Self {
        Self(Timestamp::from_milliseconds(milliseconds))
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

/// Time duration with milliseconds precision, string representation is in [HH:MM::SS.SSS] format
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Duration(u64);

impl Duration {
    /// Create new duration from passed amount of milliseconds
    pub fn from_milliseconds(milliseconds: u64) -> Self {
        Self(milliseconds)
    }

    /// Number of a whole minutes in a duration
    pub fn whole_minutes(&self) -> u64 {
        self.0 / 1000 / 60
    }

    /// Number of whole hours in a duration
    pub fn whole_hours(&self) -> u64 {
        self.whole_minutes() / 60
    }

    /// Number of whole 24-hours days in a duration
    pub fn whole_days(&self) -> u64 {
        self.whole_hours() / 24
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.whole_hours();
        let minutes = self.whole_minutes() - hours * 60;
        let seconds = self.0 / 1000 - hours * 60 * 60 - minutes * 60;
        let milliseconds = self.0 - (hours * 60 * 60 + minutes * 60 + seconds) * 1000;
        f.write_fmt(format_args!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        ))
    }
}

fn parse_number<T: FromStr + Ord + Display>(s: &str, min: T, max: T) -> Result<T, String> {
    let parsed = match s.parse::<T>() {
        Ok(parsed) => parsed,
        _ => return Err(format!("Cannot parse {}", s)),
    };
    if parsed < min || parsed > max {
        return Err(format!(
            "Value is out of range of {}..{}, got {}",
            min, max, parsed
        ));
    };
    Ok(parsed)
}

fn check_format(s: &str, format: Vec<char>) -> Result<(), String> {
    let mut idx = 0;
    for c in s.chars() {
        if idx == format.len() {
            return Err(format!(
                "String is longer than expected length of {}",
                format.len()
            ));
        }
        let expected = format[idx];
        if expected == 'd' && !c.is_ascii_digit() {
            return Err(format!("Expected digit at index {}, got {}", idx, c));
        }
        if expected != 'd' && expected != c {
            return Err(format!("Expected {} at index {}, got {}", expected, idx, c));
        }
        idx += 1;
    }
    if idx != format.len() {
        return Err(format!(
            "Expected string length of {}, got {}",
            format.len(),
            idx
        ));
    }
    Ok(())
}

/// Timestamp that adjusts to the server's time to synchronize client and server clocks
/// For clients that support time synchronization to minimize time drift between client and server
#[derive(Default)]
pub struct SyncedTimestamp {
    offset_ms: i64,
}

impl SyncedTimestamp {
    /// Maximum possible round trip time after which adjustment would be ignored
    pub const MAX_RTT_MS: u64 = 10_000;

    /// Create new synced timestamp
    pub fn new() -> Self {
        Self { offset_ms: 0 }
    }

    /// Adjusts the time offset using server time and round-trip time.
    pub fn adjust(
        &mut self,
        server_time: ServerTimestamp,
        sent_at: Timestamp,
        received_at: Timestamp,
    ) {
        let rtt = received_at.diff(&sent_at);
        if rtt.0 > SyncedTimestamp::MAX_RTT_MS {
            // rtt is loo long and likely adjustment cannot be reliably calculated
            return;
        }
        let latency = rtt.0 / 2;
        let estimated_server_time = Timestamp::from_milliseconds(sent_at.0 + latency);
        // Not using Duration here as it can be only positive
        self.offset_ms = server_time.0 .0 as i64 - estimated_server_time.0 as i64;
    }

    /// Returns current timestamp adjusted for the server offset
    pub fn now(&self) -> Timestamp {
        let adjusted = Timestamp::now().0 as i64 + self.offset_ms;
        Timestamp::from_milliseconds(adjusted as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn date_format() {
        assert_eq!(Date::new(2022, 5, 9).to_string(), "2022-05-09")
    }

    #[test]
    fn date_day_of_week() {
        let day = Date::new(2023, 7, 6);
        assert_eq!(day.days_from_monday(), 3); // Thursday is 3 days from Monday
        assert_eq!(day.remove_days(3).days_from_monday(), 0); // Finding beginning of the week - Monday
        assert_eq!(day.add_days(3).days_from_monday(), 6); // Finding end of the week - Sunday
    }

    #[test]
    fn date_as_start() {
        let day = Date::new(2023, 7, 6);
        assert_eq!(day.as_start_of_week(), Date::new(2023, 7, 3));
        assert_eq!(day.as_start_of_month(), Date::new(2023, 7, 1));
        assert_eq!(day.as_start_of_year(), Date::new(2023, 1, 1));
    }

    #[test]
    fn date_parse() {
        assert_eq!("2022-05-09".parse::<Date>().unwrap(), Date::new(2022, 5, 9));
        assert_eq!("2022-01-01".parse::<Date>().unwrap(), Date::new(2022, 1, 1));
        assert_eq!(
            "2022-12-31".parse::<Date>().unwrap(),
            Date::new(2022, 12, 31)
        );
        assert!("2022-13-31".parse::<Date>().is_err());
        assert!("2022-12-32".parse::<Date>().is_err());
        assert!("2022-00-09".parse::<Date>().is_err());
        assert!("2022-13-09".parse::<Date>().is_err());
        assert!("2022-09-32".parse::<Date>().is_err());
    }

    #[test]
    fn date_diff() {
        let d1 = Date::new(2022, 5, 9);
        let d2 = Date::new(2022, 5, 10);
        assert_eq!(d1.diff(&d2).whole_days(), 1);
        assert_eq!(d2.diff(&d1).whole_days(), 1);
    }

    #[test]
    fn duration_new() {
        // Minutes
        assert_eq!(Duration::from_milliseconds(1000).whole_minutes(), 0);
        assert_eq!(Duration::from_milliseconds(60 * 1000).whole_minutes(), 1);
        assert_eq!(Duration::from_milliseconds(61 * 1000).whole_minutes(), 1);
        // Hours
        assert_eq!(Duration::from_milliseconds(1000).whole_hours(), 0);
        assert_eq!(Duration::from_milliseconds(60 * 60 * 1000).whole_hours(), 1);
        assert_eq!(
            Duration::from_milliseconds(3 * 60 * 60 * 1000 + 1).whole_hours(),
            3
        );
        // Days
        assert_eq!(Duration::from_milliseconds(1000).whole_days(), 0);
        assert_eq!(
            Duration::from_milliseconds(24 * 60 * 60 * 1000).whole_days(),
            1
        );
        assert_eq!(
            Duration::from_milliseconds(2 * 24 * 60 * 60 * 1000 + 1).whole_days(),
            2
        );
        // Min/Max
        assert_eq!(Duration::from_milliseconds(0).whole_days(), 0);
        assert_eq!(
            Duration::from_milliseconds(u64::MAX).whole_days(),
            213_503_982_334
        );
    }

    #[test]
    fn duration_display() {
        assert_eq!(Duration::from_milliseconds(0).to_string(), "00:00:00.000");
        assert_eq!(
            Duration::from_milliseconds(83_245).to_string(),
            "00:01:23.245"
        );
        assert_eq!(
            Duration::from_milliseconds(5 * 60 * 60 * 1000 + 83_984).to_string(),
            "05:01:23.984"
        );
        assert_eq!(
            Duration::from_milliseconds(u64::MAX).to_string(),
            "5124095576030:25:51.615"
        );
    }

    #[test]
    fn timestamp_diff() {
        assert_eq!(
            Timestamp::from_milliseconds(1).diff(&Timestamp::from_milliseconds(2)),
            Duration::from_milliseconds(1)
        );
        let t1 = Timestamp::from_milliseconds(0);
        let t2 = Timestamp::from_milliseconds(u64::MAX);
        // Order doesn't matter
        assert_eq!(t1.diff(&t2), Duration::from_milliseconds(u64::MAX));
        assert_eq!(t2.diff(&t1), Duration::from_milliseconds(u64::MAX));
    }

    #[test]
    fn timestamp_now() {
        // Just a compilation test to ensure getting current time works on a target platform
        let now = Timestamp::now();
        assert!(now.0 > 0);
    }

    #[test]
    fn synced_timestamp_adjust() {
        let mut ts = SyncedTimestamp::new();

        // Perfect sync
        ts.adjust(
            ServerTimestamp(Timestamp(1_500)),
            Timestamp(0),
            Timestamp(3_000),
        );
        assert_eq!(ts.offset_ms, 0);

        // Server is ahead
        ts.adjust(
            ServerTimestamp(Timestamp(2_000)),
            Timestamp(0),
            Timestamp(3_000),
        );
        assert_eq!(ts.offset_ms, 500);

        // Client is ahead
        ts.adjust(
            ServerTimestamp(Timestamp(1_000)),
            Timestamp(2_000),
            Timestamp(3_000),
        );
        assert_eq!(ts.offset_ms, -1_500);

        // Long RTT adjustment are ignored
        let mut ts = SyncedTimestamp::new();
        ts.adjust(
            ServerTimestamp(Timestamp(4_000)),
            Timestamp(0),
            Timestamp(SyncedTimestamp::MAX_RTT_MS + 1),
        );
        assert_eq!(ts.offset_ms, 0);
    }
}
