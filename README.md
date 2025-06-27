# UE Types Library for Rust

A Rust library providing common Unreal Engine data types optimized for game servers. Built on top of the high-performance `glam` math library with full serialization support.

## Features

- **Complete UE Type Coverage**: Vector, Rotator, Transform, Colors, Bounding Volumes
- **Multiple Serialization Formats**: JSON (serde) and Binary (bincode)
- **Display Formatting**: Human-readable output for all types
- **High Performance**: Built on `glam` for optimal math operations
- **Type Safety**: Leverages Rust's type system for safe game development
- **UE Compatibility**: Familiar API for Unreal Engine developers

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ue-types = "0.1.0"
```

Basic usage:

```rust
use ue_types::*;

// Create a transform
let transform = Transform::new(
    Vector::new(10.0, 20.0, 30.0),                // Location
    Rotator::new(0.0, 45.0, 0.0).to_quaternion(), // Rotation (45° yaw)
    Vector::splat(2.0)                            // Uniform scale of 2.0
);

// Display the transform
println!("Transform: {}", transform);

// Serialize to JSON
let json = serde_json::to_string(&transform)?;

// Serialize to binary
let binary_data = transform.to_binary()?;
```

## Type Overview

### Vector Types

- **`Vector`** (Vec3) - 3D position, velocity, direction
- **`Vector2D`** (Vec2) - 2D coordinates, UI positions
- **`Vector4`** (Vec4) - 4D vectors, homogeneous coordinates
- **`Quaternion`** (Quat) - Efficient 3D rotations
- **`Matrix3`**, **`Matrix4`** - Transformation matrices

```rust
let position = Vector::new(10.0, 20.0, 30.0);
let direction = Vector::new(1.0, 0.0, 0.0);

// UE-style operations
println!("Size: {:.2}", position.size());
println!("Is normalized: {}", direction.is_normalized());
```

### Rotator

UE-style Euler angle rotation in degrees (Pitch, Yaw, Roll):

```rust
let rotation = Rotator::new(30.0, 45.0, 0.0);  // 30° pitch, 45° yaw
let forward = rotation.get_forward_vector();
let quaternion = rotation.to_quaternion();

println!("Rotation: {}", rotation);  // "P=30.00° Y=45.00° R=0.00°"
```

### Transform

Complete 3D transformation with location, rotation, and scale:

```rust
let transform = Transform::from_location_rotator_scale(
    Vector::new(100.0, 200.0, 300.0),
    Rotator::new(0.0, 90.0, 0.0),
    Vector::splat(1.5)
);

// Transform points and vectors
let point = Vector::new(5.0, 0.0, 0.0);
let transformed_point = transform.transform_point(point);

// Combine transforms
let combined = transform1.combine(transform2);
```

### Colors

#### LinearColor (HDR, 0.0-1.0 range)

```rust
let color = LinearColor::new(0.8, 0.4, 0.2, 1.0);
let from_hsv = LinearColor::from_hsv(120.0, 1.0, 0.5);  // Green
let lerped = LinearColor::RED.lerp(LinearColor::BLUE, 0.5);

println!("{}", color);  // "LinearColor(R=0.800, G=0.400, B=0.200, A=1.000)"
```

#### Color (sRGB, 0-255 range)

```rust
let color = Color::from_hex(0xFF8040);
let linear = color.to_linear();  // Convert to linear space
let back = Color::from_linear(linear);  // Convert back to sRGB

println!("{}", color);  // "Color(R=255, G=128, B=64, A=255) [#FF8040FF]"
```

### Bounding Volumes

#### BoundingBox (AABB)

```rust
let bbox = BoundingBox::new(
    Vector::new(-10.0, -10.0, -10.0),
    Vector::new(10.0, 10.0, 10.0)
);

let center = bbox.center();
let volume = bbox.volume();
let contains_point = bbox.contains_point(Vector::ZERO);

// Transform the bounding box
let transformed_bbox = bbox.transform(some_transform);
```

#### BoundingSphere

```rust
let sphere = BoundingSphere::new(Vector::ZERO, 5.0);
let intersects = sphere.intersects_box(bbox);
let distance = sphere.distance_to_point(some_point);
```

## Serialization

All types support multiple serialization formats:

### JSON Serialization (serde)

```rust
use serde_json;

let transform = Transform::from_location(Vector::new(1.0, 2.0, 3.0));

// Serialize
let json = serde_json::to_string(&transform)?;
let pretty_json = serde_json::to_string_pretty(&transform)?;

// Deserialize
let restored: Transform = serde_json::from_str(&json)?;
```

### Binary Serialization (bincode)

```rust
// All types implement BinarySerializable
let color = LinearColor::RED;

// Serialize to binary
let binary_data = color.to_binary()?;
println!("Binary size: {} bytes", binary_data.len());

// Deserialize from binary
let restored = LinearColor::from_binary(&binary_data)?;
```

## Display Formatting

All types implement `Display` for human-readable output:

```rust
let transform = Transform::new(
    Vector::new(10.0, 20.0, 30.0),
    Rotator::new(45.0, 90.0, 0.0).to_quaternion(),
    Vector::splat(2.0)
);

println!("{}", transform);
// Output: "Location: (10.00, 20.00, 30.00), Rotation: P=45.00° Y=90.00° R=0.00°, Scale: (2.00, 2.00, 2.00)"
```

## Game Server Example

```rust
use ue_types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: u32,
    name: String,
    transform: Transform,
    health: f32,
    color: LinearColor,
    bounding_box: BoundingBox,
}

impl BinarySerializable for Player {}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {} '{}' at {} (HP: {:.1})", 
               self.id, self.name, self.transform.location, self.health)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player = Player {
        id: 1,
        name: "Alice".to_string(),
        transform: Transform::from_location(Vector::new(100.0, 200.0, 0.0)),
        health: 95.5,
        color: LinearColor::BLUE,
        bounding_box: BoundingBox::from_center_and_extent(
            Vector::ZERO, 
            Vector::new(0.5, 0.5, 1.0)
        ),
    };

    // Display
    println!("{}", player);

    // JSON serialization for web APIs
    let json = serde_json::to_string(&player)?;
    println!("JSON: {}", json);

    // Binary serialization for network protocols
    let binary = player.to_binary()?;
    println!("Binary size: {} bytes", binary.len());

    // Deserialize
    let restored_player = Player::from_binary(&binary)?;
    println!("Restored: {}", restored_player);

    Ok(())
}
```

## Performance Considerations

- **Built on glam**: Leverages SIMD optimizations when available
- **Zero-cost abstractions**: Wrapper types compile to underlying glam types
- **Efficient serialization**: Binary format is compact and fast
- **Memory layout**: All types are `#[repr(C)]` compatible where applicable

## UE Compatibility Notes

- **Coordinate system**: Follows UE's left-handed coordinate system (X=Forward, Y=Right, Z=Up)
- **Rotation order**: Uses UE's YXZ Euler rotation order for Rotators
- **Units**: Distances in centimeters (UE default), rotations in degrees
- **Color spaces**: Proper sRGB ↔ Linear conversion with gamma correction

## Feature Flags

```toml
[dependencies]
ue-types = { version = "0.1.0", features = ["serde"] }
```

Available features:
- `serde` (default) - JSON serialization support
- `binary` (default) - Binary serialization support

## License

MIT License - see LICENSE file for details.