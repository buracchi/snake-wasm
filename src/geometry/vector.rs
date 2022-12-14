use std::ops;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

/**
 * Rust currently override scalar operator instead of overloading, therefore this Vector class
 * represents an element of a Right Vector Space instead of a more common Left Vector Space.
 */
#[wasm_bindgen]
impl Vector {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self { Self { x, y } }

    pub fn length(&self) -> f64 { f64::hypot(self.x, self.y) }

    pub fn normalized(&self) -> Self { *self * (1 as f64 / self.length()) }

    pub fn opposite(&self) -> Self { *self * -1 }

    pub fn round(&self) -> Self { Self { x: self.x.round(), y: self.y.round() } }

    pub fn scalar_product(lhs: &Vector, rhs: &Vector) -> f64 { lhs.x * rhs.x + lhs.y * rhs.y }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool { f64::abs(self.x - other.x) < f64::EPSILON && f64::abs(self.y - other.y) < f64::EPSILON }

    fn ne(&self, other: &Self) -> bool { f64::abs(self.x - other.x) > f64::EPSILON || f64::abs(self.y - other.y) > f64::EPSILON }
}

macro_rules! impl_scalar_ops {
    (for $($T:ty),+) => {
        $(impl ops::Mul<$T> for Vector {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self::Output {
                Self {
                    x: self.x * rhs as f64,
                    y: self.y * rhs as f64,
                }
            }
        }
        impl ops::Div<$T> for Vector {
            type Output = Self;

            fn div(self, rhs: $T) -> Self::Output {
                Self {
                    x: self.x / rhs as f64,
                    y: self.y / rhs as f64,
                }
            }
        })*
    }
}

impl_scalar_ops!(for i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
