# UE Types Library for Rust

A comprehensive Rust library providing Unreal Engine 5 data types optimized for game servers and applications. Built on top of the high-performance `glam` math library with full serialization support and UE5-compatible APIs.

## Features

- **Complete UE5 Type Coverage**: All essential UE5 data types with familiar APIs
- **Multiple Serialization Formats**: JSON (serde) and Binary (bincode) 
- **Display Formatting**: Human-readable output for debugging and logging
- **High Performance**: Built on `glam` with SIMD optimizations
- **Type Safety**: Leverages Rust's type system for safe game development
- **UE5 Compatibility**: Familiar method names and behavior for UE developers
- **Container Types**: UE5-style collections (TArray, TMap, TSet)
- **Utility Types**: DateTime, GUID, Name, Text, Version and more

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ue-types = "0.1.1"
```

## Quick Start

```rust
use ue_types::*;

// Create a transform with location, rotation, and scale
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

## Core Math Types

### Vector Types

UE5-compatible 3D vector with familiar methods:

```rust
use ue_types::*;

// Create vectors
let position = Vector::new(10.0, 20.0, 30.0);
let direction = Vector::FORWARD;  // (1, 0, 0)
let up = Vector::UP;              // (0, 0, 1)

// UE5-style operations
println!("Size: {:.2}", position.size());           // Magnitude
println!("Size squared: {:.2}", position.size_squared());
println!("Is normalized: {}", direction.is_normalized());
println!("Is zero: {}", Vector::ZERO.is_zero());

// Math operations
let normalized = position.get_normalized();
let dot_product = position.dot(direction);
let cross_product = position.cross(direction);
let distance = position.distance_to(Vector::ZERO);

// Lerp and interpolation
let lerped = Vector::ZERO.lerp(position, 0.5);
```

**Constants Available:**
- `Vector::ZERO` - (0, 0, 0)
- `Vector::ONE` - (1, 1, 1) 
- `Vector::FORWARD` - (1, 0, 0)
- `Vector::RIGHT` - (0, 1, 0)
- `Vector::UP` - (0, 0, 1)

### Vector2D

2D vector for UI coordinates and 2D math:

```rust
let ui_pos = Vector2D::new(100.0, 200.0);
let screen_center = Vector2D::new(960.0, 540.0);

let distance = ui_pos.distance_to(screen_center);
let normalized = ui_pos.get_normalized();
```

### Rotator

UE5-style Euler angle rotation in degrees (Pitch, Yaw, Roll):

```rust
// Create rotations
let rotation = Rotator::new(30.0, 45.0, 0.0);  // 30° pitch, 45° yaw
let yaw_only = Rotator::from_yaw(90.0);

// Get direction vectors
let forward = rotation.get_forward_vector();
let right = rotation.get_right_vector();
let up = rotation.get_up_vector();

// Convert to/from quaternion
let quaternion = rotation.to_quaternion();
let back_to_rotator = Rotator::from_quaternion(quaternion);

// Rotation math
let combined = rotation + Rotator::new(0.0, 45.0, 0.0);
let normalized = rotation.get_normalized();  // Clamps to [-180, 180]

println!("Rotation: {}", rotation);  // "P=30.00° Y=45.00° R=0.00°"
```

### Transform

Complete 3D transformation with location, rotation, and scale:

```rust
// Create transforms
let transform = Transform::from_location_rotator_scale(
    Vector::new(100.0, 200.0, 300.0),
    Rotator::new(0.0, 90.0, 0.0),
    Vector::splat(1.5)
);

let simple = Transform::from_location(Vector::new(10.0, 0.0, 0.0));
let identity = Transform::IDENTITY;

// Transform points and vectors
let point = Vector::new(5.0, 0.0, 0.0);
let transformed_point = transform.transform_point(point);
let transformed_vector = transform.transform_vector(point);

// Combine transforms (parent * child)
let combined = parent_transform.combine(child_transform);

// Inverse transforms
let inverse = transform.inverse();
let inv_point = transform.inverse_transform_point(transformed_point);

// Convert to/from matrix
let matrix = transform.to_matrix();
let from_matrix = Transform::from_matrix(matrix);
```

## Color Types

### LinearColor (HDR, 0.0-1.0 range)

High dynamic range color for rendering and lighting:

```rust
// Create colors
let color = LinearColor::new(0.8, 0.4, 0.2, 1.0);
let red = LinearColor::RED;
let transparent = LinearColor::TRANSPARENT;

// Color operations
let brighter = color * 2.0;  // HDR brightening
let lerped = LinearColor::RED.lerp(LinearColor::BLUE, 0.5);
let desaturated = color.desaturate(0.5);

// HSV color space
let from_hsv = LinearColor::from_hsv(120.0, 1.0, 0.5);  // Green
let (h, s, v) = color.to_hsv();

// sRGB conversion
let srgb_color = color.to_color();  // Convert to Color (0-255)

println!("{}", color);  // "LinearColor(R=0.800, G=0.400, B=0.200, A=1.000)"
```

**Predefined Colors:**
- `LinearColor::WHITE`, `BLACK`, `RED`, `GREEN`, `BLUE`
- `LinearColor::YELLOW`, `CYAN`, `MAGENTA`
- `LinearColor::TRANSPARENT`

### Color (sRGB, 0-255 range)

Standard 8-bit per channel color:

```rust
// Create colors
let color = Color::new(255, 128, 64, 255);
let from_hex = Color::from_hex(0xFF8040);
let from_rgb = Color::from_rgb(255, 128, 64);

// Convert to linear space for calculations
let linear = color.to_linear();
let processed = linear * 1.2;  // Brighten in linear space
let back_to_srgb = processed.to_color();

// Color mixing
let mixed = color.lerp(Color::BLUE, 0.3);

println!("{}", color);  // "Color(R=255, G=128, B=64, A=255) [#FF8040FF]"
```

## Bounding Volumes

### BoundingBox (Axis-Aligned Bounding Box)

```rust
// Create bounding boxes
let bbox = BoundingBox::new(
    Vector::new(-10.0, -10.0, -10.0),  // Min
    Vector::new(10.0, 10.0, 10.0)      // Max
);

let from_center = BoundingBox::from_center_and_extent(
    Vector::ZERO,                       // Center
    Vector::new(5.0, 5.0, 5.0)        // Half-extents
);

// Properties
let center = bbox.center();
let size = bbox.size();
let volume = bbox.volume();
let is_valid = bbox.is_valid();

// Spatial queries
let contains_point = bbox.contains_point(Vector::new(5.0, 0.0, 0.0));
let contains_box = bbox.contains(other_bbox);
let intersects = bbox.intersects(other_bbox);

// Expand and modify
let expanded = bbox.expand_by_point(Vector::new(15.0, 0.0, 0.0));
let grown = bbox.expand_by(5.0);  // Grow by 5 units in all directions

// Transform
let transformed = bbox.transform(some_transform);
```

### BoundingSphere

```rust
// Create spheres
let sphere = BoundingSphere::new(Vector::ZERO, 5.0);
let from_points = BoundingSphere::from_points(&[
    Vector::new(1.0, 0.0, 0.0),
    Vector::new(-1.0, 0.0, 0.0),
    Vector::new(0.0, 1.0, 0.0),
]);

// Properties
let volume = sphere.volume();
let surface_area = sphere.surface_area();

// Spatial queries
let contains_point = sphere.contains_point(Vector::new(3.0, 0.0, 0.0));
let distance = sphere.distance_to_point(Vector::new(10.0, 0.0, 0.0));
let intersects_box = sphere.intersects_box(bbox);
let intersects_sphere = sphere.intersects(other_sphere);

// Transform
let transformed = sphere.transform(some_transform);
```

## Container Types

UE5-style collections with familiar APIs:

### TArray<T> (Dynamic Array)

```rust
use ue_types::*;

// Create arrays
let mut arr = TArray::new();
let from_vec = TArray::from_vec(vec![1, 2, 3, 4, 5]);

// UE5-style methods
arr.add(42);                    // Add element, returns index
let count = arr.num();          // Get count (returns i32)
let item = arr.get(0);          // Get by index (returns Option)
let last = arr.last();          // Get last element

// Find and search
let index = arr.find(&42);      // Returns i32 (-1 if not found)
let contains = arr.contains(&42); // Returns bool

// Remove operations
arr.remove_at(1);               // Remove by index
arr.remove(&42);                // Remove by value
arr.empty();                    // Clear all elements

// Iteration
for item in &arr {
    println!("Item: {}", item);
}

println!("{}", arr);  // "TArray[1, 2, 3]"
```

### TMap<K, V> (Hash Map)

```rust
// Create maps
let mut map = TMap::new();
let with_capacity = TMap::with_capacity(100);

// UE5-style methods
map.add("player1", 100);        // Add/update entry
let count = map.num();          // Get count
let value = map.find(&"player1"); // Find value (returns Option)
let contains = map.contains(&"player1"); // Check key exists

// Modify
map.remove(&"player1");         // Remove by key
let old_value = map.add("player1", 200); // Returns old value if exists

// Find or add pattern
let entry = map.find_or_add("new_player", 0); // Get or insert default

// Iteration
for (key, value) in &map {
    println!("{}: {}", key, value);
}

println!("{}", map);  // "TMap{player1: 100, player2: 200}"
```

### TSet<T> (Hash Set)

```rust
// Create sets
let mut set = TSet::new();
let from_iter = TSet::from_iter(vec![1, 2, 3, 2, 1]); // Deduplicates

// UE5-style methods
let added = set.add(42);        // Returns true if newly added
let count = set.num();          // Get count
let contains = set.contains(&42); // Check membership

// Set operations
let removed = set.remove(&42);   // Remove element
set.empty();                    // Clear all elements

// Set theory operations
let union_iter = set1.union(&set2);
let intersection_iter = set1.intersection(&set2);
let difference_iter = set1.difference(&set2);

println!("{}", set);  // "TSet{1, 2, 3}"
```

## Utility Types

### DateTime

UE5-style DateTime for timestamps and scheduling:

```rust
// Create DateTime
let now = DateTime::now();
let from_timestamp = DateTime::from_unix_timestamp(1640995200); // 2022-01-01
let from_millis = DateTime::from_unix_timestamp_millis(1640995200000);

// Convert
let timestamp = now.to_unix_timestamp();
let millis = now.to_unix_timestamp_millis();
let system_time = now.to_system_time();

// Date arithmetic
let later = now.add_timespan(Timespan::from_hours(2.0));
let earlier = now.sub_timespan(Timespan::from_minutes(30.0));
let difference = later.difference(earlier); // Returns Timespan

// Date parts
let date_only = now.date();  // Midnight of the same day
let time_of_day = now.time_of_day(); // Timespan since midnight

println!("{}", now);  // "DateTime(2024-01-15 14:30:45 UTC)"
```

### Timespan

Duration and time interval representation:

```rust
// Create Timespan
let duration = Timespan::from_hours(2.5);
let short_time = Timespan::from_milliseconds(500.0);
let from_seconds = Timespan::from_seconds(30.0);

// Get values
let total_hours = duration.total_hours();
let total_minutes = duration.total_minutes();
let total_seconds = duration.total_seconds();

// Arithmetic
let sum = timespan1.add(timespan2);
let difference = timespan1.subtract(timespan2);
let absolute = negative_timespan.abs();

// Properties
let is_positive = duration.is_positive();
let is_negative = duration.is_negative();
let is_zero = Timespan::ZERO.is_zero();

println!("{}", duration);  // "Timespan(2h 30m 0.0s)"
```

### Guid

UE5-style GUID for unique identifiers:

```rust
// Create GUIDs
let new_guid = Guid::new_guid();  // Generate new GUID
let specific = Guid::new(0x12345678, 0x9ABCDEF0, 0x12345678, 0x9ABCDEF0);
let invalid = Guid::INVALID;      // All zeros

// Parse and format
let parsed = Guid::parse("12345678-9ABCDEF0-12345678-9ABCDEF0")?;
let string_repr = guid.to_string();

// Properties
let is_valid = guid.is_valid();   // Non-zero check

// Binary conversion
let bytes = guid.to_bytes();      // [u8; 16]
let from_bytes = Guid::from_bytes(bytes);

println!("{}", guid);  // "Guid(12345678-9ABCDEF0-12345678-9ABCDEF0)"
```

### Name

Fast string comparisons using cached hashes:

```rust
// Create Names
let name1 = Name::new("PlayerCharacter");
let name2 = Name::from("EnemyAI");
let from_string = Name::new(String::from("ItemPickup"));

// Fast comparisons (uses cached hash)
let are_equal = name1 == name2;
let hash_value = name1.hash();

// String operations
let as_string = name1.as_str();
let length = name1.len();
let is_empty = name1.is_empty();

println!("{}", name1);  // "Name("PlayerCharacter")"
```

### Text

Localized text with namespace support:

```rust
// Simple text
let simple = Text::new("Hello World");
let from_str = Text::from("Welcome");

// Localized text
let localized = Text::from_key(
    "UI",           // Namespace
    "WelcomeMsg",   // Key
    "Welcome!"      // Source/default text
);

// Properties
let display = text.as_str();
let is_localizable = localized.is_localizable(); // Has namespace/key
let is_empty = text.is_empty();

// Update display string (for localization system)
let mut text = localized;
text.set_display_string("¡Bienvenido!");

println!("{}", text);  // "Text("Welcome!")"
```

### Version

Version information with semantic versioning support:

```rust
// Create versions
let version = Version::new(1, 2, 3, 4);  // Major.Minor.Patch.Build
let semver = Version::from_semver(1, 2, 3); // No build number
let parsed = Version::parse("1.2.3.4")?;

// Convert
let version_string = version.to_string(); // "1.2.3.4" or "1.2.3"

// Compatibility
let compatible = version1.is_compatible_with(version2); // Same major version

println!("{}", version);  // "Version(1.2.3.4)"
```

## Math Utility Types

### IntVector and IntVector2

Integer vectors for discrete coordinates:

```rust
// 2D integer vector
let grid_pos = IntVector2::new(10, 20);
let moved = grid_pos + IntVector2::new(1, 0);

// 3D integer vector  
let voxel_pos = IntVector::new(5, 10, 15);
let manhattan = voxel_pos.manhattan_distance(IntVector::ZERO);

// Convert to/from float vectors
let float_vec = grid_pos.to_vector2d();
let back_to_int = IntVector2::from_vector2d(float_vec);
```

### Plane

Mathematical plane representation:

```rust
// Create planes
let plane = Plane::new(Vector::UP, 0.0);  // XY plane at Z=0
let from_points = Plane::from_three_points(p1, p2, p3);

// Plane operations
let distance = plane.distance_to_point(some_point);
let projected = plane.project_point(some_point);
let is_front = plane.is_point_in_front(some_point);
```

### Ray and LineSegment

Ray casting and line segment math:

```rust
// Ray (origin + direction)
let ray = Ray::new(Vector::ZERO, Vector::FORWARD);
let point_on_ray = ray.point_at(5.0);  // 5 units along ray

// Line segment (start + end points)
let line = LineSegment::new(Vector::ZERO, Vector::new(10.0, 0.0, 0.0));
let midpoint = line.lerp(0.5);  // Point halfway along line
let closest = line.closest_point_to(some_point);
```

## Networking Types

Specialized types for networked games:

### NetworkGuid

Network-specific GUID with additional metadata:

```rust
let net_guid = NetworkGuid::new();
let from_value = NetworkGuid::from_value(12345);

let is_valid = net_guid.is_valid();
let is_default = net_guid.is_default();
```

### RepMovement

Replicated movement data:

```rust
let movement = RepMovement {
    location: Vector::new(100.0, 200.0, 50.0),
    rotation: Rotator::new(0.0, 45.0, 0.0),
    linear_velocity: Vector::new(500.0, 0.0, 0.0),
    angular_velocity: Vector::ZERO,
};

println!("{}", movement);
```

### NetworkStats

Network performance metrics:

```rust
let mut stats = NetworkStats::new();
stats.ping = 50.0;          // ms
stats.packet_loss = 0.02;   // 2%
stats.jitter = 25.0;        // ms

let is_good = stats.is_connection_good();
let quality = stats.connection_quality(); // 0.0 to 1.0
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

println!("JSON: {}", pretty_json);
```

### Binary Serialization (bincode)

```rust
// All types implement BinarySerializable trait
let color = LinearColor::RED;

// Serialize to binary
let binary_data = color.to_binary()?;
println!("Binary size: {} bytes", binary_data.len());

// Deserialize from binary
let restored = LinearColor::from_binary(&binary_data)?;
assert_eq!(color, restored);
```

## Game Server Example

Complete example showing usage in a game server context:

```rust
use ue_types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    level_name: Name,
    match_id: Guid,
    start_time: DateTime,
    duration: Timespan,
    players: TArray<Player>,
    spawn_points: TMap<String, Transform>,
    pickup_locations: TSet<Vector>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    id: NetworkGuid,
    name: Text,
    transform: Transform,
    health: f32,
    team_color: LinearColor,
    bounding_box: BoundingBox,
    movement: RepMovement,
    inventory: TArray<Item>,
    stats: TMap<String, i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: Guid,
    name: Name,
    rarity_color: Color,
    transform: Transform,
    pickup_radius: BoundingSphere,
}

// Implement binary serialization for network protocol
impl BinarySerializable for GameState {}
impl BinarySerializable for Player {}
impl BinarySerializable for Item {}

// Implement display for logging
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {} '{}' at {} (HP: {:.1})", 
               self.id, self.name.as_str(), self.transform.location, self.health)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create game state
    let mut game_state = GameState {
        level_name: Name::new("MainLevel"),
        match_id: Guid::new_guid(),
        start_time: DateTime::now(),
        duration: Timespan::from_minutes(15.0),
        players: TArray::new(),
        spawn_points: TMap::new(),
        pickup_locations: TSet::new(),
    };

    // Add spawn points
    game_state.spawn_points.add("TeamA_Spawn1".to_string(), 
        Transform::from_location(Vector::new(-500.0, 0.0, 100.0)));
    game_state.spawn_points.add("TeamB_Spawn1".to_string(),
        Transform::from_location(Vector::new(500.0, 0.0, 100.0)));

    // Add pickup locations
    game_state.pickup_locations.add(Vector::new(0.0, 0.0, 50.0));
    game_state.pickup_locations.add(Vector::new(250.0, 250.0, 50.0));

    // Create a player
    let player = Player {
        id: NetworkGuid::new(),
        name: Text::from_key("UI", "PlayerName", "Alice"),
        transform: Transform::from_location_rotator_scale(
            Vector::new(100.0, 200.0, 100.0),
            Rotator::new(0.0, 45.0, 0.0),
            Vector::ONE
        ),
        health: 100.0,
        team_color: LinearColor::BLUE,
        bounding_box: BoundingBox::from_center_and_extent(
            Vector::ZERO, 
            Vector::new(30.0, 30.0, 90.0)  // Capsule-like bounds
        ),
        movement: RepMovement {
            location: Vector::new(100.0, 200.0, 100.0),
            rotation: Rotator::new(0.0, 45.0, 0.0),
            linear_velocity: Vector::new(350.0, 0.0, 0.0),
            angular_velocity: Vector::ZERO,
        },
        inventory: TArray::new(),
        stats: TMap::new(),
    };

    // Add player to game
    game_state.players.add(player);
    
    // Display game state
    println!("Game State: Level {} ({})", 
        game_state.level_name.as_str(), 
        game_state.match_id
    );
    println!("Players: {}", game_state.players.num());
    println!("Spawn points: {}", game_state.spawn_points.num());

    // JSON serialization for web APIs/tools
    let json = serde_json::to_string_pretty(&game_state)?;
    println!("\nJSON representation:\n{}", json);

    // Binary serialization for network protocol
    let binary = game_state.to_binary()?;
    println!("\nBinary size: {} bytes", binary.len());

    // Deserialize and verify
    let restored_state = GameState::from_binary(&binary)?;
    println!("Restored game with {} players", restored_state.players.num());

    // Demonstrate spatial queries
    let player_pos = game_state.players.get(0).unwrap().transform.location;
    let nearby_pickups: Vec<Vector> = game_state.pickup_locations
        .iter()
        .filter(|&&pickup_pos| player_pos.distance_to(pickup_pos) < 100.0)
        .copied()
        .collect();
    
    println!("Pickups within 100 units: {}", nearby_pickups.len());

    Ok(())
}
```

## Performance Considerations

- **SIMD Optimizations**: Built on `glam` which uses SIMD when available
- **Zero-cost Abstractions**: Wrapper types compile to efficient native code
- **Memory Layout**: Types are `#[repr(C)]` compatible where applicable
- **Efficient Serialization**: Binary format is compact and cache-friendly
- **Hash-based Collections**: TMap and TSet use high-performance hashmaps

### Benchmarking

```rust
use std::time::Instant;

// Vector operations are SIMD-optimized
let vectors: Vec<Vector> = (0..1_000_000)
    .map(|i| Vector::new(i as f32, i as f32, i as f32))
    .collect();

let start = Instant::now();
let sum: Vector = vectors.iter().fold(Vector::ZERO, |acc, v| acc + *v);
let duration = start.elapsed();

println!("Summed {} vectors in {:?}", vectors.len(), duration);
```

## UE5 Compatibility Notes

- **Coordinate System**: Left-handed (X=Forward, Y=Right, Z=Up)
- **Rotation Order**: YXZ Euler order for Rotators (matches UE5)
- **Units**: Distances in centimeters, rotations in degrees
- **Color Spaces**: Proper sRGB ↔ Linear conversion
- **Container Behavior**: TArray, TMap, TSet match UE5 semantics
- **Serialization**: Compatible with UE5 data formats where applicable

## Error Handling

```rust
use ue_types::*;

// Most operations return Results for proper error handling
match Guid::parse("invalid-guid-format") {
    Ok(guid) => println!("Parsed: {}", guid),
    Err(e) => println!("Parse error: {}", e),
}

// Serialization errors
match some_type.to_binary() {
    Ok(data) => println!("Serialized {} bytes", data.len()),
    Err(e) => println!("Serialization failed: {}", e),
}
```

## Testing

Run the test suite:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

Test specific modules:

```bash
cargo test vector
cargo test containers
cargo test serialization
```

## Feature Flags

```toml
[dependencies]
ue-types = { version = "0.1.1", default-features = false, features = ["serde"] }
```

Available features:
- `serde` (default) - JSON serialization support via serde
- `binary` (default) - Binary serialization support via bincode

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for your changes
4. Ensure all tests pass: `cargo test`
5. Check formatting: `cargo fmt`
6. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Changelog

### Version 0.1.1
- Added utility types: DateTime, Timespan, Guid, Name, Text, Version
- Extracted types into separate modules for better organization
- Improved documentation and examples
- Enhanced container types with more UE5-compatible methods

### Version 0.1.0
- Initial release with core math types
- Vector, Rotator, Transform, Color types
- Bounding volume types
- Basic container types
- JSON and binary serialization