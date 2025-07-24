//! UE5-style Timespan for durations and time intervals

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// UE5-style Timespan for durations and time intervals
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timespan {
    /// Ticks (100-nanosecond intervals)
    pub ticks: i64,
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_seconds = self.total_seconds();
        let days = total_seconds / 86400.0;
        let hours = (total_seconds % 86400.0) / 3600.0;
        let minutes = (total_seconds % 3600.0) / 60.0;
        let seconds = total_seconds % 60.0;

        if days >= 1.0 {
            write!(f, "Timespan({:.0}d {:.0}h {:.0}m {:.1}s)", days.floor(), hours.floor(), minutes.floor(), seconds)
        } else if hours >= 1.0 {
            write!(f, "Timespan({:.0}h {:.0}m {:.1}s)", hours.floor(), minutes.floor(), seconds)
        } else if minutes >= 1.0 {
            write!(f, "Timespan({:.0}m {:.1}s)", minutes.floor(), seconds)
        } else {
            write!(f, "Timespan({:.3}s)", seconds)
        }
    }
}

impl BinarySerializable for Timespan {}

impl Timespan {
    /// Zero timespan
    pub const ZERO: Self = Self { ticks: 0 };
    /// Maximum timespan
    pub const MAX: Self = Self { ticks: i64::MAX };
    /// Minimum timespan (negative max)
    pub const MIN: Self = Self { ticks: i64::MIN };

    /// Create a Timespan from ticks
    pub fn from_ticks(ticks: i64) -> Self {
        Self { ticks }
    }

    /// Create Timespan from days
    pub fn from_days(days: f64) -> Self {
        Self::from_ticks((days * 86400.0 * super::DateTime::TICKS_PER_SECOND as f64) as i64)
    }

    /// Create Timespan from hours
    pub fn from_hours(hours: f64) -> Self {
        Self::from_ticks((hours * 3600.0 * super::DateTime::TICKS_PER_SECOND as f64) as i64)
    }

    /// Create Timespan from minutes
    pub fn from_minutes(minutes: f64) -> Self {
        Self::from_ticks((minutes * 60.0 * super::DateTime::TICKS_PER_SECOND as f64) as i64)
    }

    /// Create Timespan from seconds
    pub fn from_seconds(seconds: f64) -> Self {
        Self::from_ticks((seconds * super::DateTime::TICKS_PER_SECOND as f64) as i64)
    }

    /// Create Timespan from milliseconds
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Self::from_ticks((milliseconds * super::DateTime::TICKS_PER_MILLISECOND as f64) as i64)
    }

    /// Get total days as floating point
    pub fn total_days(self) -> f64 {
        self.ticks as f64 / (super::DateTime::TICKS_PER_SECOND as f64 * 86400.0)
    }

    /// Get total hours as floating point
    pub fn total_hours(self) -> f64 {
        self.ticks as f64 / (super::DateTime::TICKS_PER_SECOND as f64 * 3600.0)
    }

    /// Get total minutes as floating point
    pub fn total_minutes(self) -> f64 {
        self.ticks as f64 / (super::DateTime::TICKS_PER_SECOND as f64 * 60.0)
    }

    /// Get total seconds as floating point
    pub fn total_seconds(self) -> f64 {
        self.ticks as f64 / super::DateTime::TICKS_PER_SECOND as f64
    }

    /// Get total milliseconds as floating point
    pub fn total_milliseconds(self) -> f64 {
        self.ticks as f64 / super::DateTime::TICKS_PER_MILLISECOND as f64
    }

    /// Add another timespan
    pub fn add(self, other: Timespan) -> Self {
        Self::from_ticks(self.ticks + other.ticks)
    }

    /// Subtract another timespan
    pub fn subtract(self, other: Timespan) -> Self {
        Self::from_ticks(self.ticks - other.ticks)
    }

    /// Get absolute value
    pub fn abs(self) -> Self {
        Self::from_ticks(self.ticks.abs())
    }

    /// Get duration (always positive)
    pub fn duration(self) -> Self {
        self.abs()
    }

    /// Check if timespan is negative
    pub fn is_negative(self) -> bool {
        self.ticks < 0
    }

    /// Check if timespan is positive
    pub fn is_positive(self) -> bool {
        self.ticks > 0
    }

    /// Check if timespan is zero
    pub fn is_zero(self) -> bool {
        self.ticks == 0
    }
}

impl Default for Timespan {
    fn default() -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timespan() {
        let ts = Timespan::from_hours(2.5);
        assert_eq!(ts.total_minutes(), 150.0);
        assert_eq!(ts.total_seconds(), 9000.0);
        
        let ts2 = Timespan::from_minutes(30.0);
        let sum = ts.add(ts2);
        assert_eq!(sum.total_hours(), 3.0);
    }
}