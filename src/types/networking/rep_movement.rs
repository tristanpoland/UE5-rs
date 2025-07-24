//! Replication information for network movement

use crate::BinarySerializable;
use crate::types::{Vector, Rotator};
use super::NetworkGUID;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Replication information for network movement
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RepMovement {
    /// Current location
    pub location: Vector,
    /// Current rotation
    pub rotation: Rotator,
    /// Current linear velocity
    pub linear_velocity: Vector,
    /// Current angular velocity (pitch, yaw, roll rates)
    pub angular_velocity: Vector,
    /// Location base (for relative movement)
    pub location_base: Option<NetworkGUID>,
    /// Relative location offset
    pub relative_location: Vector,
    /// Server timestamp when this movement was recorded
    pub server_frame: u32,
    /// Whether this movement is simulated physics
    pub is_simulated: bool,
    /// Whether the location base is valid
    pub has_location_base: bool,
}

impl fmt::Display for RepMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RepMovement(Loc: ({:.2}, {:.2}, {:.2}), Rot: {}, Vel: ({:.2}, {:.2}, {:.2}), Frame: {})",
            self.location.x, self.location.y, self.location.z,
            self.rotation,
            self.linear_velocity.x, self.linear_velocity.y, self.linear_velocity.z,
            self.server_frame
        )
    }
}

impl BinarySerializable for RepMovement {}

impl RepMovement {
    /// Create a new RepMovement with default values
    pub fn new() -> Self {
        Self {
            location: Vector::ZERO,
            rotation: Rotator::ZERO,
            linear_velocity: Vector::ZERO,
            angular_velocity: Vector::ZERO,
            location_base: None,
            relative_location: Vector::ZERO,
            server_frame: 0,
            is_simulated: false,
            has_location_base: false,
        }
    }

    /// Create RepMovement from basic transform data
    pub fn from_transform(location: Vector, rotation: Rotator, velocity: Vector) -> Self {
        Self {
            location,
            rotation,
            linear_velocity: velocity,
            angular_velocity: Vector::ZERO,
            location_base: None,
            relative_location: Vector::ZERO,
            server_frame: 0,
            is_simulated: false,
            has_location_base: false,
        }
    }

    /// Set the location base for relative movement
    pub fn set_location_base(&mut self, base_guid: NetworkGUID, relative_location: Vector) {
        self.location_base = Some(base_guid);
        self.relative_location = relative_location;
        self.has_location_base = true;
    }

    /// Clear the location base
    pub fn clear_location_base(&mut self) {
        self.location_base = None;
        self.relative_location = Vector::ZERO;
        self.has_location_base = false;
    }

    /// Get the effective world location (considering location base)
    pub fn get_world_location(&self, base_transform_fn: Option<impl Fn(NetworkGUID) -> Option<Vector>>) -> Vector {
        if self.has_location_base {
            if let Some(base_guid) = self.location_base {
                if let Some(get_transform) = base_transform_fn {
                    if let Some(base_location) = get_transform(base_guid) {
                        return base_location + self.relative_location;
                    }
                }
            }
        }
        self.location
    }

    /// Update the server frame timestamp
    pub fn set_server_frame(&mut self, frame: u32) {
        self.server_frame = frame;
    }

    /// Mark as simulated physics movement
    pub fn set_simulated(&mut self, simulated: bool) {
        self.is_simulated = simulated;
    }
}

impl Default for RepMovement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rep_movement() {
        let mut rep_movement = RepMovement::from_transform(
            Vector::new(10.0, 20.0, 30.0),
            Rotator::new(0.0, 90.0, 0.0),
            Vector::new(5.0, 0.0, 0.0)
        );
        
        assert_eq!(rep_movement.location, Vector::new(10.0, 20.0, 30.0));
        assert_eq!(rep_movement.linear_velocity, Vector::new(5.0, 0.0, 0.0));
        
        let base_guid = NetworkGUID::new(999);
        rep_movement.set_location_base(base_guid, Vector::new(1.0, 2.0, 3.0));
        
        assert!(rep_movement.has_location_base);
        assert_eq!(rep_movement.location_base, Some(base_guid));
        assert_eq!(rep_movement.relative_location, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_json_serialization() {
        let rep_movement = RepMovement::from_transform(
            Vector::new(1.0, 2.0, 3.0),
            Rotator::new(0.0, 90.0, 0.0),
            Vector::new(5.0, 0.0, 0.0)
        );
        let json = serde_json::to_string(&rep_movement).unwrap();
        let deserialized: RepMovement = serde_json::from_str(&json).unwrap();
        assert_eq!(rep_movement, deserialized);
    }
}