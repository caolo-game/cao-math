//! Basic 3 by 3 float matrices
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use wasm_bindgen::prelude::*;

/// 3 by 3 column major matrix
#[wasm_bindgen(js_class=Mat3f)]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Mat3f {
    #[wasm_bindgen(skip)]
    pub x_axis: [f32; 3],
    #[wasm_bindgen(skip)]
    pub y_axis: [f32; 3],
    #[wasm_bindgen(skip)]
    pub w_axis: [f32; 3],
}

impl From<[[f32; 3]; 3]> for Mat3f {
    fn from([x_axis, y_axis, w_axis]: [[f32; 3]; 3]) -> Self {
        Self {
            x_axis,
            y_axis,
            w_axis,
        }
    }
}

#[wasm_bindgen(js_class=Mat3f)]
impl Mat3f {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen(js_name=xAxis)]
    pub fn x_axis(&self) -> Vec3 {
        self.x_axis.into()
    }

    #[wasm_bindgen(js_name=yAxis)]
    pub fn y_axis(&self) -> Vec3 {
        self.y_axis.into()
    }

    #[wasm_bindgen(js_name=wAxis)]
    pub fn w_axis(&self) -> Vec3 {
        self.w_axis.into()
    }

    /// Returns a flat list in column-major order
    #[wasm_bindgen(js_name=asList)]
    pub fn as_list(&self) -> Vec<f32> {
        let mut v = Vec::with_capacity(9);
        v.extend_from_slice(&self.x_axis);
        v.extend_from_slice(&self.y_axis);
        v.extend_from_slice(&self.w_axis);
        v
    }

    /// Returns a new matrix which is the inverse of this.
    ///
    /// If `this` is not invertible then __null__ is returned.
    #[wasm_bindgen]
    pub fn inverted(&self) -> Option<Mat3f> {
        let det = self.det();
        if det == 0.0 {
            return None;
        }

        let u = Vec3::from(self.x_axis);
        let v = Vec3::from(self.y_axis);
        let w = Vec3::from(self.w_axis);

        let dev_inv = 1.0 / det;
        let x_axis = v.cross(&w) * dev_inv;
        let y_axis = w.cross(&u) * dev_inv;
        let w_axis = u.cross(&v) * dev_inv;

        let mat = Self {
            x_axis: x_axis.into(),
            y_axis: y_axis.into(),
            w_axis: w_axis.into(),
        }
        .transposed();

        Some(mat)
    }

    /// Returns the identity matrix
    #[wasm_bindgen]
    pub fn identity() -> Mat3f {
        Mat3f::scale(1.)
    }

    /// 2D scale transformation matrix.
    #[wasm_bindgen]
    pub fn scale(a: f32) -> Self {
        Self {
            // note:
            // what you see is the transposed view of the actual matrix
            x_axis: [a, 0., 0.],
            y_axis: [0., a, 0.],
            w_axis: [0., 0., 1.],
        }
    }

    /// Returns a 2D rotation matrix, rotatig with `rads` radians counter-clockwise around the origin
    #[wasm_bindgen]
    pub fn rotation(rads: f32) -> Self {
        let cos = rads.cos();
        let sin = rads.sin();
        Self {
            // note:
            // what you see is the transposed view of the actual matrix
            x_axis: [cos, sin, 0.],
            y_axis: [-sin, cos, 0.],
            w_axis: [0., 0., 1.],
        }
    }

    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    #[wasm_bindgen]
    pub fn translate(Vec2 { x, y }: Vec2) -> Self {
        Self {
            // note:
            // what you see is the transposed view of the actual matrix
            x_axis: [1., 0., 0.],
            y_axis: [0., 1., 0.],
            w_axis: [x, y, 1.],
        }
    }

    /// Returns a new matrix that is the transponent of this
    #[wasm_bindgen]
    pub fn transposed(&self) -> Self {
        Self {
            x_axis: [self.x_axis[0], self.y_axis[0], self.w_axis[0]],
            y_axis: [self.x_axis[1], self.y_axis[1], self.w_axis[1]],
            w_axis: [self.x_axis[2], self.y_axis[2], self.w_axis[2]],
        }
    }

    /// Swaps the matrices in place
    #[wasm_bindgen]
    pub fn swap(&mut self, other: &mut Mat3f) {
        swap(self, other);
    }

    /// Returns axis 0, 1 or 2. Panics if `col` is greater than 2.
    #[wasm_bindgen]
    pub fn axis(&self, col: usize) -> Vec3 {
        let axis = match col {
            0 => &self.x_axis,
            1 => &self.y_axis,
            2 => &self.w_axis,
            _ => unreachable!(),
        };
        axis.clone().into()
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.axis_ref(col)[row]
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        self.axis_mut(col)[row] = val;
    }

    /// `v*M` where `M` is self
    #[wasm_bindgen(js_name=leftProd)]
    pub fn left_prod(&self, v: &Vec3) -> Vec3 {
        let mut res = [0.0; 3];
        for c in 0..3 {
            res[c] += v[0] * self.at(c, 0);
            res[c] += v[1] * self.at(c, 1);
            res[c] += v[2] * self.at(c, 2);
        }
        res.into()
    }

    /// `M*v` where `M` is self
    #[wasm_bindgen(js_name=rightProd)]
    pub fn right_prod(&self, v: &Vec3) -> Vec3 {
        let mut res = [0.0; 3];
        for r in 0..3 {
            res[r] += v[0] * self.at(0, r);
            res[r] += v[1] * self.at(1, r);
            res[r] += v[2] * self.at(2, r);
        }
        res.into()
    }

    /// Calculate `A*B=C` where `A` is self
    #[wasm_bindgen(js_name=matMul)]
    #[allow(non_snake_case)]
    pub fn mat_mul(&self, B: &Mat3f) -> Mat3f {
        let mut C = Self::default();
        for c in 0..3 {
            for r in 0..3 {
                let x = C.at_mut(c, r);
                *x = self.at(0, r) * B.at(c, 0)
                    + self.at(1, r) * B.at(c, 1)
                    + self.at(2, r) * B.at(c, 2);
            }
        }
        C
    }

    /// Calculate the determinant
    pub fn det(&self) -> f32 {
        (self.x_axis[0]
            * det2(
                [self.y_axis[1], self.y_axis[2]],
                [self.w_axis[1], self.w_axis[2]],
            ))
            - (self.y_axis[0]
                * det2(
                    [self.x_axis[1], self.x_axis[2]],
                    [self.w_axis[1], self.w_axis[2]],
                ))
            + (self.w_axis[0]
                * det2(
                    [self.x_axis[1], self.x_axis[2]],
                    [self.y_axis[1], self.y_axis[2]],
                ))
    }
}

#[inline]
fn det2([x1, y1]: [f32; 2], [x2, y2]: [f32; 2]) -> f32 {
    x1 * y2 - x2 * y1
}

impl Mat3f {
    pub fn axis_mut(&mut self, col: usize) -> &mut [f32; 3] {
        match col {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.w_axis,
            _ => unreachable!(),
        }
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        &mut self.axis_mut(col)[row]
    }

    pub fn axis_ref(&self, col: usize) -> &[f32; 3] {
        match col {
            0 => &self.x_axis,
            1 => &self.y_axis,
            2 => &self.w_axis,
            _ => unreachable!(),
        }
    }
}

impl<'a> Mul<&'a Mat3f> for &'a Mat3f {
    type Output = Mat3f;

    fn mul(self, rhs: &'a Mat3f) -> Self::Output {
        self.mat_mul(rhs)
    }
}

impl<'a> Mul<&'a Mat3f> for Mat3f {
    type Output = Mat3f;

    fn mul(self, rhs: &'a Mat3f) -> Self::Output {
        self.mat_mul(rhs)
    }
}

impl<'a> Mul<Mat3f> for &'a Mat3f {
    type Output = Mat3f;

    fn mul(self, rhs: Mat3f) -> Self::Output {
        self.mat_mul(&rhs)
    }
}

impl Mul<Mat3f> for Mat3f {
    type Output = Mat3f;

    fn mul(self, rhs: Mat3f) -> Self::Output {
        self.mat_mul(&rhs)
    }
}

impl Mul<f32> for Mat3f {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Mat3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x *= rhs);
        self.y_axis.iter_mut().for_each(|x| *x *= rhs);
        self.w_axis.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl Div<f32> for Mat3f {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Mat3f {
    fn div_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x /= rhs);
        self.y_axis.iter_mut().for_each(|x| *x /= rhs);
        self.w_axis.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<&Self> for Mat3f {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Self> for Mat3f {
    fn add_assign(&mut self, rhs: &Self) {
        self.x_axis
            .iter_mut()
            .zip(rhs.x_axis.iter())
            .for_each(|(a, b)| *a += b);
        self.y_axis
            .iter_mut()
            .zip(rhs.y_axis.iter())
            .for_each(|(a, b)| *a += b);
        self.w_axis
            .iter_mut()
            .zip(rhs.w_axis.iter())
            .for_each(|(a, b)| *a += b);
    }
}

impl Sub<&Self> for Mat3f {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<&Self> for Mat3f {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x_axis
            .iter_mut()
            .zip(rhs.x_axis.iter())
            .for_each(|(a, b)| *a -= b);
        self.y_axis
            .iter_mut()
            .zip(rhs.y_axis.iter())
            .for_each(|(a, b)| *a -= b);
        self.w_axis
            .iter_mut()
            .zip(rhs.w_axis.iter())
            .for_each(|(a, b)| *a -= b);
    }
}
