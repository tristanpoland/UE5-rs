//! UE5-style Name for fast string comparisons (using string interning concept)

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// UE5-style Name for fast string comparisons (using string interning concept)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name {
    /// The actual string value
    pub value: String,
    /// Cached hash for fast comparisons
    #[serde(skip)]
    hash: u64,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name(\"{}\")", self.value)
    }
}

impl BinarySerializable for Name {}

impl Name {
    /// Create a new Name
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        let hash = Self::calculate_hash(&value);
        Self { value, hash }
    }

    /// Get the string value
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Get the hash value
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Check if the name is empty
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get the length of the name
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Calculate hash for a string (simple FNV-1a hash)
    fn calculate_hash(s: &str) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in s.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}

impl Default for Name {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Name {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let name1 = Name::new("TestName");
        let name2 = Name::new("TestName");
        let name3 = Name::new("DifferentName");
        
        assert_eq!(name1, name2);
        assert_ne!(name1, name3);
        assert_eq!(name1.hash(), name2.hash());
        assert_ne!(name1.hash(), name3.hash());
    }
}