//! Additional math types and utilities for UE5 compatibility

use crate::vector::*;
use crate::BinarySerializable;
use glam::{Vec2, Vec3, Vec4, Mat3, Mat4, Quat};
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
}

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
        Vector::new(self.x as f32, self.y as f32, self.z as f32)
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
}

/// 2D Plane representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane2D {
    /// Plane normal (should be normalized)
    pub normal: Vector2D,
    /// Distance from origin along normal
    pub distance: f32,
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
    pub fn new(normal: Vector2D, distance: f32) -> Self {
        Self { normal, distance }
    }

    /// Create a plane from a point and normal
    pub fn from_point_normal(point: Vector2D, normal: Vector2D) -> Self {
        let normalized = normal.normalize();
        let distance = point.dot(normalized);
        Self::new(normalized, distance)
    }

    /// Get the signed distance from a point to the plane
    pub fn distance_to_point(self, point: Vector2D) -> f32 {
        self.normal.dot(point) - self.distance
    }

    /// Check which side of the plane a point is on
    pub fn point_side(self, point: Vector2D) -> f32 {
        self.distance_to_point(point)
    }
}

/// 3D Plane representation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Plane {
    /// Plane normal (should be normalized)
    pub normal: Vector,
    /// Distance from origin along normal
    pub distance: f32,
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
    pub fn new(normal: Vector, distance: f32) -> Self {
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
    pub fn distance_to_point(self, point: Vector) -> f32 {
        self.normal.dot(point) - self.distance
    }

    /// Check which side of the plane a point is on
    pub fn point_side(self, point: Vector) -> f32 {
        self.distance_to_point(point)
    }

    /// Project a point onto the plane
    pub fn project_point(self, point: Vector) -> Vector {
        point - self.normal * self.distance_to_point(point)
    }
}

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

    /// Get a point along the ray at the given distance
    pub fn point_at_distance(self, distance: f32) -> Vector {
        self.origin + self.direction * distance
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
}

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

    /// Get the direction vector of the line segment
    pub fn direction(self) -> Vector {
        (self.end - self.start).normalize()
    }

    /// Get the length of the line segment
    pub fn length(self) -> f32 {
        (self.end - self.start).length()
    }

    /// Get a point along the line segment (t=0 is start, t=1 is end)
    pub fn lerp(self, t: f32) -> Vector {
        self.start.lerp(self.end, t.clamp(0.0, 1.0))
    }

    /// Get the closest point on the line segment to a given point
    pub fn closest_point_to(self, point: Vector) -> Vector {
        let segment_vec = self.end - self.start;
        let to_point = point - self.start;
        let t = to_point.dot(segment_vec) / segment_vec.dot(segment_vec);
        self.lerp(t)
    }

    /// Get the distance from the line segment to a point
    pub fn distance_to_point(self, point: Vector) -> f32 {
        let closest = self.closest_point_to(point);
        (point - closest).length()
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
    fn test_int_vector_operations() {
        let iv1 = IntVector::new(1, 2, 3);
        let iv2 = IntVector::new(4, 5, 6);
        
        assert_eq!(iv1.add(iv2), IntVector::new(5, 7, 9));
        assert_eq!(iv2.sub(iv1), IntVector::new(3, 3, 3));
        assert_eq!(iv1.dot(iv2), 32); // 1*4 + 2*5 + 3*6 = 32
    }

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
    fn test_ray_operations() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let point = ray.point_at_distance(5.0);
        assert_eq!(point, Vector::new(5.0, 0.0, 0.0));
        
        let test_point = Vector::new(3.0, 4.0, 0.0);
        let closest = ray.closest_point_to(test_point);
        assert_eq!(closest, Vector::new(3.0, 0.0, 0.0));
    }

    #[test]
    fn test_line_segment_operations() {
        let segment = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
        assert_eq!(segment.length(), 10.0);
        
        let midpoint = segment.lerp(0.5);
        assert_eq!(midpoint, Vector::new(5.0, 0.0, 0.0));
        
        let test_point = Vector::new(3.0, 4.0, 0.0);
        let closest = segment.closest_point_to(test_point);
        assert_eq!(closest, Vector::new(3.0, 0.0, 0.0));
        assert_eq!(segment.distance_to_point(test_point), 4.0);
    }

    #[test]
    fn test_json_serialization() {
        let iv = IntVector::new(1, 2, 3);
        let json = serde_json::to_string(&iv).unwrap();
        let deserialized: IntVector = serde_json::from_str(&json).unwrap();
        assert_eq!(iv, deserialized);
        
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 5.0);
        let json = serde_json::to_string(&plane).unwrap();
        let deserialized: Plane = serde_json::from_str(&json).unwrap();
        assert_eq!(plane, deserialized);
    }

    #[test]
    fn test_binary_serialization() {
        let ray = Ray::new(Vector::ZERO, Vector::new(1.0, 0.0, 0.0));
        let binary = ray.to_binary().unwrap();
        let deserialized = Ray::from_binary(&binary).unwrap();
        assert_eq!(ray, deserialized);
    }
}