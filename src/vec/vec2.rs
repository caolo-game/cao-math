use super::vec3;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Vec2f, inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[wasm_bindgen(js_class=Vec2f)]
impl Vec2 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Vec2) {
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
    pub fn dot(&self, b: &Vec2) -> f32 {
        self.x * b.x + self.y + b.y
    }

    #[wasm_bindgen]
    pub fn add(&self, b: &Vec2) -> Self {
        *self + *b
    }

    #[wasm_bindgen]
    pub fn sub(&self, b: &Vec2) -> Self {
        *self - *b
    }
}

impl Into<[f32; 2]> for Vec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<[f32; 2]> for &Vec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl<'a> Into<[&'a mut f32; 2]> for &'a mut Vec2 {
    fn into(self) -> [&'a mut f32; 2] {
        [&mut self.x, &mut self.y]
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index {} is out of range", index),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index {} is out of range", index),
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, p: Self) {
        self.x += p.x;
        self.y += p.y;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(mut self, p: Self) -> Self {
        self += p;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, p: Self) {
        self.x -= p.x;
        self.y -= p.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(mut self, p: Self) -> Self {
        self -= p;
        self
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, a: f32) {
        self.x *= a;
        self.y *= a;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(mut self, a: f32) -> Self {
        self *= a;
        self
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, a: f32) {
        self.x /= a;
        self.y /= a;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(mut self, a: f32) -> Self {
        self /= a;
        self
    }
}

// impl interoperability
//
#[wasm_bindgen(js_class = Vec2f)]
impl Vec2 {
    #[wasm_bindgen(js_name=to3dVector)]
    /// Casts `this` to a 3d displacement/vector.
    pub fn to_3d_vector(&self) -> vec3::Vec3 {
        vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: Default::default(),
        }
    }

    #[wasm_bindgen(js_name=toHomogeneous)]
    /// Casts `this` to a homogenous coordinate representation
    pub fn to_homogeneous(&self, w: Option<f32>) -> vec3::Vec3 {
        vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: w.unwrap_or_default(),
        }
    }

    #[wasm_bindgen(js_name=fromHomogeneous)]
    /// Cast the Homogenous representation back to 2D
    pub fn from_homogeneous(point: vec3::Vec3) -> Self {
        if point.x.abs() < std::f32::EPSILON {
            // if x is about 0
            Self {
                x: point.x,
                y: point.y,
            }
        } else {
            Self {
                x: point.x / point.z,
                y: point.y / point.z,
            }
        }
    }
}
