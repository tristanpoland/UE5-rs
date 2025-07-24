//! Ray representation for ray casting and intersection tests

use crate::vector::Vector;
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Ray representation for ray casting and intersection tests
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    /// Ray origin point
    pub origin: Vector,
    /// Ray direction (should be normalized)
    pub direction: Vector,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ray(Origin: ({:.3}, {:.3}, {:.3}), Direction: ({:.3}, {:.3}, {:.3}))", 
               self.origin.x, self.origin.y, self.origin.z,
               self.direction.x, self.direction.y, self.direction.z)
    }
}

impl BinarySerializable for Ray {}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { 
            origin, 
            direction: direction.normalize() 
        }
    }

    /// Create a ray from origin to target point
    pub fn from_origin_to_target(origin: Vector, target: Vector) -> Self {
        Self::new(origin, target - origin)
    }

    /// Get a point along the ray at the given distance
    pub fn point_at_distance(self, distance: f32) -> Vector {
        self.origin + self.direction * distance
    }

    /// Alternative name for point_at_distance (UE5 compatibility)
    pub fn point_at(self, distance: f32) -> Vector {
        self.point_at_distance(distance)
    }

    /// Get the closest point on the ray to a given point
    pub fn closest_point_to(self, point: Vector) -> Vector {
        let to_point = point - self.origin;
        let distance = to_point.dot(self.direction).max(0.0);
        self.point_at_distance(distance)
    }

    /// Get the distance along the ray to the closest point to a given point
    pub fn distance_to_closest_point(self, point: Vector) -> f32 {
        let to_point = point - self.origin;
        to_point.dot(self.direction).max(0.0)
    }

    /// Get the shortest distance from the ray to a point
    pub fn distance_to_point(self, point: Vector) -> f32 {
        let closest = self.closest_point_to(point);
        (point - closest).length()
    }

    /// Check if a point is approximately on the ray
    pub fn contains_point(self, point: Vector, tolerance: f32) -> bool {
        self.distance_to_point(point) <= tolerance
    }

    /// Transform the ray by moving its origin and rotating its direction
    pub fn transform(self, transform: &crate::Transform) -> Self {
        Self {
            origin: transform.transform_point(self.origin),
            direction: transform.transform_vector(self.direction).normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        assert_eq!(ray.origin, Vector::ZERO);
        assert_eq!(ray.direction, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_point_at_distance() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let point = ray.point_at_distance(5.0);
        assert_eq!(point, Vector::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_closest_point() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let test_point = Vector::new(3.0, 4.0, 0.0);
        let closest = ray.closest_point_to(test_point);
        assert_eq!(closest, Vector::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn test_ray_distance_to_point() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let test_point = Vector::new(3.0, 4.0, 0.0);
        let distance = ray.distance_to_point(test_point);
        assert_eq!(distance, 4.0);
    }

    #[test]
    fn test_ray_from_origin_to_target() {
        let origin = Vector::new(1.0, 2.0, 3.0);
        let target = Vector::new(4.0, 6.0, 3.0);
        let ray = Ray::from_origin_to_target(origin, target);
        
        assert_eq!(ray.origin, origin);
        // Direction should be normalized
        let expected_dir = Vector::new(3.0, 4.0, 0.0).normalize();
        assert!((ray.direction - expected_dir).length() < 0.001);
    }

    #[test]
    fn test_ray_contains_point() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        
        assert!(ray.contains_point(Vector::new(5.0, 0.0, 0.0), 0.01));
        assert!(!ray.contains_point(Vector::new(5.0, 1.0, 0.0), 0.01));
        assert!(ray.contains_point(Vector::new(5.0, 0.1, 0.0), 0.5));
    }

    #[test]
    fn test_ray_display() {
        let ray = Ray::new(Vector::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let display_str = format!("{}", ray);
        assert!(display_str.contains("Origin: (1.000, 2.000, 3.000)"));
        assert!(display_str.contains("Direction: (0.000, 1.000, 0.000)"));
    }

    #[test]
    fn test_ray_json_serialization() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let json = serde_json::to_string(&ray).unwrap();
        let deserialized: Ray = serde_json::from_str(&json).unwrap();
        assert_eq!(ray, deserialized);
    }

    #[test]
    fn test_ray_binary_serialization() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let binary = ray.to_binary().unwrap();
        let deserialized = Ray::from_binary(&binary).unwrap();
        assert_eq!(ray, deserialized);
    }
}