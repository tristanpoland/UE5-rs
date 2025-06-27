//! Vector types and utilities

use crate::BinarySerializable;
use glam::{Vec2, Vec3, Vec4};

/// 3D Vector representing position, velocity, direction, etc.
/// This is the most commonly used vector type in UE.
pub type Vector = Vec3;

/// 2D Vector for UI coordinates, texture coordinates, etc.
pub type Vector2D = Vec2;

/// 4D Vector for homogeneous coordinates, RGBA colors, etc.
pub type Vector4 = Vec4;

/// Quaternion for rotations (preferred over Rotator for math operations)
pub type Quaternion = glam::Quat;

/// 3x3 Matrix
pub type Matrix3 = glam::Mat3;

/// 4x4 Matrix for transformations
pub type Matrix4 = glam::Mat4;

// Implement BinarySerializable for glam types
impl BinarySerializable for Vector {}
impl BinarySerializable for Vector2D {}
impl BinarySerializable for Vector4 {}
impl BinarySerializable for Quaternion {}
impl BinarySerializable for Matrix3 {}
impl BinarySerializable for Matrix4 {}

/// UE-style vector constants and utility functions
pub struct VectorConstants;

impl VectorConstants {
    /// Forward vector (positive X)
    pub const FORWARD: Vector = Vec3::X;
    
    /// Right vector (positive Y)  
    pub const RIGHT: Vector = Vec3::Y;
    
    /// Up vector (positive Z)
    pub const UP: Vector = Vec3::Z;
    
    /// Zero vector
    pub const ZERO: Vector = Vec3::ZERO;
    
    /// One vector (1, 1, 1)
    pub const ONE: Vector = Vec3::ONE;
}

/// Extension trait for Vector operations common in UE
pub trait VectorExt {
    /// Get the size (magnitude) of the vector
    fn size(self) -> f32;
    
    /// Get the squared size (magnitude squared) - faster than size()
    fn size_squared(self) -> f32;
    
    /// Check if the vector is nearly zero
    fn is_nearly_zero(self, tolerance: f32) -> bool;
    
    /// Check if the vector is normalized (unit length)
    fn is_normalized(self) -> bool;
    
    /// Get a normalized copy of the vector
    fn get_safe_normal(self, tolerance: f32) -> Vector;
}

impl VectorExt for Vector {
    fn size(self) -> f32 {
        self.length()
    }
    
    fn size_squared(self) -> f32 {
        self.length_squared()
    }
    
    fn is_nearly_zero(self, tolerance: f32) -> bool {
        self.length_squared() <= tolerance * tolerance
    }
    
    fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 0.01
    }
    
    fn get_safe_normal(self, tolerance: f32) -> Vector {
        let square_sum = self.length_squared();
        if square_sum == 1.0 {
            return self;
        } else if square_sum < tolerance * tolerance {
            return Vector::ZERO;
        }
        self.normalize()
    }
}

/// Extension trait for Vector2D operations
pub trait Vector2DExt {
    /// Get the size (magnitude) of the 2D vector
    fn size(self) -> f32;
    
    /// Get the squared size (magnitude squared)
    fn size_squared(self) -> f32;
    
    /// Check if the vector is nearly zero
    fn is_nearly_zero(self, tolerance: f32) -> bool;
}

impl Vector2DExt for Vector2D {
    fn size(self) -> f32 {
        self.length()
    }
    
    fn size_squared(self) -> f32 {
        self.length_squared()
    }
    
    fn is_nearly_zero(self, tolerance: f32) -> bool {
        self.length_squared() <= tolerance * tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_constants() {
        assert_eq!(VectorConstants::FORWARD, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(VectorConstants::RIGHT, Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(VectorConstants::UP, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector_ext() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert_eq!(v.size(), 5.0);
        assert_eq!(v.size_squared(), 25.0);
        assert!(!v.is_nearly_zero(0.1));
        
        let normalized = v.get_safe_normal(0.001);
        assert!(normalized.is_normalized());
    }
}