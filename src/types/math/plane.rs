//! 2D and 3D Plane representation for geometric operations

use crate::vector::{Vector, Vector2D};
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// 2D Plane representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane2D {
    /// Plane normal (should be normalized)
    pub normal: Vector2D,
    /// Distance from origin along normal
    pub distance: f64,
}

impl fmt::Display for Plane2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane2D(Normal: ({:.3}, {:.3}), Distance: {:.3})", 
               self.normal.x, self.normal.y, self.distance)
    }
}

impl BinarySerializable for Plane2D {}

impl Plane2D {
    /// Create a new 2D plane
    pub fn new(normal: Vector2D, distance: f64) -> Self {
        Self { normal, distance }
    }

    /// Create a plane from a point and normal
    pub fn from_point_normal(point: Vector2D, normal: Vector2D) -> Self {
        let normalized = normal.normalize();
        let distance = point.dot(normalized);
        Self::new(normalized, distance)
    }

    /// Get the signed distance from a point to the plane
    pub fn distance_to_point(self, point: Vector2D) -> f64 {
        self.normal.dot(point) - self.distance
    }

    /// Check which side of the plane a point is on
    pub fn point_side(self, point: Vector2D) -> f64 {
        self.distance_to_point(point)
    }

    /// Check if a point is in front of the plane
    pub fn is_point_in_front(self, point: Vector2D) -> bool {
        self.distance_to_point(point) > 0.0
    }

    /// Project a point onto the plane
    pub fn project_point(self, point: Vector2D) -> Vector2D {
        point - self.normal * self.distance_to_point(point)
    }
}

/// 3D Plane representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    /// Plane normal (should be normalized)
    pub normal: Vector,
    /// Distance from origin along normal
    pub distance: f64,
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane(Normal: ({:.3}, {:.3}, {:.3}), Distance: {:.3})", 
               self.normal.x, self.normal.y, self.normal.z, self.distance)
    }
}

impl BinarySerializable for Plane {}

impl Plane {
    /// Create a new 3D plane
    pub fn new(normal: Vector, distance: f64) -> Self {
        Self { normal, distance }
    }

    /// Create a plane from a point and normal
    pub fn from_point_normal(point: Vector, normal: Vector) -> Self {
        let normalized = normal.normalize();
        let distance = point.dot(normalized);
        Self::new(normalized, distance)
    }

    /// Create a plane from three points
    pub fn from_three_points(p1: Vector, p2: Vector, p3: Vector) -> Self {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let normal = v1.cross(v2).normalize();
        Self::from_point_normal(p1, normal)
    }

    /// Get the signed distance from a point to the plane
    pub fn distance_to_point(self, point: Vector) -> f64 {
        self.normal.dot(point) - self.distance
    }

    /// Check which side of the plane a point is on
    pub fn point_side(self, point: Vector) -> f64 {
        self.distance_to_point(point)
    }

    /// Check if a point is in front of the plane
    pub fn is_point_in_front(self, point: Vector) -> bool {
        self.distance_to_point(point) > 0.0
    }

    /// Project a point onto the plane
    pub fn project_point(self, point: Vector) -> Vector {
        point - self.normal * self.distance_to_point(point)
    }

    /// Get the closest point on the plane to a given point
    pub fn closest_point_to(self, point: Vector) -> Vector {
        self.project_point(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_creation() {
        let normal = Vector::new(0.0, 0.0, 1.0);
        let point = Vector::new(1.0, 2.0, 5.0);
        let plane = Plane::from_point_normal(point, normal);
        
        assert_eq!(plane.distance, 5.0);
        assert_eq!(plane.distance_to_point(Vector::new(0.0, 0.0, 5.0)), 0.0);
        assert_eq!(plane.distance_to_point(Vector::new(0.0, 0.0, 7.0)), 2.0);
    }

    #[test]
    fn test_plane_from_three_points() {
        let p1 = Vector::new(0.0, 0.0, 0.0);
        let p2 = Vector::new(1.0, 0.0, 0.0);
        let p3 = Vector::new(0.0, 1.0, 0.0);
        let plane = Plane::from_three_points(p1, p2, p3);
        
        // Normal should point up (Z axis)
        assert!((plane.normal - Vector::new(0.0, 0.0, 1.0)).length() < 0.001);
        assert_eq!(plane.distance, 0.0);
    }

    #[test]
    fn test_plane_point_projection() {
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 5.0);
        let point = Vector::new(3.0, 4.0, 10.0);
        let projected = plane.project_point(point);
        
        assert_eq!(projected, Vector::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_plane2d_operations() {
        let normal = Vector2D::new(1.0, 0.0);
        let plane = Plane2D::from_point_normal(Vector2D::new(5.0, 0.0), normal);
        
        assert_eq!(plane.distance, 5.0);
        assert!(plane.is_point_in_front(Vector2D::new(7.0, 0.0)));
        assert!(!plane.is_point_in_front(Vector2D::new(3.0, 0.0)));
    }

    #[test]
    fn test_plane_display() {
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 5.0);
        let display_str = format!("{}", plane);
        assert!(display_str.contains("Normal: (0.000, 0.000, 1.000)"));
        assert!(display_str.contains("Distance: 5.000"));
    }

    #[test]
    fn test_plane_json_serialization() {
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 5.0);
        let json = serde_json::to_string(&plane).unwrap();
        let deserialized: Plane = serde_json::from_str(&json).unwrap();
        assert_eq!(plane, deserialized);
    }

    #[test]
    fn test_plane_binary_serialization() {
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 5.0);
        let binary = plane.to_binary().unwrap();
        let deserialized = Plane::from_binary(&binary).unwrap();
        assert_eq!(plane, deserialized);
    }
}