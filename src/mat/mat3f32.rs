//! Basic 3 by 3 float matrices
use crate::vec::vec3f32::Point;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use wasm_bindgen::prelude::*;

type Storage = [[f32; 3]; 3];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Matrix {
    // since these are just arrays we can use a 2d array instead of a flat buffer
    // without any problems down the line
    pub values: Storage,
}

#[wasm_bindgen(js_name=Mat3f)]
pub struct Proxy {
    #[wasm_bindgen(skip)]
    pub val: Matrix,
}

#[wasm_bindgen(js_class=Mat3f)]
impl Proxy {
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
    pub fn left_prod(&self, a: &Point) -> Point {
        let a = self.val.left_prod(a.into());
        Point::from(a)
    }

    #[wasm_bindgen(js_name=rightProd)]
    /// Calculate the `M*a` product
    pub fn right_prod(&self, a: &Point) -> Point {
        let a = self.val.right_prod(a.into());
        Point::from(a)
    }

    #[wasm_bindgen(js_name=scaleMatrix)]
    pub fn scale(a: f32) -> Self {
        Matrix::scale(a).into()
    }
}

impl From<Matrix> for Proxy {
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
        let mut mat = Self::default();
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    mat.set(i, j, a);
                }
            }
        }
        mat
    }

    pub fn swap(&mut self, other: &mut Matrix) {
        swap(self, other);
    }

    pub fn at(&self, col: usize, row: usize) -> f32 {
        assert!(col < 3);
        assert!(row < 3);
        self.values[row][col]
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        assert!(col < 3);
        assert!(row < 3);
        &mut self.values[row][col]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        assert!(col < 3);
        assert!(row < 3);
        self.values[row][col] = val;
    }

    /// `v*M` where `M` is self
    pub fn left_prod(&self, v: [f32; 3]) -> [f32; 3] {
        let mut res = [0.0; 3];
        for c in 0..3 {
            for r in 0..3 {
                res[c] += v[r] * self.at(c, r);
            }
        }
        res
    }

    /// `M*v` where `M` is self
    pub fn right_prod(&self, v: [f32; 3]) -> [f32; 3] {
        let mut res = [0.0; 3];
        for r in 0..3 {
            for c in 0..3 {
                res[r] += v[c] * self.at(c, r);
            }
        }
        res
    }
}