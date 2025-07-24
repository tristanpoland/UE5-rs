//! Line segment representation for geometric operations

use crate::vector::Vector;
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Line segment representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineSegment {
    /// Start point of the line segment
    pub start: Vector,
    /// End point of the line segment
    pub end: Vector,
}

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LineSegment(Start: ({:.3}, {:.3}, {:.3}), End: ({:.3}, {:.3}, {:.3}))", 
               self.start.x, self.start.y, self.start.z,
               self.end.x, self.end.y, self.end.z)
    }
}

impl BinarySerializable for LineSegment {}

impl LineSegment {
    /// Create a new line segment
    pub fn new(start: Vector, end: Vector) -> Self {
        Self { start, end }
    }

    /// Get the direction vector of the line segment (not normalized)
    pub fn direction_vector(self) -> Vector {
        self.end - self.start
    }

    /// Get the normalized direction vector of the line segment
    pub fn direction(self) -> Vector {
        (self.end - self.start).normalize()
    }

    /// Get the length of the line segment
    pub fn length(self) -> f32 {
        (self.end - self.start).length()
    }

    /// Get the squared length of the line segment (faster than length())
    pub fn length_squared(self) -> f32 {
        (self.end - self.start).length_squared()
    }

    /// Get the center point of the line segment
    pub fn center(self) -> Vector {
        (self.start + self.end) * 0.5
    }

    /// Get a point along the line segment (t=0 is start, t=1 is end)
    pub fn lerp(self, t: f32) -> Vector {
        self.start.lerp(self.end, t.clamp(0.0, 1.0))
    }

    /// Get a point along the line segment without clamping t
    pub fn lerp_unclamped(self, t: f32) -> Vector {
        self.start.lerp(self.end, t)
    }

    /// Get the closest point on the line segment to a given point
    pub fn closest_point_to(self, point: Vector) -> Vector {
        let segment_vec = self.end - self.start;
        let segment_length_squared = segment_vec.length_squared();
        
        if segment_length_squared < f32::EPSILON {
            // Degenerate segment (start == end)
            return self.start;
        }
        
        let to_point = point - self.start;
        let t = to_point.dot(segment_vec) / segment_length_squared;
        self.lerp(t)
    }

    /// Get the distance from the line segment to a point
    pub fn distance_to_point(self, point: Vector) -> f32 {
        let closest = self.closest_point_to(point);
        (point - closest).length()
    }

    /// Get the squared distance from the line segment to a point (faster)
    pub fn distance_squared_to_point(self, point: Vector) -> f32 {
        let closest = self.closest_point_to(point);
        (point - closest).length_squared()
    }

    /// Check if a point is approximately on the line segment
    pub fn contains_point(self, point: Vector, tolerance: f32) -> bool {
        self.distance_to_point(point) <= tolerance
    }

    /// Extend the line segment by a given distance in both directions
    pub fn extend(self, distance: f32) -> Self {
        let direction = self.direction();
        Self {
            start: self.start - direction * distance,
            end: self.end + direction * distance,
        }
    }

    /// Scale the line segment from its center by a factor
    pub fn scale_from_center(self, factor: f32) -> Self {
        let center = self.center();
        let half_vec = (self.end - self.start) * 0.5 * factor;
        Self {
            start: center - half_vec,
            end: center + half_vec,
        }
    }

    /// Get the closest points between this line segment and another
    pub fn closest_points_to_segment(self, other: LineSegment) -> (Vector, Vector) {
        // This is a simplified implementation - for production use,
        // a more sophisticated algorithm might be needed
        let point_on_self = self.closest_point_to(other.center());
        let point_on_other = other.closest_point_to(point_on_self);
        (point_on_self, point_on_other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_segment_creation() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        assert_eq!(segment.start, Vector::ZERO);
        assert_eq!(segment.end, Vector::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_segment_length() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        assert_eq!(segment.length(), 10.0);
        assert_eq!(segment.length_squared(), 100.0);
    }

    #[test]
    fn test_line_segment_lerp() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        let midpoint = segment.lerp(0.5);
        assert_eq!(midpoint, Vector::new(5.0, 0.0, 0.0));
        
        // Test clamping
        let clamped = segment.lerp(-0.5);
        assert_eq!(clamped, Vector::ZERO);
        
        let clamped2 = segment.lerp(1.5);
        assert_eq!(clamped2, Vector::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_segment_closest_point() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        let test_point = Vector::new(3.0, 4.0, 0.0);
        let closest = segment.closest_point_to(test_point);
        assert_eq!(closest, Vector::new(3.0, 0.0, 0.0));
        assert_eq!(segment.distance_to_point(test_point), 4.0);
    }

    #[test]
    fn test_line_segment_center() {
        let segment = LineSegment::new(Vector::new(2.0, 4.0, 6.0), Vector::new(8.0, 12.0, 18.0));
        let center = segment.center();
        assert_eq!(center, Vector::new(5.0, 8.0, 12.0));
    }

    #[test]
    fn test_line_segment_direction() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(3.0, 4.0, 0.0));
        let direction = segment.direction();
        let expected = Vector::new(3.0, 4.0, 0.0).normalize();
        assert!((direction - expected).length() < 0.001);
    }

    #[test]
    fn test_line_segment_contains_point() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        
        assert!(segment.contains_point(Vector::new(5.0, 0.0, 0.0), 0.01));
        assert!(!segment.contains_point(Vector::new(5.0, 1.0, 0.0), 0.01));
        assert!(segment.contains_point(Vector::new(5.0, 0.1, 0.0), 0.5));
    }

    #[test]
    fn test_line_segment_extend() {
        let segment = LineSegment::new(Vector::new(1.0, 0.0, 0.0), Vector::new(9.0, 0.0, 0.0));
        let extended = segment.extend(1.0);
        assert_eq!(extended.start, Vector::new(0.0, 0.0, 0.0));
        assert_eq!(extended.end, Vector::new(10.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_segment_scale_from_center() {
        let segment = LineSegment::new(Vector::new(2.0, 0.0, 0.0), Vector::new(8.0, 0.0, 0.0));
        let scaled = segment.scale_from_center(2.0);
        assert_eq!(scaled.start, Vector::new(-1.0, 0.0, 0.0));
        assert_eq!(scaled.end, Vector::new(11.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_segment_display() {
        let segment = LineSegment::new(Vector::new(1.0, 2.0, 3.0), Vector::new(4.0, 5.0, 6.0));
        let display_str = format!("{}", segment);
        assert!(display_str.contains("Start: (1.000, 2.000, 3.000)"));
        assert!(display_str.contains("End: (4.000, 5.000, 6.000)"));
    }

    #[test]
    fn test_line_segment_json_serialization() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        let json = serde_json::to_string(&segment).unwrap();
        let deserialized: LineSegment = serde_json::from_str(&json).unwrap();
        assert_eq!(segment, deserialized);
    }

    #[test]
    fn test_line_segment_binary_serialization() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        let binary = segment.to_binary().unwrap();
        let deserialized = LineSegment::from_binary(&binary).unwrap();
        assert_eq!(segment, deserialized);
    }
}