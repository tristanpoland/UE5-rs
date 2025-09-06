//! Axis-Aligned Bounding Box (AABB)

use crate::BinarySerializable;
use crate::types::{Vector, Transform};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Axis-Aligned Bounding Box (AABB)
/// 
/// Represents a 3D bounding box aligned with the coordinate axes.
/// Commonly used for collision detection and spatial partitioning.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Minimum corner of the box
    pub min: Vector,
    /// Maximum corner of the box
    pub max: Vector,
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let center = self.center();
        let size = self.size();
        write!(
            f,
            "BoundingBox(Min: ({:.2}, {:.2}, {:.2}), Max: ({:.2}, {:.2}, {:.2}), Center: ({:.2}, {:.2}, {:.2}), Size: ({:.2}, {:.2}, {:.2}))",
            self.min.x, self.min.y, self.min.z,
            self.max.x, self.max.y, self.max.z,
            center.x, center.y, center.z,
            size.x, size.y, size.z
        )
    }
}

impl BinarySerializable for BoundingBox {}

impl BoundingBox {
    /// Empty bounding box (inverted min/max for initialization)
    pub const EMPTY: Self = Self {
        min: Vector::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        max: Vector::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
    };

    /// Create a new bounding box with the given min and max corners
    pub fn new(min: Vector, max: Vector) -> Self {
        Self { min, max }
    }

    /// Create a bounding box from center and extent (half-size)
    pub fn from_center_and_extent(center: Vector, extent: Vector) -> Self {
        Self {
            min: center - extent,
            max: center + extent,
        }
    }

    /// Create a bounding box from a single point
    pub fn from_point(point: Vector) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    /// Create a bounding box that encompasses all given points
    pub fn from_points(points: &[Vector]) -> Self {
        if points.is_empty() {
            return Self::EMPTY;
        }

        let mut bbox = Self::from_point(points[0]);
        for &point in &points[1..] {
            bbox = bbox.expand_to_include(point);
        }
        bbox
    }

    /// Get the center point of the bounding box
    pub fn center(self) -> Vector {
        (self.min + self.max) * 0.5
    }

    /// Get the extent (half-size) of the bounding box
    pub fn extent(self) -> Vector {
        (self.max - self.min) * 0.5
    }

    /// Get the size (full dimensions) of the bounding box
    pub fn size(self) -> Vector {
        self.max - self.min
    }

    /// Get the volume of the bounding box
    pub fn volume(self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }

    /// Get the surface area of the bounding box
    pub fn surface_area(self) -> f64 {
        let size = self.size();
        2.0 * (size.x * size.y + size.y * size.z + size.z * size.x)
    }

    /// Check if the bounding box is valid (min <= max for all axes)
    pub fn is_valid(self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    /// Check if the bounding box is empty (has zero or negative volume)
    pub fn is_empty(self) -> bool {
        self.min.x >= self.max.x || self.min.y >= self.max.y || self.min.z >= self.max.z
    }

    /// Check if a point is inside the bounding box
    pub fn contains_point(self, point: Vector) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
            && point.z >= self.min.z && point.z <= self.max.z
    }

    /// Check if another bounding box is completely inside this one
    pub fn contains_box(self, other: BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    /// Check if this bounding box intersects with another
    pub fn intersects(self, other: BoundingBox) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x
            && self.min.y <= other.max.y && self.max.y >= other.min.y
            && self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    /// Expand the bounding box to include a point
    pub fn expand_to_include(self, point: Vector) -> Self {
        Self {
            min: self.min.min(point),
            max: self.max.max(point),
        }
    }

    /// Expand the bounding box to include another bounding box
    pub fn expand_to_include_box(self, other: BoundingBox) -> Self {
        if other.is_empty() {
            return self;
        }
        if self.is_empty() {
            return other;
        }
        
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Expand the bounding box by a given amount in all directions
    pub fn expand_by(self, amount: f64) -> Self {
        let expansion = Vector::splat(amount);
        Self {
            min: self.min - expansion,
            max: self.max + expansion,
        }
    }

    /// Get the intersection of two bounding boxes
    pub fn intersection(self, other: BoundingBox) -> Self {
        if !self.intersects(other) {
            return Self::EMPTY;
        }

        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    /// Transform the bounding box by the given transform
    pub fn transform(self, transform: Transform) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        // Transform all 8 corners of the box
        let corners = [
            Vector::new(self.min.x, self.min.y, self.min.z),
            Vector::new(self.max.x, self.min.y, self.min.z),
            Vector::new(self.min.x, self.max.y, self.min.z),
            Vector::new(self.max.x, self.max.y, self.min.z),
            Vector::new(self.min.x, self.min.y, self.max.z),
            Vector::new(self.max.x, self.min.y, self.max.z),
            Vector::new(self.min.x, self.max.y, self.max.z),
            Vector::new(self.max.x, self.max.y, self.max.z),
        ];

        let transformed_corners: Vec<Vector> = corners
            .iter()
            .map(|&corner| transform.transform_point(corner))
            .collect();

        Self::from_points(&transformed_corners)
    }

    /// Get the distance from a point to the bounding box (0 if inside)
    pub fn distance_to_point(self, point: Vector) -> f64 {
        let closest = point.clamp(self.min, self.max);
        (point - closest).length()
    }

    /// Get the closest point on the bounding box to a given point
    pub fn closest_point_to(self, point: Vector) -> Vector {
        point.clamp(self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_creation() {
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -1.0, -1.0),
            Vector::new(1.0, 1.0, 1.0)
        );
        
        assert_eq!(bbox.center(), Vector::ZERO);
        assert_eq!(bbox.extent(), Vector::ONE);
        assert_eq!(bbox.size(), Vector::splat(2.0));
    }

    #[test]
    fn test_bounding_box_contains() {
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -1.0, -1.0),
            Vector::new(1.0, 1.0, 1.0)
        );
        
        assert!(bbox.contains_point(Vector::ZERO));
        assert!(bbox.contains_point(Vector::new(0.5, 0.5, 0.5)));
        assert!(!bbox.contains_point(Vector::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_bounding_box_intersection() {
        let bbox1 = BoundingBox::new(
            Vector::new(-1.0, -1.0, -1.0),
            Vector::new(1.0, 1.0, 1.0)
        );
        let bbox2 = BoundingBox::new(
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(2.0, 2.0, 2.0)
        );
        
        assert!(bbox1.intersects(bbox2));
        
        let intersection = bbox1.intersection(bbox2);
        assert_eq!(intersection.min, Vector::ZERO);
        assert_eq!(intersection.max, Vector::ONE);
    }

    #[test]
    fn test_bounding_box_display() {
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -2.0, -3.0),
            Vector::new(1.0, 2.0, 3.0)
        );
        
        let display_str = format!("{}", bbox);
        assert!(display_str.contains("Min: (-1.00, -2.00, -3.00)"));
        assert!(display_str.contains("Max: (1.00, 2.00, 3.00)"));
        assert!(display_str.contains("Center: (0.00, 0.00, 0.00)"));
        assert!(display_str.contains("Size: (2.00, 4.00, 6.00)"));
    }

    #[test]
    fn test_bounding_box_json_serialization() {
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -2.0, -3.0),
            Vector::new(1.0, 2.0, 3.0)
        );
        
        // Test JSON serialization
        let json = serde_json::to_string(&bbox).unwrap();
        let deserialized: BoundingBox = serde_json::from_str(&json).unwrap();
        
        assert_eq!(bbox, deserialized);
    }

    #[test]
    fn test_bounding_box_binary_serialization() {
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -2.0, -3.0),
            Vector::new(1.0, 2.0, 3.0)
        );
        
        // Test binary serialization
        let binary = bbox.to_binary().unwrap();
        let deserialized = BoundingBox::from_binary(&binary).unwrap();
        
        assert_eq!(bbox, deserialized);
    }
}