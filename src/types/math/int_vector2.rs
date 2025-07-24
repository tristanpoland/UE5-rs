//! 2D Integer Vector for grid coordinates, texture coordinates, etc.

use crate::vector::Vector2D;
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 2D Integer Vector for grid coordinates, texture coordinates, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntVector2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for IntVector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntVector2(X={}, Y={})", self.x, self.y)
    }
}

impl BinarySerializable for IntVector2 {}

impl IntVector2 {
    /// Zero vector constant
    pub const ZERO: Self = Self { x: 0, y: 0 };
    /// Unit vector along X axis
    pub const UNIT_X: Self = Self { x: 1, y: 0 };
    /// Unit vector along Y axis
    pub const UNIT_Y: Self = Self { x: 0, y: 1 };
    /// One vector (1, 1)
    pub const ONE: Self = Self { x: 1, y: 1 };

    /// Create a new 2D integer vector
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Convert to floating point Vector2D
    pub fn to_vector2d(self) -> Vector2D {
        Vector2D::new(self.x as f32, self.y as f32)
    }

    /// Create from floating point Vector2D (rounds to nearest integer)
    pub fn from_vector2d(v: Vector2D) -> Self {
        Self::new(v.x.round() as i32, v.y.round() as i32)
    }

    /// Get the squared magnitude
    pub fn size_squared(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    /// Get the magnitude as floating point
    pub fn size(self) -> f32 {
        (self.size_squared() as f32).sqrt()
    }

    /// Component-wise addition
    pub fn add(self, other: IntVector2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    /// Component-wise subtraction
    pub fn sub(self, other: IntVector2) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    /// Scale by an integer factor
    pub fn scale(self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    /// Dot product
    pub fn dot(self, other: IntVector2) -> i32 {
        self.x * other.x + self.y * other.y
    }

    /// Manhattan distance to another point
    pub fn manhattan_distance(self, other: IntVector2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Operator overloads
impl std::ops::Add for IntVector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl std::ops::Sub for IntVector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sub(rhs)
    }
}

impl std::ops::Mul<i32> for IntVector2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.scale(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_vector2_creation() {
        let iv = IntVector2::new(5, -3);
        assert_eq!(iv.x, 5);
        assert_eq!(iv.y, -3);
    }

    #[test]
    fn test_int_vector2_conversion() {
        let fv = Vector2D::new(3.7, -2.3);
        let iv = IntVector2::from_vector2d(fv);
        assert_eq!(iv, IntVector2::new(4, -2));
        
        let back_to_fv = iv.to_vector2d();
        assert_eq!(back_to_fv, Vector2D::new(4.0, -2.0));
    }

    #[test]
    fn test_int_vector2_operations() {
        let iv1 = IntVector2::new(3, 4);
        let iv2 = IntVector2::new(1, 2);
        
        assert_eq!(iv1.add(iv2), IntVector2::new(4, 6));
        assert_eq!(iv1.sub(iv2), IntVector2::new(2, 2));
        assert_eq!(iv1.dot(iv2), 11); // 3*1 + 4*2 = 11
    }

    #[test]
    fn test_int_vector2_manhattan_distance() {
        let p1 = IntVector2::new(0, 0);
        let p2 = IntVector2::new(3, 4);
        assert_eq!(p1.manhattan_distance(p2), 7);
    }

    #[test]
    fn test_int_vector2_operators() {
        let iv1 = IntVector2::new(3, 4);
        let iv2 = IntVector2::new(1, 2);
        
        assert_eq!(iv1 + iv2, IntVector2::new(4, 6));
        assert_eq!(iv1 - iv2, IntVector2::new(2, 2));
        assert_eq!(iv1 * 2, IntVector2::new(6, 8));
    }

    #[test]
    fn test_int_vector2_display() {
        let iv = IntVector2::new(5, -3);
        let display_str = format!("{}", iv);
        assert!(display_str.contains("X=5"));
        assert!(display_str.contains("Y=-3"));
    }

    #[test]
    fn test_int_vector2_json_serialization() {
        let iv = IntVector2::new(5, -3);
        let json = serde_json::to_string(&iv).unwrap();
        let deserialized: IntVector2 = serde_json::from_str(&json).unwrap();
        assert_eq!(iv, deserialized);
    }

    #[test]
    fn test_int_vector2_binary_serialization() {
        let iv = IntVector2::new(5, -3);
        let binary = iv.to_binary().unwrap();
        let deserialized = IntVector2::from_binary(&binary).unwrap();
        assert_eq!(iv, deserialized);
    }
}