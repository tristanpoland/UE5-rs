//! # UE Types Library
//!
//! A Rust library providing common Unreal Engine data types for game servers.
//! 
//! This crate provides Rust equivalents of common Unreal Engine types like:
//! - `Vector`, `Vector2D`, `Vector4`
//! - `Rotator` and `Quaternion`
//! - `Transform`
//! - `Color` and `LinearColor`
//! - `BoundingBox` and other geometric types
//!
//! All types support:
//! - Display formatting
//! - JSON serialization/deserialization with serde
//! - Binary serialization/deserialization with bincode
//! - Built on top of the high-performance `glam` math library

pub mod vector;
pub mod rotator;
pub mod transform;
pub mod color;
pub mod bounds;

// Re-export glam types for convenience
pub use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

// Re-export our custom types
pub use vector::*;
pub use rotator::*;
pub use transform::*;
pub use color::*;
pub use bounds::*;

/// Trait for binary serialization/deserialization
pub trait BinarySerializable: Sized {
    /// Serialize to binary format
    fn to_binary(&self) -> Result<Vec<u8>, bincode::Error>
    where
        Self: serde::Serialize,
    {
        bincode::serialize(self)
    }

    /// Deserialize from binary format
    fn from_binary(data: &[u8]) -> Result<Self, bincode::Error>
    where
        Self: serde::de::DeserializeOwned,
    {
        bincode::deserialize(data)
    }
}