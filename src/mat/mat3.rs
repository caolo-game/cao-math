//! Basic 3 by 3 float matrices
use serde_derive::{Deserialize, Serialize};
use std::mem::swap;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Mat33 {
    // since these are just arrays we can use a 2d array instead of a flat buffer
    // without any problems down the line
    //
    // column major storage
    pub x_axis: [f32; 3],
    pub y_axis: [f32; 3],
    pub w_axis: [f32; 3],
}

impl From<[[f32; 3]; 3]> for Mat33 {
    fn from([x_axis, y_axis, w_axis]: [[f32; 3]; 3]) -> Self {
        Self {
            x_axis,
            y_axis,
            w_axis,
        }
    }
}

impl Mat33 {
    pub fn scale(a: f32) -> Self {
        Self {
            // note:
            // what you see is the transposed view of the actual matrix
            x_axis: [a, 0., 0.],
            y_axis: [0., a, 0.],
            w_axis: [0., 0., a],
        }
    }

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
    pub fn translate([x, y]: [f32; 2]) -> Self {
        Self {
            // note:
            // what you see is the transposed view of the actual matrix
            x_axis: [1., 0., 0.],
            y_axis: [0., 1., 0.],
            w_axis: [x, y, 1.],
        }
    }

    pub fn swap(&mut self, other: &mut Mat33) {
        swap(self, other);
    }

    pub fn axis(&self, col: usize) -> &[f32; 3] {
        match col {
            0 => &self.x_axis,
            1 => &self.y_axis,
            2 => &self.w_axis,
            _ => unreachable!(),
        }
    }

    pub fn axis_mut(&mut self, col: usize) -> &mut [f32; 3] {
        match col {
            0 => &mut self.x_axis,
            1 => &mut self.y_axis,
            2 => &mut self.w_axis,
            _ => unreachable!(),
        }
    }

    pub fn at(&self, col: usize, row: usize) -> f32 {
        assert!(col < 3);
        assert!(row < 3);
        self.axis(col)[row]
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        assert!(col < 3);
        assert!(row < 3);
        &mut self.axis_mut(col)[row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        assert!(col < 3);
        assert!(row < 3);
        self.axis_mut(col)[row] = val;
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
    pub fn mat_mul(&self, B: &Self) -> Self {
        let mut C = Self::default();
        for c in 0..3 {
            for r in 0..3 {
                let x = C.at_mut(c, r);
                *x = 0.;
                for i in 0..3 {
                    *x += self.at(i, r) * B.at(c, i);
                }
            }
        }
        C
    }
}

impl Mul<f32> for Mat33 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for Mat33 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x *= rhs);
        self.y_axis.iter_mut().for_each(|x| *x *= rhs);
        self.w_axis.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl Div<f32> for Mat33 {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f32> for Mat33 {
    fn div_assign(&mut self, rhs: f32) {
        self.x_axis.iter_mut().for_each(|x| *x /= rhs);
        self.y_axis.iter_mut().for_each(|x| *x /= rhs);
        self.w_axis.iter_mut().for_each(|x| *x /= rhs);
    }
}

impl Add<&Self> for Mat33 {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<&Self> for Mat33 {
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

impl Sub<&Self> for Mat33 {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<&Self> for Mat33 {
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
