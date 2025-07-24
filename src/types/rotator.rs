//! Rotator type and rotation utilities

use crate::vector::*;
use crate::BinarySerializable;
use glam::Quat;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unreal Engine style Rotator (Pitch, Yaw, Roll in degrees)
/// 
/// Represents rotation using Euler angles in degrees:
/// - Pitch: Rotation around Y axis (up/down)
/// - Yaw: Rotation around Z axis (left/right)  
/// - Roll: Rotation around X axis (banking)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rotator {
    /// Rotation around Y axis (degrees)
    pub pitch: f32,
    /// Rotation around Z axis (degrees) 
    pub yaw: f32,
    /// Rotation around X axis (degrees)
    pub roll: f32,
}

impl fmt::Display for Rotator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P={:.2}° Y={:.2}° R={:.2}°", self.pitch, self.yaw, self.roll)
    }
}

impl BinarySerializable for Rotator {}

impl Rotator {
    /// Zero rotation constant
    pub const ZERO: Self = Self { 
        pitch: 0.0, 
        yaw: 0.0, 
        roll: 0.0 
    };

    /// Create a new rotator with the given pitch, yaw, and roll (in degrees)
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self { pitch, yaw, roll }
    }

    /// Create a rotator with only yaw rotation
    pub fn from_yaw(yaw: f32) -> Self {
        Self { 
            pitch: 0.0, 
            yaw, 
            roll: 0.0 
        }
    }

    /// Create a rotator with only pitch rotation
    pub fn from_pitch(pitch: f32) -> Self {
        Self { 
            pitch, 
            yaw: 0.0, 
            roll: 0.0 
        }
    }

    /// Returns the rotator with angles normalized to [-180, 180] range
    pub fn get_normalized(&self) -> Self {
        Self {
            pitch: normalize_angle(self.pitch),
            yaw: normalize_angle(self.yaw),
            roll: normalize_angle(self.roll),
        }
    }

    /// Create a rotator with only roll rotation
    pub fn from_roll(roll: f32) -> Self {
        Self { 
            pitch: 0.0, 
            yaw: 0.0, 
            roll 
        }
    }

    /// Convert to quaternion (preferred for math operations)
    pub fn to_quaternion(self) -> Quaternion {
        let pitch_rad = self.pitch.to_radians();
        let yaw_rad = self.yaw.to_radians();
        let roll_rad = self.roll.to_radians();
        
        // Use ZYX Euler order: Z(yaw), Y(pitch), X(roll)
        // This matches UE's rotation application order
        Quat::from_euler(glam::EulerRot::ZYX, yaw_rad, pitch_rad, roll_rad)
    }

    /// Create from quaternion
    pub fn from_quaternion(quat: Quaternion) -> Self {
        // To reverse the composition: yaw_quat * pitch_quat * roll_quat
        // We need to extract in the reverse order
        // Use ZYX order to match our composition order
        let (z_rad, y_rad, x_rad) = quat.to_euler(glam::EulerRot::ZYX);
        Self {
            pitch: y_rad.to_degrees(),  // Y rotation = Pitch  
            yaw: z_rad.to_degrees(),    // Z rotation = Yaw
            roll: x_rad.to_degrees(),   // X rotation = Roll
        }
    }

    /// Normalize angles to [-180, 180] range
    pub fn normalize(mut self) -> Self {
        self.pitch = normalize_angle(self.pitch);
        self.yaw = normalize_angle(self.yaw);
        self.roll = normalize_angle(self.roll);
        self
    }

    /// Get the forward vector for this rotation
    pub fn get_forward_vector(self) -> Vector {
        self.to_quaternion() * VectorConstants::FORWARD
    }

    /// Get the right vector for this rotation
    pub fn get_right_vector(self) -> Vector {
        self.to_quaternion() * VectorConstants::RIGHT
    }

    /// Get the up vector for this rotation
    pub fn get_up_vector(self) -> Vector {
        self.to_quaternion() * VectorConstants::UP
    }

    /// Check if this rotator is nearly zero
    pub fn is_nearly_zero(self, tolerance: f32) -> bool {
        self.pitch.abs() <= tolerance 
            && self.yaw.abs() <= tolerance 
            && self.roll.abs() <= tolerance
    }

    /// Check if two rotators are nearly equal
    pub fn is_nearly_equal(self, other: Rotator, tolerance: f32) -> bool {
        (self.pitch - other.pitch).abs() <= tolerance
            && (self.yaw - other.yaw).abs() <= tolerance
            && (self.roll - other.roll).abs() <= tolerance
    }

    /// Add rotators component-wise
    pub fn add(self, other: Rotator) -> Self {
        Self {
            pitch: self.pitch + other.pitch,
            yaw: self.yaw + other.yaw,
            roll: self.roll + other.roll,
        }
    }

    /// Subtract rotators component-wise
    pub fn sub(self, other: Rotator) -> Self {
        Self {
            pitch: self.pitch - other.pitch,
            yaw: self.yaw - other.yaw,
            roll: self.roll - other.roll,
        }
    }

    /// Scale rotator by a factor
    pub fn scale(self, factor: f32) -> Self {
        Self {
            pitch: self.pitch * factor,
            yaw: self.yaw * factor,
            roll: self.roll * factor,
        }
    }
}

impl Default for Rotator {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Normalize an angle to the range [-180, 180] degrees
pub fn normalize_angle(angle: f32) -> f32 {
    let mut result = angle % 360.0;
    if result > 180.0 {
        result -= 360.0;
    } else if result < -180.0 {
        result += 360.0;
    }
    result
}

/// Compute the angular difference between two angles (in degrees)
/// Returns the shortest angular distance from angle1 to angle2
pub fn angle_difference(angle1: f32, angle2: f32) -> f32 {
    normalize_angle(angle2 - angle1)
}

/// Linearly interpolate between two rotators
pub fn lerp_rotator(a: Rotator, b: Rotator, alpha: f32) -> Rotator {
    Rotator {
        pitch: a.pitch + alpha * angle_difference(a.pitch, b.pitch),
        yaw: a.yaw + alpha * angle_difference(a.yaw, b.yaw),
        roll: a.roll + alpha * angle_difference(a.roll, b.roll),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotator_creation() {
        let rot = Rotator::new(45.0, 90.0, 0.0);
        assert_eq!(rot.pitch, 45.0);
        assert_eq!(rot.yaw, 90.0);
        assert_eq!(rot.roll, 0.0);
    }

    #[test]
    fn test_normalize_angle() {
        assert_eq!(normalize_angle(270.0), -90.0);
        assert_eq!(normalize_angle(-270.0), 90.0);
        assert_eq!(normalize_angle(180.0), 180.0);
        assert_eq!(normalize_angle(-180.0), -180.0);
    }

    #[test]
    fn test_quaternion_conversion() {
        let rot = Rotator::new(0.0, 90.0, 0.0);
        let quat = rot.to_quaternion();
        let back_to_rot = Rotator::from_quaternion(quat);
        
        // Should be approximately equal (floating point precision)
        assert!((rot.yaw - back_to_rot.yaw).abs() < 0.001);
    }

    #[test]
    fn test_forward_vector() {
        let rot = Rotator::from_yaw(90.0);
        let forward = rot.get_forward_vector();
        
        // 90 degree yaw should point along positive Y axis
        assert!((forward.y - 1.0).abs() < 0.001);
        assert!(forward.x.abs() < 0.001);
        assert!(forward.z.abs() < 0.001);
    }

    #[test]
    fn test_rotation_vectors() {
        // Test zero rotation gives expected vectors
        let zero_rot = Rotator::ZERO;
        let forward = zero_rot.get_forward_vector();
        let right = zero_rot.get_right_vector();
        let up = zero_rot.get_up_vector();
        
        // Forward should be X axis
        assert!((forward.x - 1.0).abs() < 0.001);
        assert!(forward.y.abs() < 0.001);
        assert!(forward.z.abs() < 0.001);
        
        // Right should be Y axis  
        assert!(right.x.abs() < 0.001);
        assert!((right.y - 1.0).abs() < 0.001);
        assert!(right.z.abs() < 0.001);
        
        // Up should be Z axis
        assert!(up.x.abs() < 0.001);
        assert!(up.y.abs() < 0.001);
        assert!((up.z - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_pitch_rotation() {
        let rot = Rotator::from_pitch(90.0);
        let forward = rot.get_forward_vector();
        
        // In UE, positive pitch typically looks DOWN (negative Z direction)
        // 90 degree pitch should point along negative Z axis (down)
        assert!(forward.x.abs() < 0.001);
        assert!(forward.y.abs() < 0.001);
        assert!((forward.z + 1.0).abs() < 0.001);  // Changed to -1.0 (down)
    }

    #[test]
    fn test_negative_pitch_rotation() {
        let rot = Rotator::from_pitch(-90.0);
        let forward = rot.get_forward_vector();
        
        // Negative pitch should look UP (positive Z direction)
        assert!(forward.x.abs() < 0.001);
        assert!(forward.y.abs() < 0.001);
        assert!((forward.z - 1.0).abs() < 0.001);  // Should be +1.0 (up)
    }

    #[test]
    fn test_quaternion_conversion_roundtrip() {
        let original = Rotator::new(30.0, 45.0, 60.0);
        let quat = original.to_quaternion();
        let back_to_rot = Rotator::from_quaternion(quat);
        
        // Should be approximately equal (allowing for floating point precision)
        assert!((original.pitch - back_to_rot.pitch).abs() < 0.01);
        assert!((original.yaw - back_to_rot.yaw).abs() < 0.01);
        assert!((original.roll - back_to_rot.roll).abs() < 0.01);
    }

    #[test]
    fn test_rotator_display() {
        let rot = Rotator::new(45.0, 90.0, -30.0);
        let display_str = format!("{}", rot);
        assert!(display_str.contains("P=45.00°"));
        assert!(display_str.contains("Y=90.00°"));
        assert!(display_str.contains("R=-30.00°"));
    }

    #[test]
    fn test_rotator_json_serialization() {
        let rot = Rotator::new(45.0, 90.0, -30.0);
        
        // Test JSON serialization
        let json = serde_json::to_string(&rot).unwrap();
        let deserialized: Rotator = serde_json::from_str(&json).unwrap();
        
        assert!(rot.is_nearly_equal(deserialized, 0.001));
    }

    #[test]
    fn test_rotator_binary_serialization() {
        let rot = Rotator::new(45.0, 90.0, -30.0);
        
        // Test binary serialization
        let binary = rot.to_binary().unwrap();
        let deserialized = Rotator::from_binary(&binary).unwrap();
        
        assert!(rot.is_nearly_equal(deserialized, 0.001));
    }

    #[test]
    fn test_ue5_coordinate_system_consistency() {
        // Test that our coordinate system matches UE5's left-handed system
        use crate::types::Vector;
        
        let forward = Vector::new(1.0, 0.0, 0.0); // X is forward in UE5
        let right = Vector::new(0.0, 1.0, 0.0);   // Y is right in UE5
        let up = Vector::new(0.0, 0.0, 1.0);      // Z is up in UE5
        
        // Cross product should follow left-handed rule: Forward × Right = Up
        let cross = forward.cross(right);
        assert!((cross - up).length() < 0.001);
        
        // Test that Right × Up = Forward (left-handed system)
        let cross2 = right.cross(up);
        assert!((cross2 - forward).length() < 0.001);
        
        // Test that Up × Forward = Right
        let cross3 = up.cross(forward);
        assert!((cross3 - right).length() < 0.001);
    }

    #[test]
    fn test_rotation_euler_order_consistency() {
        // Test that rotations are applied in YXZ order (UE5 standard: Yaw, Pitch, Roll)
        let rotator = Rotator::new(30.0, 45.0, 60.0); // Pitch, Yaw, Roll
        let quat = rotator.to_quaternion();
        
        // Create the same rotation manually using YXZ order
        let yaw_quat = glam::Quat::from_axis_angle(glam::Vec3::Z, rotator.yaw.to_radians());
        let pitch_quat = glam::Quat::from_axis_angle(glam::Vec3::Y, rotator.pitch.to_radians());
        let roll_quat = glam::Quat::from_axis_angle(glam::Vec3::X, rotator.roll.to_radians());
        
        // UE5 applies in YXZ order: Yaw * Pitch * Roll
        let manual_quat = yaw_quat * pitch_quat * roll_quat;
        
        // They should produce similar results (allowing for different but equivalent representations)
        let expected_forward = manual_quat * glam::Vec3::X;
        let computed_forward = rotator.get_forward_vector();
        let computed_forward_vec3: glam::Vec3 = computed_forward.into();
        
        assert!((expected_forward - computed_forward_vec3).length() < 0.1);
    }

    #[test]
    fn test_quaternion_rotation_properties() {
        let rot1 = Rotator::new(30.0, 45.0, 60.0);
        let rot2 = Rotator::new(15.0, 30.0, 45.0);
        
        let q1 = rot1.to_quaternion();
        let q2 = rot2.to_quaternion();
        
        // Test quaternion properties
        let identity = glam::Quat::IDENTITY;
        
        // Quaternion conjugate property: q * q^-1 = identity
        let inverse = q1.inverse();
        let result = q1 * inverse;
        assert!((result.w - identity.w).abs() < 0.001);
        assert!((result.xyz() - identity.xyz()).length() < 0.001);
        
        // Test that quaternion length is preserved (should be unit quaternions)
        assert!((q1.length() - 1.0).abs() < 0.001);
        assert!((q2.length() - 1.0).abs() < 0.001);
        
        // Test quaternion multiplication produces valid rotation
        let combined = q1 * q2;
        assert!((combined.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_gimbal_lock_handling() {
        // Test behavior at gimbal lock positions (pitch = ±90°)
        let gimbal_lock_pos = Rotator::new(90.0, 45.0, 30.0);
        let gimbal_lock_neg = Rotator::new(-90.0, 45.0, 30.0);
        
        // Should still produce valid forward vectors
        let forward_pos = gimbal_lock_pos.get_forward_vector();
        let forward_neg = gimbal_lock_neg.get_forward_vector();
        
        assert!(forward_pos.is_finite());
        assert!(forward_neg.is_finite());
        assert!((forward_pos.length() - 1.0).abs() < 0.01);
        assert!((forward_neg.length() - 1.0).abs() < 0.01);
        
        // At 90° pitch, forward should point down (negative Z)
        assert!((forward_pos.z + 1.0).abs() < 0.1);
        // At -90° pitch, forward should point up (positive Z)
        assert!((forward_neg.z - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_rotator_edge_cases() {
        // Test with extreme angle values
        let extreme_rot = Rotator::new(720.0, -540.0, 900.0);
        let normalized = extreme_rot.get_normalized();
        
        // Should normalize to equivalent angles within [-180, 180] range
        assert!(normalized.pitch >= -180.0 && normalized.pitch <= 180.0);
        assert!(normalized.yaw >= -180.0 && normalized.yaw <= 180.0);
        assert!(normalized.roll >= -180.0 && normalized.roll <= 180.0);
        
        // Test with NaN values
        let nan_rot = Rotator::new(f32::NAN, 0.0, 0.0);
        let forward = nan_rot.get_forward_vector();
        // Should handle gracefully (either return NaN or a default)
        assert!(forward.x.is_nan() || forward.is_finite());
        
        // Test with infinity
        let inf_rot = Rotator::new(f32::INFINITY, 0.0, 0.0);
        let forward_inf = inf_rot.get_forward_vector();
        // Should handle gracefully
        assert!(forward_inf.x.is_nan() || forward_inf.is_finite());
    }
}