//! UE5 data types module
//! 
//! This module provides Rust equivalents of common Unreal Engine data types
//! organized into logical categories for easy access and maintenance.

// Core math types
pub mod vector;
pub mod rotator;
pub mod transform;
pub mod math;

// Visual types
pub mod color;

// Geometric types
pub mod bounds;

// Container types
pub mod containers;

// Networking types
pub mod networking;

// Utility types
pub mod time;
pub mod guid;
pub mod name;
pub mod text;
pub mod version;

// Integration tests
mod integration_tests;

// Re-export commonly used types for convenience
pub use vector::*;
pub use rotator::*;
pub use transform::*;
pub use math::*;
pub use color::*;
pub use bounds::*;
pub use containers::*;
pub use networking::*;
pub use time::*;
pub use guid::*;
pub use name::*;
pub use text::*;
pub use version::*;

// Re-export glam types for convenience
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};