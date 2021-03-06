use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Vec3f, inspectable)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

    /// Calculates the 'scalar triple' product
    /// (sometimes referred to as __[uvw]__) of the 3 given vectors.
    ///
    /// This is equivalent to __det([u v w])__ or
    /// __(u cross v) dot w__
    #[wasm_bindgen(js_name=scalarTriple)]
    pub fn scalar_triple(u: &Vec3, v: &Vec3, w: &Vec3) -> f32 {
        u.cross(v).dot(w)
    }

    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Vec3) {
        swap(self, other);
    }

    #[wasm_bindgen]
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[wasm_bindgen]
    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[wasm_bindgen]
    pub fn add(&self, other: &Vec3) -> Self {
        *self + *other
    }

    #[wasm_bindgen]
    pub fn sub(&self, other: &Vec3) -> Self {
        *self - *other
    }

    #[wasm_bindgen]
    pub fn mul(&self, other: f32) -> Self {
        *self * other
    }

    /// Returns a new vector with the same direction but a length of 1
    #[wasm_bindgen]
    pub fn normalized(&self) -> Self {
        let len_sq = self.dot(self);
        *self / len_sq.sqrt()
    }

    /// Squared length of this vector
    #[wasm_bindgen(js_name=lenSq)]
    pub fn len_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Length of this vector
    ///
    /// Calculated as `sqrt lenSq()`. If you only need to compare lengths of vectors (for example
    /// when sorting) prefer using `lenSq`.
    #[wasm_bindgen(js_name=len)]
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
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
            2 => &self.z,
            _ => panic!("Vec3 index {} is out of range", index),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
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

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, a: Vec3) -> f32 {
        self.dot(&a)
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
