//! Date and time related functionality

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// Date, format YYYY-MM-DD
#[derive(Debug, PartialEq, Clone)]
pub struct Date(time::Date);

impl Date {
    /// Create new date object from given parameters where month and day starts with 1
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        let month = time::Month::try_from(month).expect("invalid month number");
        let date = time::Date::from_calendar_date(year, month, day).expect("invalid date");
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
        let diff = (self.0 - other.0).whole_seconds().unsigned_abs() as u32;
        Duration::new(diff)
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
    fn duration_display() {
        assert_eq!(Duration::new(0).to_string(), "00:00:00");
        assert_eq!(Duration::new(83).to_string(), "00:01:23");
        assert_eq!(Duration::new(5 * 60 * 60 + 83).to_string(), "05:01:23");
        assert_eq!(Duration::new(u32::MAX).to_string(), "1193046:28:15");
    }

    #[test]
    fn timestamp_diff() {
        assert_eq!(Timestamp::new(1).diff(&Timestamp::new(2)), Duration::new(1));
        let t1 = Timestamp::new(0);
        let t2 = Timestamp::new(u32::MAX);
        // Order doesn't matter
        assert_eq!(t1.diff(&t2), Duration::new(u32::MAX));
        assert_eq!(t2.diff(&t1), Duration::new(u32::MAX));
    }

    #[test]
    fn timestamp_now() {
        // Just a compilation test to ensure getting current time works on a target platform
        let now = Timestamp::now();
        assert!(now.0 > 0);
    }
}
