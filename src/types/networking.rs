//! UE5 networking types for game server development

use crate::vector::*;
use crate::rotator::*;
use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Network GUID for identifying objects across the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkGUID {
    /// The unique identifier value
    pub value: u32,
}

impl fmt::Display for NetworkGUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NetworkGUID({})", self.value)
    }
}

impl BinarySerializable for NetworkGUID {}

impl NetworkGUID {
    /// Invalid/null network GUID
    pub const INVALID: Self = Self { value: 0 };

    /// Create a new network GUID
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// Check if this GUID is valid (non-zero)
    pub fn is_valid(self) -> bool {
        self.value != 0
    }

    /// Generate a new GUID based on current time (simple implementation)
    pub fn generate() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u32;
        Self::new(now)
    }
}

impl Default for NetworkGUID {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Replication information for network movement
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RepMovement {
    /// Current location
    pub location: Vector,
    /// Current rotation
    pub rotation: Rotator,
    /// Current linear velocity
    pub linear_velocity: Vector,
    /// Current angular velocity (pitch, yaw, roll rates)
    pub angular_velocity: Vector,
    /// Location base (for relative movement)
    pub location_base: Option<NetworkGUID>,
    /// Relative location offset
    pub relative_location: Vector,
    /// Server timestamp when this movement was recorded
    pub server_frame: u32,
    /// Whether this movement is simulated physics
    pub is_simulated: bool,
    /// Whether the location base is valid
    pub has_location_base: bool,
}

impl fmt::Display for RepMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RepMovement(Loc: ({:.2}, {:.2}, {:.2}), Rot: {}, Vel: ({:.2}, {:.2}, {:.2}), Frame: {})",
            self.location.x, self.location.y, self.location.z,
            self.rotation,
            self.linear_velocity.x, self.linear_velocity.y, self.linear_velocity.z,
            self.server_frame
        )
    }
}

impl BinarySerializable for RepMovement {}

impl RepMovement {
    /// Create a new RepMovement with default values
    pub fn new() -> Self {
        Self {
            location: Vector::ZERO,
            rotation: Rotator::ZERO,
            linear_velocity: Vector::ZERO,
            angular_velocity: Vector::ZERO,
            location_base: None,
            relative_location: Vector::ZERO,
            server_frame: 0,
            is_simulated: false,
            has_location_base: false,
        }
    }

    /// Create RepMovement from basic transform data
    pub fn from_transform(location: Vector, rotation: Rotator, velocity: Vector) -> Self {
        Self {
            location,
            rotation,
            linear_velocity: velocity,
            angular_velocity: Vector::ZERO,
            location_base: None,
            relative_location: Vector::ZERO,
            server_frame: 0,
            is_simulated: false,
            has_location_base: false,
        }
    }

    /// Set the location base for relative movement
    pub fn set_location_base(&mut self, base_guid: NetworkGUID, relative_location: Vector) {
        self.location_base = Some(base_guid);
        self.relative_location = relative_location;
        self.has_location_base = true;
    }

    /// Clear the location base
    pub fn clear_location_base(&mut self) {
        self.location_base = None;
        self.relative_location = Vector::ZERO;
        self.has_location_base = false;
    }

    /// Get the effective world location (considering location base)
    pub fn get_world_location(&self, base_transform_fn: Option<impl Fn(NetworkGUID) -> Option<Vector>>) -> Vector {
        if self.has_location_base {
            if let Some(base_guid) = self.location_base {
                if let Some(get_transform) = base_transform_fn {
                    if let Some(base_location) = get_transform(base_guid) {
                        return base_location + self.relative_location;
                    }
                }
            }
        }
        self.location
    }

    /// Update the server frame timestamp
    pub fn set_server_frame(&mut self, frame: u32) {
        self.server_frame = frame;
    }

    /// Mark as simulated physics movement
    pub fn set_simulated(&mut self, simulated: bool) {
        self.is_simulated = simulated;
    }
}

impl Default for RepMovement {
    fn default() -> Self {
        Self::new()
    }
}

/// Network statistics for monitoring connection quality
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Packets sent per second
    pub packets_out_per_second: f32,
    /// Packets received per second
    pub packets_in_per_second: f32,
    /// Bytes sent per second
    pub bytes_out_per_second: f32,
    /// Bytes received per second
    pub bytes_in_per_second: f32,
    /// Round trip time in milliseconds
    pub ping: f32,
    /// Packet loss percentage (0.0 to 1.0)
    pub packet_loss: f32,
    /// Network jitter in milliseconds
    pub jitter: f32,
    /// Connection uptime in seconds
    pub uptime: f32,
}

impl fmt::Display for NetworkStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NetworkStats(Ping: {:.1}ms, Loss: {:.2}%, Jitter: {:.1}ms, Out: {:.1} B/s, In: {:.1} B/s)",
            self.ping,
            self.packet_loss * 100.0,
            self.jitter,
            self.bytes_out_per_second,
            self.bytes_in_per_second
        )
    }
}

impl BinarySerializable for NetworkStats {}

impl NetworkStats {
    /// Create new network stats with default values
    pub fn new() -> Self {
        Self {
            packets_out_per_second: 0.0,
            packets_in_per_second: 0.0,
            bytes_out_per_second: 0.0,
            bytes_in_per_second: 0.0,
            ping: 0.0,
            packet_loss: 0.0,
            jitter: 0.0,
            uptime: 0.0,
        }
    }

    /// Check if the connection quality is good
    pub fn is_connection_good(&self) -> bool {
        self.ping < 100.0 && self.packet_loss < 0.05 && self.jitter < 50.0
    }

    /// Get connection quality as a value from 0.0 (worst) to 1.0 (best)
    pub fn connection_quality(&self) -> f32 {
        let ping_score = (1.0 - (self.ping / 200.0).min(1.0)).max(0.0);
        let loss_score = (1.0 - (self.packet_loss * 20.0).min(1.0)).max(0.0);
        let jitter_score = (1.0 - (self.jitter / 100.0).min(1.0)).max(0.0);
        
        (ping_score + loss_score + jitter_score) / 3.0
    }
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Player network information for game servers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerNetInfo {
    /// Unique player identifier
    pub player_id: NetworkGUID,
    /// Player's chosen display name
    pub player_name: String,
    /// Network connection statistics
    pub net_stats: NetworkStats,
    /// Whether the player is currently connected
    pub is_connected: bool,
    /// Player's current team (if applicable)
    pub team_id: i32,
    /// Player's network role (spectator, player, admin, etc.)
    pub role: PlayerRole,
    /// Time when the player joined (Unix timestamp)
    pub join_time: u64,
    /// Player's IP address (for admin purposes)
    pub ip_address: String,
}

/// Player network role enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerRole {
    /// Regular player
    Player,
    /// Spectator (can observe but not participate)
    Spectator,
    /// Game moderator
    Moderator,
    /// Server administrator
    Admin,
    /// AI bot
    Bot,
}

impl fmt::Display for PlayerRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            PlayerRole::Player => "Player",
            PlayerRole::Spectator => "Spectator",
            PlayerRole::Moderator => "Moderator",
            PlayerRole::Admin => "Admin",
            PlayerRole::Bot => "Bot",
        };
        write!(f, "{}", role_str)
    }
}

impl fmt::Display for PlayerNetInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PlayerNetInfo(ID: {}, Name: \"{}\", Role: {}, Connected: {}, {})",
            self.player_id,
            self.player_name,
            self.role,
            self.is_connected,
            self.net_stats
        )
    }
}

impl BinarySerializable for PlayerNetInfo {}

impl PlayerNetInfo {
    /// Create new player network information
    pub fn new(player_id: NetworkGUID, player_name: String, ip_address: String) -> Self {
        Self {
            player_id,
            player_name,
            net_stats: NetworkStats::new(),
            is_connected: true,
            team_id: -1, // No team
            role: PlayerRole::Player,
            join_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ip_address,
        }
    }

    /// Set the player's team
    pub fn set_team(&mut self, team_id: i32) {
        self.team_id = team_id;
    }

    /// Set the player's role
    pub fn set_role(&mut self, role: PlayerRole) {
        self.role = role;
    }

    /// Update network statistics
    pub fn update_net_stats(&mut self, stats: NetworkStats) {
        self.net_stats = stats;
    }

    /// Disconnect the player
    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }

    /// Reconnect the player
    pub fn reconnect(&mut self) {
        self.is_connected = true;
    }

    /// Get how long the player has been connected (in seconds)
    pub fn connection_duration(&self) -> u64 {
        if self.is_connected {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .saturating_sub(self.join_time)
        } else {
            0
        }
    }

    /// Check if the player has administrative privileges
    pub fn is_admin(&self) -> bool {
        matches!(self.role, PlayerRole::Admin | PlayerRole::Moderator)
    }

    /// Check if the player can participate in gameplay
    pub fn can_play(&self) -> bool {
        self.is_connected && matches!(self.role, PlayerRole::Player | PlayerRole::Bot)
    }
}

/// Game session information for matchmaking and server browser
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSessionInfo {
    /// Unique session identifier
    pub session_id: NetworkGUID,
    /// Session name/title
    pub session_name: String,
    /// Game mode being played
    pub game_mode: String,
    /// Current map/level
    pub map_name: String,
    /// Maximum number of players allowed
    pub max_players: i32,
    /// Current number of connected players
    pub current_players: i32,
    /// Whether the session is password protected
    pub is_private: bool,
    /// Whether the session allows spectators
    pub allow_spectators: bool,
    /// Server region/location
    pub region: String,
    /// Game difficulty level
    pub difficulty: i32,
    /// Session creation time
    pub created_time: u64,
    /// Additional custom properties
    pub custom_properties: std::collections::HashMap<String, String>,
}

impl fmt::Display for GameSessionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GameSession(\"{}\", {}/{} players, Map: {}, Mode: {}, Private: {})",
            self.session_name,
            self.current_players,
            self.max_players,
            self.map_name,
            self.game_mode,
            self.is_private
        )
    }
}

impl BinarySerializable for GameSessionInfo {}

impl GameSessionInfo {
    /// Create a new game session
    pub fn new(session_name: String, game_mode: String, map_name: String, max_players: i32) -> Self {
        Self {
            session_id: NetworkGUID::generate(),
            session_name,
            game_mode,
            map_name,
            max_players,
            current_players: 0,
            is_private: false,
            allow_spectators: true,
            region: "Unknown".to_string(),
            difficulty: 1,
            created_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            custom_properties: std::collections::HashMap::new(),
        }
    }

    /// Check if the session has available slots
    pub fn has_available_slots(&self) -> bool {
        self.current_players < self.max_players
    }

    /// Check if the session is full
    pub fn is_full(&self) -> bool {
        self.current_players >= self.max_players
    }

    /// Add a player to the session
    pub fn add_player(&mut self) -> bool {
        if self.has_available_slots() {
            self.current_players += 1;
            true
        } else {
            false
        }
    }

    /// Remove a player from the session
    pub fn remove_player(&mut self) -> bool {
        if self.current_players > 0 {
            self.current_players -= 1;
            true
        } else {
            false
        }
    }

    /// Set session privacy
    pub fn set_private(&mut self, is_private: bool) {
        self.is_private = is_private;
    }

    /// Set region
    pub fn set_region(&mut self, region: String) {
        self.region = region;
    }

    /// Add custom property
    pub fn set_custom_property(&mut self, key: String, value: String) {
        self.custom_properties.insert(key, value);
    }

    /// Get custom property
    pub fn get_custom_property(&self, key: &str) -> Option<&String> {
        self.custom_properties.get(key)
    }

    /// Get session age in seconds
    pub fn age_seconds(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .saturating_sub(self.created_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_guid() {
        let guid1 = NetworkGUID::new(12345);
        let guid2 = NetworkGUID::generate();
        
        assert!(guid1.is_valid());
        assert!(guid2.is_valid());
        assert!(!NetworkGUID::INVALID.is_valid());
        assert_ne!(guid1, guid2);
    }

    #[test]
    fn test_rep_movement() {
        let mut rep_movement = RepMovement::from_transform(
            Vector::new(10.0, 20.0, 30.0),
            Rotator::new(0.0, 90.0, 0.0),
            Vector::new(5.0, 0.0, 0.0)
        );
        
        assert_eq!(rep_movement.location, Vector::new(10.0, 20.0, 30.0));
        assert_eq!(rep_movement.linear_velocity, Vector::new(5.0, 0.0, 0.0));
        
        let base_guid = NetworkGUID::new(999);
        rep_movement.set_location_base(base_guid, Vector::new(1.0, 2.0, 3.0));
        
        assert!(rep_movement.has_location_base);
        assert_eq!(rep_movement.location_base, Some(base_guid));
        assert_eq!(rep_movement.relative_location, Vector::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_network_stats() {
        let mut stats = NetworkStats::new();
        stats.ping = 40.0;
        stats.packet_loss = 0.01; // 1%
        stats.jitter = 20.0;
        
        assert!(stats.is_connection_good());
        assert!(stats.connection_quality() > 0.7);
        
        stats.ping = 150.0;
        stats.packet_loss = 0.1; // 10%
        assert!(!stats.is_connection_good());
    }

    #[test]
    fn test_player_net_info() {
        let player_id = NetworkGUID::new(12345);
        let mut player = PlayerNetInfo::new(
            player_id,
            "TestPlayer".to_string(),
            "192.168.1.100".to_string()
        );
        
        assert!(player.is_connected);
        assert!(player.can_play());
        assert!(!player.is_admin());
        
        player.set_role(PlayerRole::Admin);
        assert!(player.is_admin());
        
        player.disconnect();
        assert!(!player.is_connected);
        assert!(!player.can_play());
    }

    #[test]
    fn test_game_session_info() {
        let mut session = GameSessionInfo::new(
            "Test Game".to_string(),
            "Deathmatch".to_string(),
            "dm_arena".to_string(),
            8
        );
        
        assert!(session.has_available_slots());
        assert!(!session.is_full());
        
        for _ in 0..8 {
            assert!(session.add_player());
        }
        
        assert!(!session.has_available_slots());
        assert!(session.is_full());
        assert!(!session.add_player()); // Should fail
        
        assert!(session.remove_player());
        assert!(session.has_available_slots());
    }

    #[test]
    fn test_display_formatting() {
        let guid = NetworkGUID::new(12345);
        assert_eq!(format!("{}", guid), "NetworkGUID(12345)");
        
        let role = PlayerRole::Admin;
        assert_eq!(format!("{}", role), "Admin");
        
        let stats = NetworkStats {
            ping: 50.0,
            packet_loss: 0.05,
            jitter: 25.0,
            bytes_out_per_second: 1024.0,
            bytes_in_per_second: 2048.0,
            ..Default::default()
        };
        
        let display_str = format!("{}", stats);
        assert!(display_str.contains("50.0ms"));
        assert!(display_str.contains("5.00%"));
        assert!(display_str.contains("25.0ms"));
    }

    #[test]
    fn test_json_serialization() {
        let guid = NetworkGUID::new(12345);
        let json = serde_json::to_string(&guid).unwrap();
        let deserialized: NetworkGUID = serde_json::from_str(&json).unwrap();
        assert_eq!(guid, deserialized);
        
        let rep_movement = RepMovement::from_transform(
            Vector::new(1.0, 2.0, 3.0),
            Rotator::new(0.0, 90.0, 0.0),
            Vector::new(5.0, 0.0, 0.0)
        );
        let json = serde_json::to_string(&rep_movement).unwrap();
        let deserialized: RepMovement = serde_json::from_str(&json).unwrap();
        assert_eq!(rep_movement, deserialized);
    }

    #[test]
    fn test_binary_serialization() {
        let player = PlayerNetInfo::new(
            NetworkGUID::new(12345),
            "TestPlayer".to_string(),
            "192.168.1.100".to_string()
        );
        
        let binary = player.to_binary().unwrap();
        let deserialized = PlayerNetInfo::from_binary(&binary).unwrap();
        assert_eq!(player, deserialized);
    }
}