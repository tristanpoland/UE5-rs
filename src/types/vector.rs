//! Vector types and utilities

use crate::BinarySerializable;
use glam::{DVec2, DVec3, DVec4};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 3D Vector representing position, velocity, direction, etc.
/// This is the most commonly used vector type in UE.
pub type Vector = glam::DVec3;

/// 2D Vector for UI coordinates, texture coordinates, etc.
pub type Vector2D = glam::DVec2;

/// 4D Vector for homogeneous coordinates, RGBA colors, etc.
pub type Vector4 = glam::DVec4;

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
    pub const FORWARD: Vector = DVec3::X;
    
    /// Right vector (positive Y)  
    pub const RIGHT: Vector = DVec3::Y;
    
    /// Up vector (positive Z)
    pub const UP: Vector = DVec3::Z;
    
    /// Zero vector
    pub const ZERO: Vector = DVec3::ZERO;
    
    /// One vector (1, 1, 1)
    pub const ONE: Vector = DVec3::ONE;
}

/// Extension trait for Vector operations common in UE
pub trait VectorExt {
    /// Get the size (magnitude) of the vector
    fn size(self) -> f64;
    
    /// Get the squared size (magnitude squared) - faster than size()
    fn size_squared(self) -> f64;
    
    /// Check if the vector is nearly zero
    fn is_nearly_zero(self, tolerance: f64) -> bool;
    
    /// Check if the vector is normalized (unit length)
    fn is_normalized(self) -> bool;
    
    /// Get a normalized copy of the vector
    fn get_safe_normal(self, tolerance: f64) -> Vector;
}

impl VectorExt for Vector {
    fn size(self) -> f64 {
        self.length()
    }
    
    fn size_squared(self) -> f64 {
        self.length_squared()
    }
    
    fn is_nearly_zero(self, tolerance: f64) -> bool {
        self.length_squared() <= tolerance * tolerance
    }
    
    fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 0.01
    }
    
    fn get_safe_normal(self, tolerance: f64) -> Vector {
        let square_sum = self.length_squared();
        if square_sum == 1.0 {
            return self;
        } else if square_sum < tolerance * tolerance {
            return DVec3::ZERO;
        } else if !square_sum.is_finite() || square_sum.is_nan() {
            return DVec3::ZERO;
        }
        let len = square_sum.sqrt();
        if !len.is_finite() || len.is_nan() || len == 0.0 {
            return DVec3::ZERO;
        }
        let norm = self / len;
        // Clamp to unit length if overflow occurred
        if norm.length().is_infinite() || norm.length().is_nan() {
            return DVec3::ZERO;
        }
        norm
    }
}

/// Extension trait for Vector2D operations
pub trait Vector2DExt {
    /// Get the size (magnitude) of the 2D vector
    fn size(self) -> f64;
    
    /// Get the squared size (magnitude squared)
    fn size_squared(self) -> f64;
    
    /// Check if the vector is nearly zero
    fn is_nearly_zero(self, tolerance: f64) -> bool;
}

impl Vector2DExt for Vector2D {
    fn size(self) -> f64 {
        self.length()
    }
    
    fn size_squared(self) -> f64 {
        self.length_squared()
    }
    
    fn is_nearly_zero(self, tolerance: f64) -> bool {
        self.length_squared() <= tolerance * tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_constants() {
        assert_eq!(VectorConstants::FORWARD, DVec3::new(1.0, 0.0, 0.0));
        assert_eq!(VectorConstants::RIGHT, DVec3::new(0.0, 1.0, 0.0));
        assert_eq!(VectorConstants::UP, DVec3::new(0.0, 0.0, 1.0));
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

    #[test]
    fn test_vector_edge_cases() {
        // Test with NaN values
        let nan_vec = Vector::new(f64::NAN, 0.0, 0.0);
        assert!(nan_vec.x.is_nan());
        assert!(nan_vec.normalize().x.is_nan());
        assert!(nan_vec.length().is_nan());
        
        // Test with infinity
        let inf_vec = Vector::new(f64::INFINITY, 0.0, 0.0);
        assert!(inf_vec.x.is_infinite());
        assert!(!inf_vec.is_finite());
        assert!(inf_vec.length().is_infinite());
        
        // Test normalization of zero vector - get_safe_normal should handle this
        let zero_normalized = Vector::ZERO.get_safe_normal(0.001);
        assert_eq!(zero_normalized, Vector::ZERO);
        
        // Test very small vectors
        let tiny_vec = Vector::new(f64::EPSILON, f64::EPSILON, f64::EPSILON);
        let tiny_safe_normal = tiny_vec.get_safe_normal(0.001);
        assert_eq!(tiny_safe_normal, Vector::ZERO); // Should return zero for tiny vectors
        
        // Test very large vectors
        let huge_vec = Vector::new(f64::MAX / 2.0, f64::MAX / 2.0, f64::MAX / 2.0);
        let huge_normalized = huge_vec.get_safe_normal(0.001);
        // Should handle overflow gracefully by returning zero vector or a normalized vector
        assert!(huge_normalized.is_normalized() || huge_normalized == Vector::ZERO);
    }

    #[test]
    fn test_vector_mathematical_properties() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let v3 = Vector::new(7.0, 8.0, 9.0);
        
        // Test associativity: (v1 + v2) + v3 = v1 + (v2 + v3)
        let left = (v1 + v2) + v3;
        let right = v1 + (v2 + v3);
        assert!((left - right).length() < f64::EPSILON);
        
        // Test commutativity: v1 + v2 = v2 + v1
        assert_eq!(v1 + v2, v2 + v1);
        
        // Test distributivity: a * (v1 + v2) = a * v1 + a * v2
        let scalar = 2.5;
        let left = scalar * (v1 + v2);
        let right = scalar * v1 + scalar * v2;
        assert!((left - right).length() < f64::EPSILON);
        
        // Test dot product properties
        assert_eq!(v1.dot(v2), v2.dot(v1)); // Commutative
        assert_eq!(v1.dot(v1), v1.length_squared()); // Self dot product
        
        // Test cross product properties  
        let cross1 = v1.cross(v2);
        let cross2 = v2.cross(v1);
        assert_eq!(cross1, -cross2); // Anti-commutative
        assert!(cross1.dot(v1).abs() < 0.001); // Perpendicular to both vectors
        assert!(cross1.dot(v2).abs() < 0.001);
    }
}