//! UE5-style GUID for unique identifiers

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// UE5-style GUID for unique identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Guid {
    /// First 32 bits
    pub a: u32,
    /// Second 32 bits
    pub b: u32,
    /// Third 32 bits
    pub c: u32,
    /// Fourth 32 bits
    pub d: u32,
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guid({:08X}-{:08X}-{:08X}-{:08X})", self.a, self.b, self.c, self.d)
    }
}

impl BinarySerializable for Guid {}

impl Guid {
    /// Invalid/null GUID
    pub const INVALID: Self = Self { a: 0, b: 0, c: 0, d: 0 };

    /// Create a new GUID
    pub fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
        Self { a, b, c, d }
    }

    /// Generate a new GUID (simple pseudo-random implementation)
    pub fn new_guid() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        // Simple pseudo-random generation based on current time
        let a = (now & 0xFFFFFFFF) as u32;
        let b = ((now >> 32) & 0xFFFFFFFF) as u32;
        let c = (now.wrapping_mul(1103515245).wrapping_add(12345) & 0xFFFFFFFF) as u32;
        let d = (now.wrapping_mul(214013).wrapping_add(2531011) & 0xFFFFFFFF) as u32;
        
        Self::new(a, b, c, d)
    }

    /// Parse GUID from string (format: XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX)
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 4 {
            return Err("Invalid GUID format");
        }

        let a = u32::from_str_radix(parts[0], 16).map_err(|_| "Invalid hex in part A")?;
        let b = u32::from_str_radix(parts[1], 16).map_err(|_| "Invalid hex in part B")?;
        let c = u32::from_str_radix(parts[2], 16).map_err(|_| "Invalid hex in part C")?;
        let d = u32::from_str_radix(parts[3], 16).map_err(|_| "Invalid hex in part D")?;

        Ok(Self::new(a, b, c, d))
    }

    /// Convert to string representation
    pub fn to_string(self) -> String {
        format!("{:08X}-{:08X}-{:08X}-{:08X}", self.a, self.b, self.c, self.d)
    }

    /// Check if this GUID is valid (non-zero)
    pub fn is_valid(self) -> bool {
        self.a != 0 || self.b != 0 || self.c != 0 || self.d != 0
    }

    /// Convert to byte array (big-endian)
    pub fn to_bytes(self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&self.a.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.b.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.c.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.d.to_be_bytes());
        bytes
    }

    /// Create from byte array (big-endian)
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let a = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let b = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let c = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let d = u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        Self::new(a, b, c, d)
    }
}

impl Default for Guid {
    fn default() -> Self {
        Self::INVALID
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid() {
        let guid1 = Guid::new_guid();
        let guid2 = Guid::new_guid();
        
        assert!(guid1.is_valid());
        assert!(guid2.is_valid());
        assert_ne!(guid1, guid2);
        
        let guid_str = guid1.to_string();
        let parsed = Guid::parse(&guid_str).unwrap();
        assert_eq!(guid1, parsed);
        
        let binary = guid1.to_binary().unwrap();
        let deserialized = Guid::from_binary(&binary).unwrap();
        assert_eq!(guid1, deserialized);
    }
}