//! Color types for UE-style color handling

use crate::BinarySerializable;
use glam::Vec4;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Linear Color (0.0 to 1.0 range, HDR capable)
/// 
/// Represents color in linear color space with floating point precision.
/// This is the preferred color format for mathematical operations and shaders.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LinearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl fmt::Display for LinearColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinearColor(R={:.3}, G={:.3}, B={:.3}, A={:.3})", 
               self.r, self.g, self.b, self.a)
    }
}

impl BinarySerializable for LinearColor {}

impl LinearColor {
    /// Pure white color
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    /// Pure black color
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    /// Pure red color
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    /// Pure green color
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    /// Pure blue color
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    /// Yellow color
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    /// Cyan color
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    /// Magenta color
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    /// Transparent color
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    /// Gray (50% brightness)
    pub const GRAY: Self = Self { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };

    /// Create a new linear color with the given RGBA values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new linear color with RGB values and full alpha
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Create a grayscale color
    pub fn gray(value: f32) -> Self {
        Self { r: value, g: value, b: value, a: 1.0 }
    }

    /// Create from HSV (Hue, Saturation, Value) color space
    pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
        let h = h % 360.0;
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Self::rgb(r + m, g + m, b + m)
    }

    /// Convert to Vec4 for shader usage
    pub fn to_vec4(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Convert from Vec4
    pub fn from_vec4(v: Vec4) -> Self {
        Self { r: v.x, g: v.y, b: v.z, a: v.w }
    }

    /// Get luminance (perceived brightness)
    pub fn luminance(self) -> f32 {
        0.299 * self.r + 0.587 * self.g + 0.114 * self.b
    }

    /// Linearly interpolate between two colors
    pub fn lerp(self, other: LinearColor, alpha: f32) -> Self {
        Self {
            r: self.r + alpha * (other.r - self.r),
            g: self.g + alpha * (other.g - self.g),
            b: self.b + alpha * (other.b - self.b),
            a: self.a + alpha * (other.a - self.a),
        }
    }

    /// Multiply color by a scalar (for brightness adjustment)
    pub fn scale(self, factor: f32) -> Self {
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a,
        }
    }

    /// Clamp color values to [0.0, 1.0] range
    pub fn clamp(self) -> Self {
        Self {
            r: self.r.clamp(0.0, 1.0),
            g: self.g.clamp(0.0, 1.0),
            b: self.b.clamp(0.0, 1.0),
            a: self.a.clamp(0.0, 1.0),
        }
    }

    /// Check if the color is nearly equal to another
    pub fn is_nearly_equal(self, other: LinearColor, tolerance: f32) -> bool {
        (self.r - other.r).abs() <= tolerance
            && (self.g - other.g).abs() <= tolerance
            && (self.b - other.b).abs() <= tolerance
            && (self.a - other.a).abs() <= tolerance
    }
}

/// Color with 0-255 integer values (sRGB color space)
/// 
/// Standard 8-bit per channel color representation commonly used in textures
/// and UI elements. Values are in sRGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(R={}, G={}, B={}, A={}) [#{:02X}{:02X}{:02X}{:02X}]", 
               self.r, self.g, self.b, self.a, self.r, self.g, self.b, self.a)
    }
}

impl BinarySerializable for Color {}

impl Color {
    /// Pure white color
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
    /// Pure black color
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    /// Pure red color
    pub const RED: Self = Self { r: 255, g: 0, b: 0, a: 255 };
    /// Pure green color
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0, a: 255 };
    /// Pure blue color
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255, a: 255 };
    /// Yellow color
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0, a: 255 };
    /// Cyan color
    pub const CYAN: Self = Self { r: 0, g: 255, b: 255, a: 255 };
    /// Magenta color
    pub const MAGENTA: Self = Self { r: 255, g: 0, b: 255, a: 255 };
    /// Transparent color
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    /// Gray (50% brightness)
    pub const GRAY: Self = Self { r: 128, g: 128, b: 128, a: 255 };

    /// Create a new color with the given RGBA values
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a new color with RGB values and full alpha
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a grayscale color
    pub fn gray(value: u8) -> Self {
        Self { r: value, g: value, b: value, a: 255 }
    }

    /// Create from hex color code (e.g., 0xFF0000 for red)
    pub fn from_hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    /// Create from hex color code with alpha (e.g., 0xFF0000FF for red)
    pub fn from_hex_rgba(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }

    /// Convert to hex value (RGB only)
    pub fn to_hex(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Convert to hex value with alpha
    pub fn to_hex_rgba(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    /// Convert to LinearColor (applies sRGB to linear conversion)
    pub fn to_linear(self) -> LinearColor {
        fn srgb_to_linear(value: u8) -> f32 {
            let normalized = value as f32 / 255.0;
            if normalized <= 0.04045 {
                normalized / 12.92
            } else {
                ((normalized + 0.055) / 1.055).powf(2.4)
            }
        }

        LinearColor {
            r: srgb_to_linear(self.r),
            g: srgb_to_linear(self.g),
            b: srgb_to_linear(self.b),
            a: self.a as f32 / 255.0,
        }
    }

    /// Convert from LinearColor (applies linear to sRGB conversion)
    pub fn from_linear(linear: LinearColor) -> Self {
        fn linear_to_srgb(value: f32) -> u8 {
            let clamped = value.clamp(0.0, 1.0);
            let converted = if clamped <= 0.0031308 {
                clamped * 12.92
            } else {
                1.055 * clamped.powf(1.0 / 2.4) - 0.055
            };
            (converted * 255.0).round() as u8
        }

        Self {
            r: linear_to_srgb(linear.r),
            g: linear_to_srgb(linear.g),
            b: linear_to_srgb(linear.b),
            a: (linear.a * 255.0).round() as u8,
        }
    }

    /// Get luminance (perceived brightness) 0-255
    pub fn luminance(self) -> u8 {
        let linear = self.to_linear();
        (linear.luminance() * 255.0).round() as u8
    }

    /// Linearly interpolate between two colors
    pub fn lerp(self, other: Color, alpha: f32) -> Self {
        let alpha = alpha.clamp(0.0, 1.0);
        let inv_alpha = 1.0 - alpha;
        
        Self {
            r: (self.r as f32 * inv_alpha + other.r as f32 * alpha).round() as u8,
            g: (self.g as f32 * inv_alpha + other.g as f32 * alpha).round() as u8,
            b: (self.b as f32 * inv_alpha + other.b as f32 * alpha).round() as u8,
            a: (self.a as f32 * inv_alpha + other.a as f32 * alpha).round() as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_hex_conversion() {
        let red = Color::from_hex(0xFF0000);
        assert_eq!(red, Color::RED);
        assert_eq!(red.to_hex(), 0xFF0000);
    }

    #[test]
    fn test_color_linear_conversion() {
        let color = Color::rgb(128, 128, 128);
        let linear = color.to_linear();
        let back_to_color = Color::from_linear(linear);
        
        // Should be approximately equal (gamma conversion introduces small errors)
        assert!((color.r as i16 - back_to_color.r as i16).abs() <= 1);
        assert!((color.g as i16 - back_to_color.g as i16).abs() <= 1);
        assert!((color.b as i16 - back_to_color.b as i16).abs() <= 1);
    }

    #[test]
    fn test_linear_color_hsv() {
        let red = LinearColor::from_hsv(0.0, 1.0, 1.0);
        assert!(red.is_nearly_equal(LinearColor::RED, 0.001));
        
        let green = LinearColor::from_hsv(120.0, 1.0, 1.0);
        assert!(green.is_nearly_equal(LinearColor::GREEN, 0.001));
        
        let blue = LinearColor::from_hsv(240.0, 1.0, 1.0);
        assert!(blue.is_nearly_equal(LinearColor::BLUE, 0.001));
    }

    #[test]
    fn test_color_lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;
        let gray = black.lerp(white, 0.5);
        
        assert!(gray.r >= 127 && gray.r <= 128);
        assert!(gray.g >= 127 && gray.g <= 128);
        assert!(gray.b >= 127 && gray.b <= 128);
    }

    #[test]
    fn test_linear_color_display() {
        let color = LinearColor::new(0.5, 0.75, 1.0, 0.8);
        let display_str = format!("{}", color);
        assert!(display_str.contains("R=0.500"));
        assert!(display_str.contains("G=0.750"));
        assert!(display_str.contains("B=1.000"));
        assert!(display_str.contains("A=0.800"));
    }

    #[test]
    fn test_color_display() {
        let color = Color::new(255, 128, 64, 255);
        let display_str = format!("{}", color);
        assert!(display_str.contains("R=255"));
        assert!(display_str.contains("G=128"));
        assert!(display_str.contains("B=64"));
        assert!(display_str.contains("#FF8040FF"));
    }

    #[test]
    fn test_linear_color_json_serialization() {
        let color = LinearColor::new(0.5, 0.75, 1.0, 0.8);
        
        // Test JSON serialization
        let json = serde_json::to_string(&color).unwrap();
        let deserialized: LinearColor = serde_json::from_str(&json).unwrap();
        
        assert!(color.is_nearly_equal(deserialized, 0.001));
    }

    #[test]
    fn test_color_json_serialization() {
        let color = Color::new(255, 128, 64, 255);
        
        // Test JSON serialization
        let json = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&json).unwrap();
        
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_linear_color_binary_serialization() {
        let color = LinearColor::new(0.5, 0.75, 1.0, 0.8);
        
        // Test binary serialization
        let binary = color.to_binary().unwrap();
        let deserialized = LinearColor::from_binary(&binary).unwrap();
        
        assert!(color.is_nearly_equal(deserialized, 0.001));
    }

    #[test]
    fn test_color_binary_serialization() {
        let color = Color::new(255, 128, 64, 255);
        
        // Test binary serialization
        let binary = color.to_binary().unwrap();
        let deserialized = Color::from_binary(&binary).unwrap();
        
        assert_eq!(color, deserialized);
    }
}