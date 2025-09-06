//! 3D Integer Vector for grid coordinates, voxel positions, etc.

use crate::vector::Vector;
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 3D Integer Vector for grid coordinates, voxel positions, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntVector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl fmt::Display for IntVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntVector(X={}, Y={}, Z={})", self.x, self.y, self.z)
    }
}

impl BinarySerializable for IntVector {}

impl IntVector {
    /// Zero vector constant
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    /// Unit vector along X axis
    pub const UNIT_X: Self = Self { x: 1, y: 0, z: 0 };
    /// Unit vector along Y axis
    pub const UNIT_Y: Self = Self { x: 0, y: 1, z: 0 };
    /// Unit vector along Z axis
    pub const UNIT_Z: Self = Self { x: 0, y: 0, z: 1 };
    /// One vector (1, 1, 1)
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };

    /// Create a new 3D integer vector
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Convert to floating point Vector
    pub fn to_vector(self) -> Vector {
        Vector::new(self.x as f64, self.y as f64, self.z as f64)
    }

    /// Create from floating point Vector (rounds to nearest integer)
    pub fn from_vector(v: Vector) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
    }

    /// Get the squared magnitude
    pub fn size_squared(self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the magnitude as floating point
    pub fn size(self) -> f32 {
        (self.size_squared() as f32).sqrt()
    }

    /// Component-wise addition
    pub fn add(self, other: IntVector) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Component-wise subtraction
    pub fn sub(self, other: IntVector) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Scale by an integer factor
    pub fn scale(self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// Cross product with another integer vector
    pub fn cross(self, other: IntVector) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Dot product
    pub fn dot(self, other: IntVector) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Manhattan distance to another point
    pub fn manhattan_distance(self, other: IntVector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

// Operator overloads
impl std::ops::Add for IntVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl std::ops::Sub for IntVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl std::ops::Mul<i32> for IntVector {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.scale(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_vector_operations() {
        let iv1 = IntVector::new(1, 2, 3);
        let iv2 = IntVector::new(4, 5, 6);
        
        assert_eq!(iv1.add(iv2), IntVector::new(5, 7, 9));
        assert_eq!(iv2.sub(iv1), IntVector::new(3, 3, 3));
        assert_eq!(iv1.dot(iv2), 32); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_int_vector_conversion() {
        let fv = Vector::new(3.7, -2.3, 1.8);
        let iv = IntVector::from_vector(fv);
        assert_eq!(iv, IntVector::new(4, -2, 2));
        
        let back_to_fv = iv.to_vector();
        assert_eq!(back_to_fv, Vector::new(4.0, -2.0, 2.0));
    }

    #[test]
    fn test_int_vector_manhattan_distance() {
        let p1 = IntVector::new(0, 0, 0);
        let p2 = IntVector::new(3, 4, 5);
        assert_eq!(p1.manhattan_distance(p2), 12);
    }

    #[test]
    fn test_int_vector_operators() {
        let iv1 = IntVector::new(1, 2, 3);
        let iv2 = IntVector::new(4, 5, 6);
        
        assert_eq!(iv1 + iv2, IntVector::new(5, 7, 9));
        assert_eq!(iv2 - iv1, IntVector::new(3, 3, 3));
        assert_eq!(iv1 * 2, IntVector::new(2, 4, 6));
    }

    #[test]
    fn test_int_vector_display() {
        let iv = IntVector::new(1, 2, 3);
        let display_str = format!("{}", iv);
        assert!(display_str.contains("X=1"));
        assert!(display_str.contains("Y=2"));
        assert!(display_str.contains("Z=3"));
    }

    #[test]
    fn test_int_vector_json_serialization() {
        let iv = IntVector::new(1, 2, 3);
        let json = serde_json::to_string(&iv).unwrap();
        let deserialized: IntVector = serde_json::from_str(&json).unwrap();
        assert_eq!(iv, deserialized);
    }

    #[test]
    fn test_int_vector_binary_serialization() {
        let iv = IntVector::new(1, 2, 3);
        let binary = iv.to_binary().unwrap();
        let deserialized = IntVector::from_binary(&binary).unwrap();
        assert_eq!(iv, deserialized);
    }
}