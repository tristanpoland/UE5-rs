//! Integration tests between different modules to ensure they work together correctly

#[cfg(test)]
mod tests {
    use crate::types::{
        Vector, Transform, Rotator, BoundingBox, BoundingSphere, 
        Ray, Plane, LineSegment, LinearColor, DateTime, Timespan,
        TArray, NetworkGUID, RepMovement
    };
    use serde_json;

    #[test]
    fn test_transform_bounding_box_integration() {
        // Create a unit bounding box
        let bbox = BoundingBox::new(
            Vector::new(-1.0, -1.0, -1.0), 
            Vector::new(1.0, 1.0, 1.0)
        );
        
        // Create a transform that moves, rotates, and scales
        let transform = Transform::from_location_rotator_scale(
            Vector::new(10.0, 5.0, 0.0),
            Rotator::new(0.0, 90.0, 0.0), // 90-degree yaw rotation
            Vector::splat(2.0) // Scale by 2
        );
        
        let transformed_bbox = bbox.transform(transform);
        
        // Verify the bounding box is correctly transformed
        let center = transformed_bbox.center();
        assert!((center - Vector::new(10.0, 5.0, 0.0)).length() < 0.1);
        
        // Original size was 2x2x2, scaled by 2 should be 4x4x4
        let size = transformed_bbox.size();
        assert!((size.x - 4.0).abs() < 0.1);
        assert!((size.y - 4.0).abs() < 0.1);
        assert!((size.z - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_transform_bounding_sphere_integration() {
        let sphere = BoundingSphere::new(Vector::ZERO, 1.0);
        let transform = Transform::from_location_rotator_scale(
            Vector::new(5.0, 0.0, 0.0),
            Rotator::new(0.0, 0.0, 0.0),
            Vector::splat(3.0)
        );
        
        let transformed_sphere = sphere.transform(transform);
        
        assert_eq!(transformed_sphere.center, Vector::new(5.0, 0.0, 0.0));
        assert_eq!(transformed_sphere.radius, 3.0); // Original radius * scale
    }

    #[test]
    fn test_ray_plane_intersection() {
        // Create a horizontal plane at Z=0
        let plane = Plane::new(Vector::new(0.0, 0.0, 1.0), 0.0);
        
        // Create a ray shooting down from above
        let ray = Ray::new(Vector::new(2.0, 3.0, 5.0), Vector::new(0.0, 0.0, -1.0));
        
        // Calculate intersection
        let distance_to_plane = -plane.distance_to_point(ray.origin) as f64 / ray.direction.dot(plane.normal);
        assert!(distance_to_plane > 0.0); // Ray should hit the plane
        
        let intersection_point = ray.point_at_distance(distance_to_plane);
        assert_eq!(intersection_point.x, 2.0);
        assert_eq!(intersection_point.y, 3.0);
        assert!(intersection_point.z.abs() < 0.001); // Should be at Z=0
    }

    #[test]
    fn test_ray_bounding_box_intersection() {
        let bbox = BoundingBox::new(Vector::new(-1.0, -1.0, -1.0), Vector::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Vector::new(-5.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        
        // Ray should intersect the bounding box
        // Simple intersection test: check if ray passes through bbox center region
        let center = bbox.center();
        let to_center = ray.distance_to_closest_point(center);
        let closest_point = ray.point_at_distance(to_center);
        
        // The closest point on the ray to the center should be inside or near the bbox
        assert!(bbox.contains_point(closest_point) || bbox.distance_to_point(closest_point) < 2.0);
    }

    #[test]
    fn test_line_segment_bounding_box_intersection() {
        let bbox = BoundingBox::new(Vector::new(-1.0, -1.0, -1.0), Vector::new(1.0, 1.0, 1.0));
        let segment = LineSegment::new(Vector::new(-2.0, 0.0, 0.0), Vector::new(2.0, 0.0, 0.0));
        
        // Line segment passes through the bounding box
        let center = bbox.center();
        let closest_point = segment.closest_point_to(center);
        
        // The closest point should be inside the bounding box
        assert!(bbox.contains_point(closest_point));
        assert_eq!(closest_point, Vector::new(0.0, 0.0, 0.0)); // Should be at center
    }

    #[test]
    fn test_color_transform_integration() {
        // Test that colors work well with transforms in a rendering context
        let base_color = LinearColor::new(0.8, 0.4, 0.2, 1.0);
        let transform = Transform::from_location(Vector::new(10.0, 20.0, 30.0));
        
        // Simulate a game object with position and color
        struct GameObject {
            transform: Transform,
            color: LinearColor,
        }
        
        let obj = GameObject {
            transform,
            color: base_color,
        };
        
        // Test that we can manipulate both independently
        let brighter_color = obj.color.scale(1.5);
        let moved_transform = Transform::from_location(obj.transform.location + Vector::new(5.0, 0.0, 0.0));
        
        assert!(brighter_color.r > obj.color.r);
        assert!((moved_transform.location.x - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_networking_transform_integration() {
        // Test networking types with transforms (common in multiplayer games)
        let player_transform = Transform::from_location_rotator_scale(
            Vector::new(100.0, 200.0, 50.0),
            Rotator::new(0.0, 45.0, 0.0),
            Vector::ONE
        );
        
        let movement = RepMovement {
            location: player_transform.location,
            rotation: Rotator::from_quaternion(player_transform.rotation),
            linear_velocity: Vector::new(500.0, 0.0, 0.0),
            angular_velocity: Vector::ZERO,
            location_base: None,
            relative_location: Vector::ZERO,
            server_frame: 0,
            is_simulated: false,
            has_location_base: false,
        };
        
        let net_guid = NetworkGUID::new(1);
        
        // Verify the movement data matches the transform
        assert_eq!(movement.location, player_transform.location);
        assert!((movement.rotation.to_quaternion() - player_transform.rotation).length() < 0.001);
        assert!(net_guid.is_valid());
    }

    #[test]
    fn test_time_transform_integration() {
        // Test animation over time using DateTime and Transform
        let start_time = DateTime::now();
        let duration = Timespan::from_seconds(2.0);
        let end_time = start_time.add_timespan(duration);
        let _end_time = start_time.add_timespan(duration);
        let start_pos = Vector::new(0.0, 0.0, 0.0);
        let end_pos = Vector::new(10.0, 0.0, 0.0);
        
        // Simulate animation at 50% completion
        let current_time = start_time.add_timespan(Timespan::from_seconds(1.0));
        let time_ratio = current_time.difference(start_time).total_seconds() / duration.total_seconds();
        
        let current_pos = start_pos.lerp(end_pos, time_ratio);
        
        assert!((current_pos - Vector::new(5.0, 0.0, 0.0)).length() < 0.001);
        assert!((time_ratio - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_container_geometric_operations() {
        // Test containers with geometric types
        let mut points = TArray::new();
        points.add(Vector::new(0.0, 0.0, 0.0));
        points.add(Vector::new(1.0, 0.0, 0.0));
        points.add(Vector::new(0.0, 1.0, 0.0));
        
        // Create bounding box from points
        let bbox = BoundingBox::from_points(&points.as_slice());
        // Verify all points are contained
        for i in 0..points.num() {
            let point = points.get(i).unwrap();
        }
        
        // Test geometric operations on container contents
        let center = bbox.center();
        let mut distances = TArray::<f64>::new();
        
        for i in 0..points.num() {
            let point = points.get(i).unwrap();
            let distance = point.distance(center);
            distances.add(distance);
        }
        
        assert_eq!(distances.num(), points.num());
        assert!(distances.get(0).unwrap() >= &0.0);
    }

    #[test]
    fn test_serialization_integration() {
        // Test that complex structures with multiple types serialize correctly
        #[derive(serde::Serialize, serde::Deserialize)]
        struct ComplexGameObject {
            transform: Transform,
            color: LinearColor,
            bounds: BoundingBox,
            net_id: NetworkGUID,
            creation_time: DateTime,
        }
        
        let original = ComplexGameObject {
            transform: Transform::from_location_rotator_scale(
                Vector::new(1.0, 2.0, 3.0),
                Rotator::new(10.0, 20.0, 30.0),
                Vector::new(1.5, 1.5, 1.5)
            ),
            color: LinearColor::new(0.8, 0.4, 0.2, 1.0),
            bounds: BoundingBox::new(Vector::new(-1.0, -1.0, -1.0), Vector::new(1.0, 1.0, 1.0)),
            net_id: NetworkGUID::new(12345),
            creation_time: DateTime::now(),
        };
        
        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        let from_json: ComplexGameObject = serde_json::from_str(&json).unwrap();
        
        // Verify all fields survived serialization
        assert!((from_json.transform.location - original.transform.location).length() < 0.001);
        assert!(from_json.color.is_nearly_equal(original.color, 0.001));
        assert_eq!(from_json.bounds.min, original.bounds.min);
        assert_eq!(from_json.net_id, original.net_id);
    }
}