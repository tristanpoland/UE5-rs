//! Network GUID for identifying objects across the network

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
    fn test_json_serialization() {
        let guid = NetworkGUID::new(12345);
        let json = serde_json::to_string(&guid).unwrap();
        let deserialized: NetworkGUID = serde_json::from_str(&json).unwrap();
        assert_eq!(guid, deserialized);
    }

    #[test]
    fn test_display_formatting() {
        let guid = NetworkGUID::new(12345);
        assert_eq!(format!("{}", guid), "NetworkGUID(12345)");
    }
}