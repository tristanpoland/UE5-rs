//! Additional math types and utilities for UE5 compatibility

pub mod int_vector;
pub mod int_vector2;
pub mod plane;
pub mod ray;
pub mod line_segment;

// Re-export all types for convenience
pub use int_vector::*;
pub use int_vector2::*;
pub use plane::*;
pub use ray::*;
pub use line_segment::*;