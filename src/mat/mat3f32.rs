//! Basic 3 by 3 float matrices
use crate::vec::vec2f32::Point as P2;
use crate::vec::vec3f32::Point;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Mul, MulAssign};
use wasm_bindgen::prelude::*;

type Storage = [[f32; 3]; 3];

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Matrix {
    // since these are just arrays we can use a 2d array instead of a flat buffer
    // without any problems down the line
    pub values: Storage,
}

#[wasm_bindgen(js_name=Mat3f)]
pub struct JsMatrix {
    #[wasm_bindgen(skip)]
    pub val: Matrix,
}

#[wasm_bindgen(js_class=Mat3f)]
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

    #[wasm_bindgen(js_name=translateMatrix)]
    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    pub fn translate(t: &P2) -> Self {
        Matrix::translate(t.into()).into()
    }

    /// Multiply by scalar value
    #[wasm_bindgen(js_name=scalarMul)]
    pub fn scalar_mul(&self, f: f32) -> JsMatrix {
        let val = self.val.clone() * f;
        Self { val }
    }

    /// Multiply two matrices
    #[wasm_bindgen(js_name=matrixMul)]
    pub fn mul(&self, b: &JsMatrix) -> JsMatrix {
        let mut res = Matrix::default();
        self.val.mat_mul(&b.val, &mut res);
        res.into()
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

    /// Calculate `A*B=C` where `A` is self
    #[allow(non_snake_case)]
    pub fn mat_mul(&self, B: &Self, C: &mut Self) {
        for c in 0..3 {
            for r in 0..3 {
                let x = C.at_mut(c, r);
                *x = 0.;
                for i in 0..3 {
                    *x += self.at(i, r) * B.at(c, i);
                }
            }
        }
    }

    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    pub fn translate([x, y]: [f32; 2]) -> Self {
        Self {
            values: [[1., 0., x], [0., 1., y], [0., 0., 1.]],
        }
    }
}

impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        self.values
            .iter_mut()
            .flat_map(|x| x.iter_mut())
            .for_each(|x| *x *= rhs);
    }
}
