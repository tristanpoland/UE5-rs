use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }
    
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 0.01
    }
    
    pub fn get_safe_normal(self, tolerance: f32) -> Vector {
        let square_sum = self.length_squared();
        if square_sum == 1.0 {
            return self;
        } else if square_sum < tolerance * tolerance {
            return Vector::ZERO;
        } else if !square_sum.is_finite() || square_sum.is_nan() {
            return Vector::ZERO;
        }
        let len = square_sum.sqrt();
        if !len.is_finite() || len.is_nan() || len == 0.0 {
            return Vector::ZERO;
        }
        let norm = self / len;
        // Clamp to unit length if overflow occurred
        if norm.length().is_infinite() || norm.length().is_nan() {
            return Vector::ZERO;
        }
        norm
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Vector;
    
    fn div(self, scalar: f32) -> Self::Output {
        Vector::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

fn main() {
    let huge_vec = Vector::new(f32::MAX / 2.0, f32::MAX / 2.0, f32::MAX / 2.0);
    println!("huge_vec: {:?}", huge_vec);
    println!("huge_vec.length(): {}", huge_vec.length());
    println!("huge_vec.length_squared(): {}", huge_vec.length_squared());
    
    let huge_normalized = huge_vec.get_safe_normal(0.001);
    println!("huge_normalized: {:?}", huge_normalized);
    println!("huge_normalized.length(): {}", huge_normalized.length());
    println!("huge_normalized.is_normalized(): {}", huge_normalized.is_normalized());
    println!("huge_normalized.length().is_infinite(): {}", huge_normalized.length().is_infinite());
    
    let test_result = huge_normalized.is_normalized() || huge_normalized.length().is_infinite();
    println!("Test result: {}", test_result);
}
