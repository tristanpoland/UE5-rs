//! Bounding Sphere for fast collision detection and culling

use crate::BinarySerializable;
use crate::types::{Vector, Transform};
use super::BoundingBox;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Bounding Sphere
/// 
/// Represents a 3D sphere defined by center and radius.
/// Often used for fast collision detection and culling.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingSphere {
    /// Center of the sphere
    pub center: Vector,
    /// Radius of the sphere
    pub radius: f32,
}

impl fmt::Display for BoundingSphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BoundingSphere(Center: ({:.2}, {:.2}, {:.2}), Radius: {:.2})",
            self.center.x, self.center.y, self.center.z, self.radius
        )
    }
}

impl BinarySerializable for BoundingSphere {}

impl BoundingSphere {
    /// Create a new bounding sphere
    pub fn new(center: Vector, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Create a bounding sphere from a bounding box
    pub fn from_box(bbox: BoundingBox) -> Self {
        let center = bbox.center();
        let radius = (bbox.max - center).length();
        Self { center, radius }
    }

    /// Create a bounding sphere that encompasses all given points
    pub fn from_points(points: &[Vector]) -> Self {
        if points.is_empty() {
            return Self::new(Vector::ZERO, 0.0);
        }

        // Simple implementation: use bounding box center and max distance
        let bbox = BoundingBox::from_points(points);
        let center = bbox.center();
        
        let radius = points
            .iter()
            .map(|&point| (point - center).length())
            .fold(0.0f32, f32::max);

        Self { center, radius }
    }

    /// Get the volume of the sphere
    pub fn volume(self) -> f32 {
        (4.0 / 3.0) * std::f32::consts::PI * self.radius.powi(3)
    }

    /// Get the surface area of the sphere
    pub fn surface_area(self) -> f32 {
        4.0 * std::f32::consts::PI * self.radius * self.radius
    }

    /// Check if a point is inside the sphere
    pub fn contains_point(self, point: Vector) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }

    /// Check if another sphere is completely inside this one
    pub fn contains_sphere(self, other: BoundingSphere) -> bool {
        let distance = (other.center - self.center).length();
        distance + other.radius <= self.radius
    }

    /// Check if this sphere intersects with another sphere
    pub fn intersects_sphere(self, other: BoundingSphere) -> bool {
        let distance_squared = (other.center - self.center).length_squared();
        let radii_sum = self.radius + other.radius;
        distance_squared <= radii_sum * radii_sum
    }

    /// Check if this sphere intersects with a bounding box
    pub fn intersects_box(self, bbox: BoundingBox) -> bool {
        let closest_point = bbox.closest_point_to(self.center);
        self.contains_point(closest_point)
    }

    /// Transform the bounding sphere by the given transform
    pub fn transform(self, transform: Transform) -> Self {
        let new_center = transform.transform_point(self.center);
        
        // Calculate the maximum scale factor to determine new radius
        let scale_x = transform.scale.x.abs();
        let scale_y = transform.scale.y.abs();
        let scale_z = transform.scale.z.abs();
        let max_scale = scale_x.max(scale_y).max(scale_z);
        
        Self {
            center: new_center,
            radius: self.radius * max_scale,
        }
    }

    /// Get the distance from a point to the sphere surface (negative if inside)
    pub fn distance_to_point(self, point: Vector) -> f32 {
        (point - self.center).length() - self.radius
    }

    /// Expand the sphere to include a point
    pub fn expand_to_include(self, point: Vector) -> Self {
        let distance = (point - self.center).length();
        if distance <= self.radius {
            return self;
        }

        Self {
            center: self.center,
            radius: distance,
        }
    }

    /// Expand the sphere to include another sphere
    pub fn expand_to_include_sphere(self, other: BoundingSphere) -> Self {
        let distance = (other.center - self.center).length();
        let new_radius = (distance + other.radius).max(self.radius);
        
        Self {
            center: self.center,
            radius: new_radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_sphere_creation() {
        let sphere = BoundingSphere::new(Vector::ZERO, 1.0);
        
        assert!(sphere.contains_point(Vector::new(0.5, 0.0, 0.0)));
        assert!(!sphere.contains_point(Vector::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_sphere_box_intersection() {
        let sphere = BoundingSphere::new(Vector::ZERO, 1.0);
        let bbox = BoundingBox::new(
            Vector::new(0.5, -0.5, -0.5),
            Vector::new(1.5, 0.5, 0.5)
        );
        
        assert!(sphere.intersects_box(bbox));
    }

    #[test]
    fn test_bounding_sphere_display() {
        let sphere = BoundingSphere::new(Vector::new(1.0, 2.0, 3.0), 5.0);
        
        let display_str = format!("{}", sphere);
        assert!(display_str.contains("Center: (1.00, 2.00, 3.00)"));
        assert!(display_str.contains("Radius: 5.00"));
    }

    #[test]
    fn test_bounding_sphere_json_serialization() {
        let sphere = BoundingSphere::new(Vector::new(1.0, 2.0, 3.0), 5.0);
        
        // Test JSON serialization
        let json = serde_json::to_string(&sphere).unwrap();
        let deserialized: BoundingSphere = serde_json::from_str(&json).unwrap();
        
        assert_eq!(sphere, deserialized);
    }

    #[test]
    fn test_bounding_sphere_binary_serialization() {
        let sphere = BoundingSphere::new(Vector::new(1.0, 2.0, 3.0), 5.0);
        
        // Test binary serialization
        let binary = sphere.to_binary().unwrap();
        let deserialized = BoundingSphere::from_binary(&binary).unwrap();
        
        assert_eq!(sphere, deserialized);
    }
}