use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Vec3f, inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[wasm_bindgen(js_class=Vec3f)]
impl Vec3 {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Vec3) {
        swap(self, other);
    }

    #[wasm_bindgen]
    pub fn dot(&self, b: &Vec3) -> f32 {
        self.x * b.x + self.y + b.y + self.z + b.z
    }

    #[wasm_bindgen]
    pub fn add(&self, b: &Vec3) -> Self {
        *self + *b
    }

    #[wasm_bindgen]
    pub fn sub(&self, b: &Vec3) -> Self {
        *self - *b
    }
}

impl Into<[f32; 3]> for Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Into<[f32; 3]> for &Vec3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl<'a> Into<[&'a mut f32; 3]> for &'a mut Vec3 {
    fn into(self) -> [&'a mut f32; 3] {
        [&mut self.x, &mut self.y, &mut self.z]
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            3 => &self.z,
            _ => panic!("Vec3 index {} is out of range", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            3 => &mut self.z,
            _ => panic!("Vec3 index {} is out of range", index),
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, p: Self) {
        self.x += p.x;
        self.y += p.y;
        self.z += p.z;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(mut self, p: Self) -> Self {
        self += p;
        self
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, p: Self) {
        self.x -= p.x;
        self.y -= p.y;
        self.z -= p.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(mut self, p: Self) -> Self {
        self -= p;
        self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, a: f32) {
        self.x *= a;
        self.y *= a;
        self.z *= a;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(mut self, a: f32) -> Self {
        self *= a;
        self
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, a: f32) {
        self.x /= a;
        self.y /= a;
        self.z /= a;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(mut self, a: f32) -> Self {
        self /= a;
        self
    }
}
