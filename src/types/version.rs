//! UE5-style Version information

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// UE5-style Version information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Version {
    /// Major version number
    pub major: u16,
    /// Minor version number
    pub minor: u16,
    /// Patch version number
    pub patch: u16,
    /// Build number
    pub build: u16,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version({}.{}.{}.{})", self.major, self.minor, self.patch, self.build)
    }
}

impl BinarySerializable for Version {}

impl Version {
    /// Create a new version
    pub fn new(major: u16, minor: u16, patch: u16, build: u16) -> Self {
        Self { major, minor, patch, build }
    }

    /// Create version with just major.minor.patch
    pub fn from_semver(major: u16, minor: u16, patch: u16) -> Self {
        Self::new(major, minor, patch, 0)
    }

    /// Parse version string (format: "major.minor.patch.build" or "major.minor.patch")
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() < 3 || parts.len() > 4 {
            return Err("Invalid version format");
        }

        let major = parts[0].parse().map_err(|_| "Invalid major version")?;
        let minor = parts[1].parse().map_err(|_| "Invalid minor version")?;
        let patch = parts[2].parse().map_err(|_| "Invalid patch version")?;
        let build = if parts.len() > 3 {
            parts[3].parse().map_err(|_| "Invalid build number")?
        } else {
            0
        };

        Ok(Self::new(major, minor, patch, build))
    }

    /// Convert to string representation
    pub fn to_string(self) -> String {
        if self.build == 0 {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
        }
    }

    /// Check if this version is compatible with another (same major version)
    pub fn is_compatible_with(self, other: Version) -> bool {
        self.major == other.major
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(1, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v1 = Version::new(1, 2, 3, 4);
        assert_eq!(v1.to_string(), "1.2.3.4");
        
        let v2 = Version::from_semver(1, 2, 3);
        assert_eq!(v2.to_string(), "1.2.3");
        
        let parsed = Version::parse("1.2.3.4").unwrap();
        assert_eq!(parsed, v1);
        
        assert!(v1.is_compatible_with(v2));
    }
}