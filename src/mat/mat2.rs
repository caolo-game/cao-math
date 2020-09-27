//! Basic 2 by 2 float matrices
use crate::vec::vec2::Vec2;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

/// 2 by 2 column major matrix
#[wasm_bindgen(js_name=Mat2f)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mat22 {
    #[wasm_bindgen(skip)]
    pub x_axis: [f32; 2],
    #[wasm_bindgen(skip)]
    pub y_axis: [f32; 2],
}

impl From<[[f32; 2]; 2]> for Mat22 {
    fn from([x_axis, y_axis]: [[f32; 2]; 2]) -> Self {
        Self { x_axis, y_axis }
    }
}
impl Mat22 {
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
impl Mat22 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen]
    pub fn scale(a: f32) -> Self {
        Self {
            x_axis: [a, 0.],
            y_axis: [0., a],
        }
    }

    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Mat22) {
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

impl Div<f32> for Mat22 {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Mat22 {
    fn div_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x /= rhs);
        self.y_axis.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<&Self> for Mat22 {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Self> for Mat22 {
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

impl Sub<&Self> for Mat22 {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<&Self> for Mat22 {
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
