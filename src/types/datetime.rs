//! UE5-style DateTime for timestamps and scheduling

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// UE5-style DateTime for timestamps and scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DateTime {
    /// Ticks since epoch (100-nanosecond intervals)
    pub ticks: i64,
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let system_time = self.to_system_time();
        let datetime = chrono::DateTime::<chrono::Utc>::from(system_time);
        write!(f, "DateTime({})", datetime.format("%Y-%m-%d %H:%M:%S UTC"))
    }
}

impl BinarySerializable for DateTime {}

impl DateTime {
    /// Ticks per second (10,000,000 = 100ns intervals)
    pub const TICKS_PER_SECOND: i64 = 10_000_000;
    /// Ticks per millisecond
    pub const TICKS_PER_MILLISECOND: i64 = 10_000;
    /// Ticks per microsecond
    pub const TICKS_PER_MICROSECOND: i64 = 10;
    
    /// Minimum DateTime value
    pub const MIN: Self = Self { ticks: i64::MIN };
    /// Maximum DateTime value
    pub const MAX: Self = Self { ticks: i64::MAX };

    /// Create a DateTime from ticks
    pub fn from_ticks(ticks: i64) -> Self {
        Self { ticks }
    }

    /// Get current DateTime
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);
        let ticks = duration.as_nanos() as i64 / 100; // Convert to 100ns ticks
        Self::from_ticks(ticks)
    }

    /// Create DateTime from Unix timestamp (seconds)
    pub fn from_unix_timestamp(seconds: i64) -> Self {
        Self::from_ticks(seconds * Self::TICKS_PER_SECOND)
    }

    /// Create DateTime from Unix timestamp with milliseconds
    pub fn from_unix_timestamp_millis(millis: i64) -> Self {
        Self::from_ticks(millis * Self::TICKS_PER_MILLISECOND)
    }

    /// Convert to Unix timestamp (seconds)
    pub fn to_unix_timestamp(self) -> i64 {
        self.ticks / Self::TICKS_PER_SECOND
    }

    /// Convert to Unix timestamp (milliseconds)
    pub fn to_unix_timestamp_millis(self) -> i64 {
        self.ticks / Self::TICKS_PER_MILLISECOND
    }

    /// Convert to SystemTime
    pub fn to_system_time(self) -> SystemTime {
        let duration = Duration::from_nanos((self.ticks * 100) as u64);
        UNIX_EPOCH + duration
    }

    /// Add a timespan
    pub fn add_timespan(self, timespan: crate::types::Timespan) -> Self {
        Self::from_ticks(self.ticks + timespan.ticks)
    }

    /// Subtract a timespan
    pub fn sub_timespan(self, timespan: crate::types::Timespan) -> Self {
        Self::from_ticks(self.ticks - timespan.ticks)
    }

    /// Get the difference between two DateTimes as a Timespan
    pub fn difference(self, other: DateTime) -> crate::types::Timespan {
        crate::types::Timespan::from_ticks(self.ticks - other.ticks)
    }

    /// Get the date part (time set to midnight)
    pub fn date(self) -> Self {
        let days = self.ticks / (Self::TICKS_PER_SECOND * 86400);
        Self::from_ticks(days * Self::TICKS_PER_SECOND * 86400)
    }

    /// Get time of day as Timespan since midnight
    pub fn time_of_day(self) -> crate::types::Timespan {
        let ticks_in_day = self.ticks % (Self::TICKS_PER_SECOND * 86400);
        crate::types::Timespan::from_ticks(ticks_in_day)
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime() {
        let dt1 = DateTime::from_unix_timestamp(1000);
        let dt2 = DateTime::from_unix_timestamp(2000);
        
        let diff = dt2.difference(dt1);
        assert_eq!(diff.total_seconds(), 1000.0);
        
        let dt3 = dt1.add_timespan(crate::types::Timespan::from_seconds(500.0));
        assert_eq!(dt3.to_unix_timestamp(), 1500);
    }

    #[test]
    fn test_serialization() {
        let dt = DateTime::now();
        let json = serde_json::to_string(&dt).unwrap();
        let deserialized: DateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, deserialized);
    }
}