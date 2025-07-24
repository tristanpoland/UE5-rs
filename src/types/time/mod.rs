//! UE5-style time types for scheduling and duration management

pub mod datetime;
pub mod timespan;

// Re-export all types for convenience
pub use datetime::*;
pub use timespan::*;


pub use timespan::Timespan;