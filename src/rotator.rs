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
        
        // UE uses YXZ rotation order
        Quat::from_euler(glam::EulerRot::YXZ, yaw_rad, pitch_rad, roll_rad)
    }

    /// Create from quaternion
    pub fn from_quaternion(quat: Quaternion) -> Self {
        let (yaw, pitch, roll) = quat.to_euler(glam::EulerRot::YXZ);
        Self {
            pitch: pitch.to_degrees(),
            yaw: yaw.to_degrees(),
            roll: roll.to_degrees(),
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
}