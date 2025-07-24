//! Transform type for representing object transformations

use crate::vector::*;
use crate::rotator::*;
use crate::BinarySerializable;
use glam::Mat4;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Transform containing Location, Rotation, and Scale
/// 
/// This represents a complete 3D transformation including:
/// - Location: 3D position in world space
/// - Rotation: 3D rotation as a quaternion
/// - Scale: 3D scale factors
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// 3D position/location
    pub location: Vector,
    /// 3D rotation as quaternion
    pub rotation: Quaternion,
    /// 3D scale factors
    pub scale: Vector,
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Location: ({:.2}, {:.2}, {:.2}), Rotation: {}, Scale: ({:.2}, {:.2}, {:.2})",
            self.location.x, self.location.y, self.location.z,
            self.get_rotator(),
            self.scale.x, self.scale.y, self.scale.z
        )
    }
}

impl BinarySerializable for Transform {}

impl Transform {
    /// Identity transform (no translation, rotation, or scaling)
    pub const IDENTITY: Self = Self {
        location: Vector::ZERO,
        rotation: Quaternion::IDENTITY,
        scale: Vector::ONE,
    };

    /// Create a new transform with the given location, rotation, and scale
    pub fn new(location: Vector, rotation: Quaternion, scale: Vector) -> Self {
        Self { location, rotation, scale }
    }

    /// Create a transform with only location (identity rotation and scale)
    pub fn from_location(location: Vector) -> Self {
        Self {
            location,
            ..Self::IDENTITY
        }
    }

    /// Create a transform with only rotation (zero location, identity scale)
    pub fn from_rotation(rotation: Quaternion) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    /// Create a transform with only scale (zero location, identity rotation)
    pub fn from_scale(scale: Vector) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    /// Create a transform with uniform scale
    pub fn from_uniform_scale(scale: f32) -> Self {
        Self {
            scale: Vector::splat(scale),
            ..Self::IDENTITY
        }
    }

    /// Create a transform from location and rotator
    pub fn from_location_rotator(location: Vector, rotator: Rotator) -> Self {
        Self {
            location,
            rotation: rotator.to_quaternion(),
            scale: Vector::ONE,
        }
    }

    /// Create a transform from location, rotator, and scale
    pub fn from_location_rotator_scale(location: Vector, rotator: Rotator, scale: Vector) -> Self {
        Self {
            location,
            rotation: rotator.to_quaternion(),
            scale,
        }
    }

    /// Convert to 4x4 transformation matrix
    pub fn to_matrix(self) -> Matrix4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.location)
    }

    /// Create transform from a 4x4 matrix
    pub fn from_matrix(matrix: Matrix4) -> Self {
        let (scale, rotation, location) = matrix.to_scale_rotation_translation();
        Self { location, rotation, scale }
    }

    /// Get the rotator representation of the rotation
    pub fn get_rotator(self) -> Rotator {
        Rotator::from_quaternion(self.rotation)
    }

    /// Set rotation using a rotator
    pub fn set_rotator(&mut self, rotator: Rotator) {
        self.rotation = rotator.to_quaternion();
    }

    /// Transform a point by this transform (applies scale, rotation, and translation)
    pub fn transform_point(self, point: Vector) -> Vector {
        self.to_matrix().transform_point3(point)
    }

    /// Transform a vector by this transform (applies scale and rotation, ignores translation)
    pub fn transform_vector(self, vector: Vector) -> Vector {
        self.rotation * (vector * self.scale)
    }

    /// Transform a direction vector (applies only rotation, ignores scale and translation)
    pub fn transform_direction(self, direction: Vector) -> Vector {
        self.rotation * direction
    }

    /// Get the inverse of this transform
    pub fn inverse(self) -> Self {
        let inv_matrix = self.to_matrix().inverse();
        Self::from_matrix(inv_matrix)
    }

    /// Combine this transform with another (this transform is applied first)
    pub fn combine(self, other: Transform) -> Self {
        let combined_matrix = other.to_matrix() * self.to_matrix();
        Self::from_matrix(combined_matrix)
    }

    /// Get the forward vector for this transform
    pub fn get_forward_vector(self) -> Vector {
        self.transform_direction(VectorConstants::FORWARD)
    }

    /// Get the right vector for this transform
    pub fn get_right_vector(self) -> Vector {
        self.transform_direction(VectorConstants::RIGHT)
    }

    /// Get the up vector for this transform
    pub fn get_up_vector(self) -> Vector {
        self.transform_direction(VectorConstants::UP)
    }

    /// Check if this transform is nearly equal to another
    pub fn is_nearly_equal(self, other: Transform, tolerance: f32) -> bool {
        (self.location - other.location).length() <= tolerance
            && self.rotation.abs_diff_eq(other.rotation, tolerance)
            && (self.scale - other.scale).length() <= tolerance
    }

    /// Check if this transform is nearly the identity transform
    pub fn is_nearly_identity(self, tolerance: f32) -> bool {
        self.is_nearly_equal(Self::IDENTITY, tolerance)
    }

    /// Linearly interpolate between two transforms
    pub fn lerp(self, other: Transform, alpha: f32) -> Self {
        Self {
            location: self.location.lerp(other.location, alpha),
            rotation: self.rotation.slerp(other.rotation, alpha),
            scale: self.scale.lerp(other.scale, alpha),
        }
    }

    /// Add translation to this transform
    pub fn add_location(mut self, delta: Vector) -> Self {
        self.location += delta;
        self
    }

    /// Add rotation to this transform
    pub fn add_rotation(mut self, delta_rotation: Quaternion) -> Self {
        self.rotation = delta_rotation * self.rotation;
        self
    }

    /// Add uniform scale to this transform
    pub fn add_uniform_scale(mut self, delta_scale: f32) -> Self {
        self.scale *= delta_scale;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_identity() {
        let transform = Transform::IDENTITY;
        let point = Vector::new(1.0, 2.0, 3.0);
        let transformed = transform.transform_point(point);
        
        // Identity transform should not change the point
        assert!((transformed - point).length() < 0.001);
    }

    #[test]
    fn test_transform_location() {
        let transform = Transform::from_location(Vector::new(10.0, 20.0, 30.0));
        let point = Vector::ZERO;
        let transformed = transform.transform_point(point);
        
        assert_eq!(transformed, Vector::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn test_transform_matrix_roundtrip() {
        let original = Transform::new(
            Vector::new(1.0, 2.0, 3.0),
            Quaternion::from_rotation_y(45.0_f32.to_radians()),
            Vector::new(2.0, 2.0, 2.0),
        );
        
        let matrix = original.to_matrix();
        let reconstructed = Transform::from_matrix(matrix);
        
        assert!(original.is_nearly_equal(reconstructed, 0.001));
    }

    #[test]
    fn test_transform_combine() {
        let t1 = Transform::from_location(Vector::new(10.0, 0.0, 0.0));
        let t2 = Transform::from_location(Vector::new(0.0, 20.0, 0.0));
        
        let combined = t1.combine(t2);
        let point = Vector::ZERO;
        let result = combined.transform_point(point);
        
        assert_eq!(result, Vector::new(10.0, 20.0, 0.0));
    }

    #[test]
    fn test_transform_inverse() {
        let transform = Transform::new(
            Vector::new(10.0, 20.0, 30.0),
            Quaternion::from_rotation_z(90.0_f32.to_radians()),
            Vector::splat(2.0),
        );
        
        let inverse = transform.inverse();
        let combined = transform.combine(inverse);
        
        assert!(combined.is_nearly_identity(0.001));
    }

    #[test]
    fn test_transform_display() {
        let transform = Transform::new(
            Vector::new(10.0, 20.0, 30.0),
            Quaternion::from_rotation_y(45.0_f32.to_radians()),
            Vector::new(2.0, 2.0, 2.0),
        );
        
        let display_str = format!("{}", transform);
        assert!(display_str.contains("Location: (10.00, 20.00, 30.00)"));
        assert!(display_str.contains("Scale: (2.00, 2.00, 2.00)"));
    }

    #[test]
    fn test_transform_json_serialization() {
        let transform = Transform::from_location(Vector::new(1.0, 2.0, 3.0));
        
        // Test JSON serialization
        let json = serde_json::to_string(&transform).unwrap();
        let deserialized: Transform = serde_json::from_str(&json).unwrap();
        
        assert!(transform.is_nearly_equal(deserialized, 0.001));
    }

    #[test]
    fn test_transform_binary_serialization() {
        let transform = Transform::new(
            Vector::new(10.0, 20.0, 30.0),
            Quaternion::from_rotation_z(90.0_f32.to_radians()),
            Vector::splat(2.0),
        );
        
        // Test binary serialization
        let binary = transform.to_binary().unwrap();
        let deserialized = Transform::from_binary(&binary).unwrap();
        
        assert!(transform.is_nearly_equal(deserialized, 0.001));
    }
}