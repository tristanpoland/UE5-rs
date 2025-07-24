//! Player network information for game servers

use crate::BinarySerializable;
use super::{NetworkGUID, NetworkStats};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_player_role_display() {
        let role = PlayerRole::Admin;
        assert_eq!(format!("{}", role), "Admin");
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