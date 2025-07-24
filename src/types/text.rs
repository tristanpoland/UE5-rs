//! UE5-style Text for localized strings

use crate::BinarySerializable;
use serde::{Deserialize, Serialize};
use std::fmt;

/// UE5-style Text for localized strings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Text {
    /// The display string
    pub display_string: String,
    /// Namespace for localization
    pub namespace: Option<String>,
    /// Key for localization
    pub key: Option<String>,
    /// Source string for localization
    pub source_string: Option<String>,
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Text(\"{}\")", self.display_string)
    }
}

impl BinarySerializable for Text {}

impl Text {
    /// Create a new Text with just display string
    pub fn new(display_string: impl Into<String>) -> Self {
        Self {
            display_string: display_string.into(),
            namespace: None,
            key: None,
            source_string: None,
        }
    }

    /// Create a localized Text
    pub fn from_key(namespace: impl Into<String>, key: impl Into<String>, source: impl Into<String>) -> Self {
        let source_string = source.into();
        Self {
            display_string: source_string.clone(), // Default to source until localized
            namespace: Some(namespace.into()),
            key: Some(key.into()),
            source_string: Some(source_string),
        }
    }

    /// Get the display string
    pub fn as_str(&self) -> &str {
        &self.display_string
    }

    /// Check if the text is empty
    pub fn is_empty(&self) -> bool {
        self.display_string.is_empty()
    }

    /// Check if this is a localizable text (has namespace and key)
    pub fn is_localizable(&self) -> bool {
        self.namespace.is_some() && self.key.is_some()
    }

    /// Update the display string (for localization)
    pub fn set_display_string(&mut self, display_string: impl Into<String>) {
        self.display_string = display_string.into();
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for Text {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text1 = Text::new("Hello World");
        assert_eq!(text1.as_str(), "Hello World");
        assert!(!text1.is_localizable());
        
        let text2 = Text::from_key("UI", "HelloWorld", "Hello World");
        assert!(text2.is_localizable());
        assert_eq!(text2.as_str(), "Hello World");
    }
}