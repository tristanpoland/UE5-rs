//! Examples demonstrating the UE Types library features
//! 
//! This file shows how to use all the major features of the library including:
//! - Display formatting
//! - JSON serialization/deserialization
//! - Binary serialization/deserialization
//! - Type conversions and mathematical operations

use ue_types::*;

fn main() {
    println!("=== UE Types Library Examples ===\n");

    // Example 1: Basic type creation and display
    basic_types_example();
    
    // Example 2: JSON serialization
    json_serialization_example();
    
    // Example 3: Binary serialization
    binary_serialization_example();
    
    // Example 4: Mathematical operations
    math_operations_example();
    
    // Example 5: Game object example
    game_object_example();
}

fn basic_types_example() {
    println!("1. Basic Types and Display Formatting");
    println!("=====================================");
    
    // Vector types
    let position = Vector::new(10.5, 20.0, 30.25);
    let velocity = Vector::new(1.0, 0.0, 0.5);
    println!("Position: {}", position);
    println!("Velocity: {}", velocity);
    
    // Rotator
    let rotation = Rotator::new(45.0, 90.0, -15.0);
    println!("Rotation: {}", rotation);
    
    // Transform
    let transform = Transform::new(position, rotation.to_quaternion(), Vector::splat(2.0));
    println!("Transform: {}", transform);
    
    // Colors
    let linear_color = LinearColor::new(0.8, 0.4, 0.2, 1.0);
    let color = Color::from_hex(0xFF8040);
    println!("Linear Color: {}", linear_color);
    println!("Color: {}", color);
    
    // Bounding volumes
    let bbox = BoundingBox::new(Vector::new(-5.0, -5.0, -5.0), Vector::new(5.0, 5.0, 5.0));
    let sphere = BoundingSphere::new(Vector::ZERO, 7.0);
    println!("Bounding Box: {}", bbox);
    println!("Bounding Sphere: {}", sphere);
    
    println!();
}

fn json_serialization_example() {
    println!("2. JSON Serialization");
    println!("====================");
    
    // Create a complex transform
    let original_transform = Transform::new(
        Vector::new(100.0, 200.0, 300.0),
        Rotator::new(30.0, 45.0, 60.0).to_quaternion(),
        Vector::new(1.5, 2.0, 0.5),
    );
    
    // Serialize to JSON
    match serde_json::to_string_pretty(&original_transform) {
        Ok(json) => {
            println!("Transform as JSON:");
            println!("{}", json);
            
            // Deserialize back
            match serde_json::from_str::<Transform>(&json) {
                Ok(deserialized) => {
                    println!("Deserialized transform: {}", deserialized);
                    println!("Equal: {}", original_transform.is_nearly_equal(deserialized, 0.001));
                }
                Err(e) => println!("Failed to deserialize: {}", e),
            }
        }
        Err(e) => println!("Failed to serialize: {}", e),
    }
    
    println!();
}

fn binary_serialization_example() {
    println!("3. Binary Serialization");
    println!("======================");
    
    // Create test data
    let color = LinearColor::new(0.9, 0.1, 0.5, 0.8);
    let bbox = BoundingBox::from_center_and_extent(
        Vector::new(50.0, 100.0, 150.0),
        Vector::new(25.0, 25.0, 25.0),
    );
    
    // Binary serialization
    match color.to_binary() {
        Ok(binary_data) => {
            println!("LinearColor binary size: {} bytes", binary_data.len());
            
            match LinearColor::from_binary(&binary_data) {
                Ok(deserialized) => {
                    println!("Original: {}", color);
                    println!("Deserialized: {}", deserialized);
                    println!("Equal: {}", color.is_nearly_equal(deserialized, 0.001));
                }
                Err(e) => println!("Failed to deserialize: {}", e),
            }
        }
        Err(e) => println!("Failed to serialize: {}", e),
    }
    
    match bbox.to_binary() {
        Ok(binary_data) => {
            println!("BoundingBox binary size: {} bytes", binary_data.len());
            
            match BoundingBox::from_binary(&binary_data) {
                Ok(deserialized) => {
                    println!("Original: {}", bbox);
                    println!("Deserialized: {}", deserialized);
                    println!("Equal: {}", bbox == deserialized);
                }
                Err(e) => println!("Failed to deserialize: {}", e),
            }
        }
        Err(e) => println!("Failed to serialize: {}", e),
    }
    
    println!();
}

fn math_operations_example() {
    println!("4. Mathematical Operations");
    println!("=========================");
    
    // Vector operations
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(4.0, 5.0, 6.0);
    let dot_product = v1.dot(v2);
    let cross_product = v1.cross(v2);
    
    println!("V1: {}", v1);
    println!("V2: {}", v2);
    println!("Dot Product: {:.2}", dot_product);
    println!("Cross Product: {}", cross_product);
    
    // Rotator operations
    let rot1 = Rotator::new(30.0, 45.0, 0.0);
    let rot2 = Rotator::new(15.0, 25.0, 10.0);
    let combined_rot = rot1.add(rot2);
    
    println!("Rotation 1: {}", rot1);
    println!("Rotation 2: {}", rot2);
    println!("Combined: {}", combined_rot);
    
    // Transform operations
    let transform1 = Transform::from_location(Vector::new(10.0, 0.0, 0.0));
    let transform2 = Transform::from_rotation(Rotator::from_yaw(90.0).to_quaternion());
    let combined_transform = transform1.combine(transform2);
    
    println!("Transform 1: {}", transform1);
    println!("Transform 2: {}", transform2);
    println!("Combined: {}", combined_transform);
    
    // Test point transformation
    let point = Vector::new(5.0, 0.0, 0.0);
    let transformed_point = combined_transform.transform_point(point);
    println!("Point: {} -> Transformed: {}", point, transformed_point);
    
    println!();
}

fn game_object_example() {
    println!("5. Game Object Example");
    println!("=====================");
    
    // Simulate a game object with transform, color, and bounding volume
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct GameObject {
        name: String,
        transform: Transform,
        color: LinearColor,
        bounding_box: BoundingBox,
        health: f32,
        is_active: bool,
    }
    
    impl BinarySerializable for GameObject {}
    
    impl std::fmt::Display for GameObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "GameObject '{}' (Health: {:.1}, Active: {})\n  Transform: {}\n  Color: {}\n  Bounds: {}",
                self.name, self.health, self.is_active, self.transform, self.color, self.bounding_box
            )
        }
    }
    
    let player = GameObject {
        name: "Player".to_string(),
        transform: Transform::new(
            Vector::new(0.0, 0.0, 100.0),
            Rotator::from_yaw(45.0).to_quaternion(),
            Vector::ONE,
        ),
        color: LinearColor::BLUE,
        bounding_box: BoundingBox::from_center_and_extent(
            Vector::ZERO,
            Vector::new(1.0, 1.0, 2.0),
        ),
        health: 100.0,
        is_active: true,
    };
    
    println!("{}", player);
    
    // Serialize the entire game object
    match serde_json::to_string_pretty(&player) {
        Ok(json) => {
            println!("\nPlayer as JSON:");
            println!("{}", json);
        }
        Err(e) => println!("Failed to serialize player: {}", e),
    }
    
    // Binary serialization
    match player.to_binary() {
        Ok(binary_data) => {
            println!("\nPlayer binary size: {} bytes", binary_data.len());
            
            match GameObject::from_binary(&binary_data) {
                Ok(deserialized) => {
                    println!("Deserialized player matches: {}", 
                        player.name == deserialized.name && 
                        player.transform.is_nearly_equal(deserialized.transform, 0.001)
                    );
                }
                Err(e) => println!("Failed to deserialize player: {}", e),
            }
        }
        Err(e) => println!("Failed to serialize player: {}", e),
    }
    
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_run_without_panic() {
        // This test ensures all examples can run without panicking
        // In a real scenario, you'd capture the output and verify it
        basic_types_example();
        json_serialization_example();
        binary_serialization_example();
        math_operations_example();
        game_object_example();
    }

    #[test]
    fn test_game_object_serialization_roundtrip() {
        #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
        struct TestGameObject {
            transform: Transform,
            color: LinearColor,
        }
        
        impl BinarySerializable for TestGameObject {}
        
        let original = TestGameObject {
            transform: Transform::from_location(Vector::new(1.0, 2.0, 3.0)),
            color: LinearColor::RED,
        };
        
        // Test JSON roundtrip
        let json = serde_json::to_string(&original).unwrap();
        let from_json: TestGameObject = serde_json::from_str(&json).unwrap();
        assert_eq!(original, from_json);
        
        // Test binary roundtrip
        let binary = original.to_binary().unwrap();
        let from_binary = TestGameObject::from_binary(&binary).unwrap();
        assert_eq!(original, from_binary);
    }
}