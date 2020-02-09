//! Basic 2d vectors
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

macro_rules! implvec2d {
    ($name: ident, $val: ty, $proxy: ident) => {
        pub mod $name {
            use super::*;

            #[wasm_bindgen(js_name=$proxy, inspectable)]
            #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
            pub struct Point {
                pub x: $val,
                pub y: $val,
            }

            #[wasm_bindgen(js_class=$proxy)]
            impl Point {
                #[wasm_bindgen(constructor)]
                pub fn new(x: $val, y: $val) -> Self {
                    Self { x, y }
                }

                #[wasm_bindgen]
                pub fn swap(&mut self, other: &mut Point) {
                    swap(self, other);
                }

                #[wasm_bindgen]
                pub fn transponent(&self) -> Self {
                    Self {
                        x: self.y,
                        y: self.x,
                    }
                }

                #[wasm_bindgen]
                pub fn dot(&self, b: Self) -> $val {
                    self.x * b.x + self.y + b.y
                }
            }

            impl Into<[$val; 2]> for Point {
                fn into(self) -> [$val; 2] {
                    [self.x, self.y]
                }
            }

            impl Into<[$val; 2]> for &Point {
                fn into(self) -> [$val; 2] {
                    [self.x, self.y]
                }
            }

            impl<'a> Into<[&'a mut $val; 2]> for &'a mut Point {
                fn into(self) -> [&'a mut $val; 2] {
                    [&mut self.x, &mut self.y]
                }
            }

            impl From<[$val; 2]> for Point {
                fn from([x, y]: [$val; 2]) -> Self {
                    Self { x, y }
                }
            }

            impl Index<usize> for Point {
                type Output = $val;
                fn index(&self, index: usize) -> &$val {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        _ => panic!("Point index {} is out of range", index),
                    }
                }
            }

            impl IndexMut<usize> for Point {
                fn index_mut(&mut self, index: usize) -> &mut $val {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        _ => panic!("Point index {} is out of range", index),
                    }
                }
            }

            impl AddAssign for Point {
                fn add_assign(&mut self, p: Self) {
                    self.x += p.x;
                    self.y += p.y;
                }
            }

            impl Add for Point {
                type Output = Self;

                fn add(mut self, p: Self) -> Self {
                    self += p;
                    self
                }
            }

            impl SubAssign for Point {
                fn sub_assign(&mut self, p: Self) {
                    self.x -= p.x;
                    self.y -= p.y;
                }
            }

            impl Sub for Point {
                type Output = Self;

                fn sub(mut self, p: Self) -> Self {
                    self -= p;
                    self
                }
            }

            impl MulAssign<$val> for Point {
                fn mul_assign(&mut self, a: $val) {
                    self.x *= a;
                    self.y *= a;
                }
            }

            impl Mul<$val> for Point {
                type Output = Self;

                fn mul(mut self, a: $val) -> Self {
                    self *= a;
                    self
                }
            }

            impl DivAssign<$val> for Point {
                fn div_assign(&mut self, a: $val) {
                    self.x /= a;
                    self.y /= a;
                }
            }

            impl Div<$val> for Point {
                type Output = Self;

                fn div(mut self, a: $val) -> Self {
                    self /= a;
                    self
                }
            }
        }
    };
}

implvec2d!(vec2f32, f32, Vec2Float);
