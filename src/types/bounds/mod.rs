//! Bounding volume and geometric utility types

pub mod bounding_box;
pub mod bounding_sphere;

// Re-export all types for convenience
pub use bounding_box::*;
pub use bounding_sphere::*;