//! Basic 2 by 2 float matrices
use super::js_mat3::JsMat33;
use crate::vec::vec2::Vec2;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Mul, MulAssign};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mat22 {
    // column major storage
    pub x_axis: [f32; 2],
    pub y_axis: [f32; 2],
}

#[wasm_bindgen(js_name=Mat2f)]
pub struct JsMat22 {
    #[wasm_bindgen(skip)]
    pub val: Mat22,
}

#[wasm_bindgen(js_class=Mat2f)]
impl JsMat22 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Mat22::default();
        Self { val }
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.val.at(col, row)
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, a: f32) {
        *self.val.at_mut(col, row) = a;
    }

    #[wasm_bindgen(js_name=leftProd)]
    /// Calculate the `a*M` product
    pub fn left_prod(&self, a: &Vec2) -> Vec2 {
        let a = self.val.left_prod(a.into());
        Vec2::from(a)
    }

    #[wasm_bindgen(js_name=rightProd)]
    /// Calculate the `M*a` product
    pub fn right_prod(&self, a: &Vec2) -> Vec2 {
        let a = self.val.right_prod(a.into());
        Vec2::from(a)
    }

    #[wasm_bindgen(js_name=scaleMatrix)]
    pub fn scale(a: f32) -> Self {
        Mat22::scale(a).into()
    }

    #[wasm_bindgen(js_name=asMat3f)]
    /// Create a 3x3 matrix from `this=A`.
    /// ```txt
    /// | a00 a10 0 |
    /// | a01 a11 0 |
    /// |   0   0 1 |
    /// ```
    pub fn as_mat3f(&self) -> JsMat33 {
        let [a00, a01] = self.val.x_axis;
        let [a10, a11] = self.val.y_axis;
        JsMat33 {
            val: [[a00, a01, 0.], [a10, a11, 0.], [0., 0., 1.]].into(),
        }
    }
}

impl From<Mat22> for JsMat22 {
    fn from(val: Mat22) -> Self {
        Self { val }
    }
}

impl From<[[f32; 2]; 2]> for Mat22 {
    fn from([x_axis, y_axis]: [[f32; 2]; 2]) -> Self {
        Self { x_axis, y_axis }
    }
}

impl Mat22 {
    pub fn scale(a: f32) -> Self {
        Self {
            x_axis: [a, 0.],
            y_axis: [0., a],
        }
    }

    pub fn swap(&mut self, other: &mut Mat22) {
        swap(self, other);
    }

    pub fn axis(&self, col: usize) -> &[f32;2] {
        match col {
            0 => &self.x_axis,
            1 => &self.y_axis,
            _ => unreachable!()
        }
    }

    pub fn axis_mut(&mut self, col: usize) -> &mut [f32;2] {
        match col {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            _ => unreachable!()
        }
    }

    pub fn at(&self, col: usize, row: usize) -> f32 {
        assert!(col < 2);
        assert!(row < 2);
        self.axis(col)[row]
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        assert!(col < 2);
        assert!(row < 2);
        &mut self.axis_mut(col)[row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        assert!(col < 2);
        assert!(row < 2);
        self.axis_mut(col)[row] = val;
    }

    /// `v*M` where `M` is self
    pub fn left_prod(&self, v: [f32; 2]) -> [f32; 2] {
        [
            v[0] * self.at(0, 0) + v[1] * self.at(0, 1),
            v[0] * self.at(1, 0) + v[1] * self.at(1, 1),
        ]
    }

    /// `M*v` where `M` is self
    pub fn right_prod(&self, v: [f32; 2]) -> [f32; 2] {
        [
            v[0] * self.at(0, 0) + v[1] * self.at(1, 0),
            v[0] * self.at(0, 1) + v[1] * self.at(1, 1),
        ]
    }
}

impl Mul<f32> for Mat22 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Mat22 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x *= rhs);
        self.y_axis.iter_mut().for_each(|x| *x *= rhs);
    }
}
