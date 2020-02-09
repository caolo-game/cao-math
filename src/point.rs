//! Basic 2d vectors
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

macro_rules! implvec2d {
    ($name: ident, $val: ty) => {
        #[wasm_bindgen]
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub struct $name {
            pub x: $val,
            pub y: $val,
        }

        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new(x: $val, y: $val) -> Self {
                Self { x, y }
            }

            #[wasm_bindgen]
            pub fn swap(&mut self, other: &mut $name) {
                swap(self, other);
            }

            #[wasm_bindgen]
            pub fn transponent(&self) -> Self {
                Self {
                    x: self.y,
                    y: self.x,
                }
            }
        }

        impl Into<[$val; 2]> for $name {
            fn into(self) -> [$val; 2] {
                [self.x, self.y]
            }
        }

        impl Into<[$val; 2]> for &$name {
            fn into(self) -> [$val; 2] {
                [self.x, self.y]
            }
        }

        impl<'a> Into<[&'a mut $val; 2]> for &'a mut $name {
            fn into(self) -> [&'a mut $val; 2] {
                [&mut self.x, &mut self.y]
            }
        }

        impl From<[$val; 2]> for $name {
            fn from([x, y]: [$val; 2]) -> Self {
                Self { x, y }
            }
        }

        impl Index<usize> for $name {
            type Output = $val;
            fn index(&self, index: usize) -> &$val {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("Point index {} is out of range", index),
                }
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut $val {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("Point index {} is out of range", index),
                }
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, p: Self) {
                self.x += p.x;
                self.y += p.y;
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(mut self, p: Self) -> Self {
                self += p;
                self
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, p: Self) {
                self.x -= p.x;
                self.y -= p.y;
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(mut self, p: Self) -> Self {
                self -= p;
                self
            }
        }

        impl MulAssign<$val> for $name {
            fn mul_assign(&mut self, a: $val) {
                self.x *= a;
                self.y *= a;
            }
        }

        impl Mul<$val> for $name {
            type Output = Self;

            fn mul(mut self, a: $val) -> Self {
                self *= a;
                self
            }
        }

        impl DivAssign<$val> for $name {
            fn div_assign(&mut self, a: $val) {
                self.x /= a;
                self.y /= a;
            }
        }

        impl Div<$val> for $name {
            type Output = Self;

            fn div(mut self, a: $val) -> Self {
                self /= a;
                self
            }
        }
    };
}

implvec2d!(Point2, i32);
implvec2d!(Vec2, f32);

impl DivAssign<f32> for Point2 {
    fn div_assign(&mut self, a: f32) {
        let Point2 { x, y } = self;
        let [mut x, mut y] = [*x as f32, *y as f32];
        x /= a;
        y /= a;
        self.x = x as i32;
        self.y = y as i32;
    }
}

impl Div<f32> for Point2 {
    type Output = Self;

    fn div(mut self, a: f32) -> Self {
        self /= a;
        self
    }
}

impl MulAssign<f32> for Point2 {
    fn mul_assign(&mut self, a: f32) {
        let Point2 { x, y } = self;
        let [mut x, mut y] = [*x as f32, *y as f32];
        x *= a;
        y *= a;
        self.x = x as i32;
        self.y = y as i32;
    }
}

impl Mul<f32> for Point2 {
    type Output = Self;

    fn mul(mut self, a: f32) -> Self {
        self *= a;
        self
    }
}

macro_rules! implblas1 {
    ($name: ident, $val:ty, $dot:ident, $cross: ident) => {
        #[wasm_bindgen]
        pub fn $dot(a: $name, b: $name) -> $val {
            a.x * b.x + a.y + b.y
        }
    };
}

implblas1!(Point2, i32, dot_point2, cross_point2);
implblas1!(Vec2, f32, dot_vec2, cross_vec2);
