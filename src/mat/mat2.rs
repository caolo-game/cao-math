//! Basic 2 by 2 float matrices
use crate::mat::mat3::Mat3f;
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

/// 2 by 2 column major matrix
#[wasm_bindgen(js_name=Mat2f)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Mat2f {
    #[wasm_bindgen(skip)]
    pub x_axis: [f32; 2],
    #[wasm_bindgen(skip)]
    pub y_axis: [f32; 2],
}

impl From<[[f32; 2]; 2]> for Mat2f {
    fn from([x_axis, y_axis]: [[f32; 2]; 2]) -> Self {
        Self { x_axis, y_axis }
    }
}
impl Mat2f {
    pub fn axis_mut(&mut self, col: usize) -> &mut [f32; 2] {
        match col {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            _ => unreachable!(),
        }
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        &mut self.axis_mut(col)[row]
    }
}

#[wasm_bindgen(js_name=Mat2f)]
impl Mat2f {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    /// Converts `this` to a 3 by 3 matrix
    ///
    /// __Layout__
    ///
    /// ```txt
    /// | this11    this12    last_column1 |
    /// | this21    this22    last_column2 |
    /// | last_row1 last_row2 last_column3 |
    /// ```
    ///
    /// __Defaults__
    ///
    /// `last_column` defaults to (0, 0, 1)
    ///
    /// `last_row` defaults to (0, 0)
    ///
    /// ```txt
    /// | this11    this12    0 |
    /// | this21    this22    0 |
    /// | 0         0         1 |
    /// ```
    #[wasm_bindgen(js_name=toMat3)]
    pub fn to_mat3(&self, last_column: Option<Vec3>, last_row: Option<Vec2>) -> Mat3f {
        let [x1, x2] = self.x_axis;
        let [y1, y2] = self.y_axis;

        let last = last_row.unwrap_or_else(|| Vec2::new(0., 0.));
        Mat3f {
            x_axis: [x1, x2, last.x],
            y_axis: [y1, y2, last.y],
            w_axis: last_column
                .map(|v| v.into())
                .unwrap_or_else(|| [0., 0., 1.]),
        }
    }

    #[wasm_bindgen]
    pub fn scale(a: f32) -> Self {
        Self {
            x_axis: [a, 0.],
            y_axis: [0., a],
        }
    }

    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Mat2f) {
        swap(self, other);
    }

    #[wasm_bindgen]
    pub fn axis(&self, col: usize) -> Vec2 {
        match col {
            0 => self.x_axis.into(),
            1 => self.y_axis.into(),
            _ => unreachable!(),
        }
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.axis(col)[row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        self.axis_mut(col)[row] = val;
    }

    /// `v*M` where `M` is self
    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod(&self, v: Vec2) -> Vec2 {
        [
            v[0] * self.at(0, 0) + v[1] * self.at(0, 1),
            v[0] * self.at(1, 0) + v[1] * self.at(1, 1),
        ]
        .into()
    }

    /// `M*v` where `M` is self
    #[wasm_bindgen(js_name=rightProd)]
    pub fn right_prod(&self, v: Vec2) -> Vec2 {
        [
            v[0] * self.at(0, 0) + v[1] * self.at(1, 0),
            v[0] * self.at(0, 1) + v[1] * self.at(1, 1),
        ]
        .into()
    }

    /// Calculate the determinant
    #[wasm_bindgen]
    pub fn det(&self) -> f32 {
        self.x_axis[0] * self.y_axis[1] - self.x_axis[1] * self.y_axis[0]
    }

    /// Returns a new matrix which is the inverse of this.
    ///
    /// If `this` is not invertible then __null__ is returned.
    #[wasm_bindgen]
    pub fn inverted(&self) -> Option<Mat2f> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }

        let [u11, u21] = self.x_axis;
        let [u12, u22] = self.y_axis;

        let dev_inv = 1.0 / det;

        let mat = Self {
            x_axis: [dev_inv * u22, -dev_inv * u21],
            y_axis: [-dev_inv * u12, dev_inv * u11],
        };

        Some(mat)
    }

    /// Returns the identity matrix
    #[wasm_bindgen]
    pub fn identity() -> Mat2f {
        Mat2f {
            x_axis: [1., 0.],
            y_axis: [0., 1.],
        }
    }

    /// Returns a new matrix that is the transponent of this
    #[wasm_bindgen]
    pub fn transposed(&self) -> Self {
        Self {
            x_axis: [self.x_axis[0], self.y_axis[0]],
            y_axis: [self.x_axis[1], self.y_axis[1]],
        }
    }

    /// Calculate `A*B=C` where `A` is self
    #[wasm_bindgen(js_name=matMul)]
    #[allow(non_snake_case)]
    pub fn mat_mul(&self, B: &Mat2f) -> Mat2f {
        let mut C = Self::default();
        for c in 0..2 {
            for r in 0..2 {
                let x = C.at_mut(c, r);
                *x = self.at(0, r) * B.at(c, 0) + self.at(1, r) * B.at(c, 1);
            }
        }
        C
    }

    /// Check if the two matrices are equal, within `epsilon` range.
    #[wasm_bindgen(js_name=almostEqual)]
    pub fn almost_equal(&self, other: &Mat2f, epsilon: f32) -> bool {
        self.x_axis
            .iter()
            .zip(other.x_axis.iter())
            .all(|(a, b)| (a - b).abs() < epsilon)
            && self
                .y_axis
                .iter()
                .zip(other.y_axis.iter())
                .all(|(a, b)| (a - b).abs() < epsilon)
    }
}

impl<'a> Mul<Mat2f> for &'a Mat2f {
    type Output = Mat2f;

    fn mul(self, rhs: Mat2f) -> Self::Output {
        self.mat_mul(&rhs)
    }
}

impl<'a> Mul<&'a Mat2f> for &'a Mat2f {
    type Output = Mat2f;

    fn mul(self, rhs: &'a Mat2f) -> Self::Output {
        self.mat_mul(rhs)
    }
}

impl<'a> Mul<&'a Mat2f> for Mat2f {
    type Output = Mat2f;

    fn mul(self, rhs: &'a Mat2f) -> Self::Output {
        self.mat_mul(rhs)
    }
}

impl Mul<Mat2f> for Mat2f {
    type Output = Mat2f;

    fn mul(self, rhs: Mat2f) -> Self::Output {
        self.mat_mul(&rhs)
    }
}

impl Mul<f32> for Mat2f {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Mat2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x *= rhs);
        self.y_axis.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl Div<f32> for Mat2f {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Mat2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x /= rhs);
        self.y_axis.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<&Self> for Mat2f {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Self> for Mat2f {
    fn add_assign(&mut self, rhs: &Self) {
        self.x_axis
            .iter_mut()
            .zip(rhs.x_axis.iter())
            .for_each(|(a, b)| *a += b);
        self.y_axis
            .iter_mut()
            .zip(rhs.y_axis.iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl Sub<&Self> for Mat2f {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<&Self> for Mat2f {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x_axis
            .iter_mut()
            .zip(rhs.x_axis.iter())
            .for_each(|(a, b)| *a -= b);
        self.y_axis
            .iter_mut()
            .zip(rhs.y_axis.iter())
            .for_each(|(a, b)| *a -= b);
    }
}
