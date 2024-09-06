//! Date related functionality

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::time::Duration;

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
}
