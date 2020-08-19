//! Basic 2 by 2 float matrices
use super::js_mat3::{self, JsMatrix as Mat3f};
use crate::vec::vec2::Vec2;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use wasm_bindgen::prelude::*;

type Storage = [[f32; 2]; 2];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Matrix {
    // since these are just arrays we can use a 2d array instead of a flat buffer
    // without any problems down the line
    pub values: Storage,
}

#[wasm_bindgen(js_name=Mat2f)]
pub struct JsMatrix {
    #[wasm_bindgen(skip)]
    pub val: Matrix,
}

#[wasm_bindgen(js_class=Mat2f)]
impl JsMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Matrix::default();
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
        Matrix::scale(a).into()
    }

    #[wasm_bindgen(js_name=asMat3f)]
    /// Create a 3x3 matrix from `this=A`.
    /// ```txt
    /// | a00 a10 0 |
    /// | a01 a11 0 |
    /// |   0   0 1 |
    /// ```
    pub fn as_mat3f(&self) -> Mat3f {
        let [[a00, a10], [a01, a11]] = self.val.values.clone();
        Mat3f {
            val: js_mat3::Payload::Matrix([[a00, a10, 0.], [a01, a11, 0.], [0., 0., 1.]].into()),
        }
    }
}

impl From<Matrix> for JsMatrix {
    fn from(val: Matrix) -> Self {
        Self { val }
    }
}

impl From<Storage> for Matrix {
    fn from(values: Storage) -> Self {
        Self { values }
    }
}

impl Matrix {
    pub fn scale(a: f32) -> Self {
        Self {
            values: [[a, 0.], [0., a]],
        }
    }

    pub fn swap(&mut self, other: &mut Matrix) {
        swap(self, other);
    }

    pub fn at(&self, col: usize, row: usize) -> f32 {
        assert!(col < 2);
        assert!(row < 2);
        self.values[row][col]
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        assert!(col < 2);
        assert!(row < 2);
        &mut self.values[row][col]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        assert!(col < 2);
        assert!(row < 2);
        self.values[row][col] = val;
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
