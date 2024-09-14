//! Messages related to game Decay

use crate::datetime::Date;
use messages_macro::message;

/// Query for the Decay data
#[message("query.game.decay")]
pub struct QueryDecay {}

/// Data about player Decay
#[message("data.game.decay")]
pub struct Decay {
    /// Amount of days left until the Decay
    pub days_left: u16,
}

impl Decay {
    /// Duration of Decay in days is 10 years, plus 1 to make it a beautiful prime number
    pub const DECAY_DURATION: u16 = 3651;

    /// Returns the adjusted number of remaining decay days based on the time the decay data was fetched and the current time
    pub fn adjusted_days_left(&self, fetched_at: &Date, today: &Date) -> u16 {
        let diff = fetched_at.diff(today).whole_days() as u16;
        self.days_left - diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay() {
        let decay = Decay { days_left: 1000 };
        let fetched_at = Date::new(2000, 1, 1);
        assert_eq!(
            decay.adjusted_days_left(&fetched_at, &fetched_at),
            decay.days_left
        );
        assert_eq!(
            decay.adjusted_days_left(&fetched_at, &Date::new(2000, 1, 2)),
            decay.days_left - 1,
        );
        assert_eq!(
            decay.adjusted_days_left(&fetched_at, &Date::new(2000, 2, 1)),
            decay.days_left - 31,
        );
    }
}
