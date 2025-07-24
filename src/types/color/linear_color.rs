//! Linear Color (0.0 to 1.0 range, HDR capable)

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_linear_color_display() {
        let color = LinearColor::new(0.5, 0.75, 1.0, 0.8);
        let display_str = format!("{}", color);
        assert!(display_str.contains("R=0.500"));
        assert!(display_str.contains("G=0.750"));
        assert!(display_str.contains("B=1.000"));
        assert!(display_str.contains("A=0.800"));
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
    fn test_linear_color_binary_serialization() {
        let color = LinearColor::new(0.5, 0.75, 1.0, 0.8);
        
        // Test binary serialization
        let binary = color.to_binary().unwrap();
        let deserialized = LinearColor::from_binary(&binary).unwrap();
        
        assert!(color.is_nearly_equal(deserialized, 0.001));
    }
}