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
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    #[wasm_bindgen]
    pub fn add(&self, other: &Vec2) -> Self {
        *self + *other
    }

    #[wasm_bindgen]
    pub fn sub(&self, other: &Vec2) -> Self {
        *self - *other
    }

    #[wasm_bindgen]
    pub fn mul(&self, other: f32) -> Self {
        *self * other
    }

    /// Returns a new vector with the same direction but a length of 1
    #[wasm_bindgen]
    pub fn normalized(&self) -> Self {
        let lensq = self.dot(self);
        *self / lensq.sqrt()
    }

    /// Squared length of this vector
    #[wasm_bindgen(js_name=lenSq)]
    pub fn len_sq(&self) -> f32 {
        self.dot(self)
    }

    /// Length of this vector.
    ///
    /// Calculated as `sqrt lenSq()`. If you only need to compare lengths of vectors (for example
    /// when sorting) prefer using `lenSq`.
    #[wasm_bindgen(js_name=len)]
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Rotate the vector around the origin counter-clockwise by `rad` radians.
    #[wasm_bindgen]
    pub fn rotate(&mut self, rad: f32) {
        let c = rad.cos();
        let s = rad.sin();
        self.x = self.x * c - self.y * s;
        self.y = self.x * s + self.y * c;
    }

    /// Returns the angle between the two vectors in radians
    #[wasm_bindgen(js_name=angleBetween)]
    pub fn angle_between(&self, other: &Vec2) -> f32 {
        let cos = self.dot(other) / (self.len_sq().sqrt() * other.len_sq().sqrt());
        cos.acos()
    }

    /// Returns the orientation of point __C__ in respect to the directed line __AB__.
    ///
    /// If the __orient2d(a, b, c)__ > 0, then __C__ lies to the left.
    ///
    /// If the __orient2d(a, b, c)__ < 0, then __C__ lies to the right.
    ///
    /// If the __orient2d(a, b, c)__ == 0, then __C__ lies to on the line.
    ///
    /// The absolute value of the returning number is double the area of the triangle __ABC__
    #[wasm_bindgen]
    pub fn orient2d(a: &Vec2, b: &Vec2, c: &Vec2) -> f32 {
        ((a.x - c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x))
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

impl Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, a: Vec2) -> Self::Output {
        self.dot(&a)
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

    #[wasm_bindgen(js_name=extend)]
    /// Casts `this` to a 3D coordinate representation
    pub fn extend(&self, w: Option<f32>) -> vec3::Vec3 {
        vec3::Vec3 {
            x: self.x,
            y: self.y,
            z: w.unwrap_or_default(),
        }
    }

    #[wasm_bindgen(js_name=fromHomogeneous)]
    /// Cast the Homogenous representation back to (x, y) representation
    pub fn from_homogeneous(point: vec3::Vec3) -> Self {
        if point.z.abs() < std::f32::EPSILON {
            // if z is about 0
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
