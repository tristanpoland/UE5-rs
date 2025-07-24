//! Game session information for matchmaking and server browser

use crate::BinarySerializable;
use super::NetworkGUID;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

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
}