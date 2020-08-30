//! Basic 2 by 2 float matrices
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Mul, MulAssign};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Mat22 {
    // column major storage
    pub x_axis: [f32; 2],
    pub y_axis: [f32; 2],
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

    pub fn axis(&self, col: usize) -> &[f32; 2] {
        match col {
            0 => &self.x_axis,
            1 => &self.y_axis,
            _ => unreachable!(),
        }
    }

    pub fn axis_mut(&mut self, col: usize) -> &mut [f32; 2] {
        match col {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            _ => unreachable!(),
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
