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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        assert!(col < 2);
        assert!(row < 2);
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
        assert!(col < 2);
        assert!(row < 2);
        self.axis(col)[row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        assert!(col < 2);
        assert!(row < 2);
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
