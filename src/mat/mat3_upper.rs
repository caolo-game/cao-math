//! Basic 3 by 3 float upper-triangle matrices
use super::mat3::Matrix;
use serde_derive::{Deserialize, Serialize};
use std::hint::unreachable_unchecked;
use std::mem::swap;
use std::ops::{Mul, MulAssign};

type Storage = ([f32; 3], [f32; 2], [f32; 1]);

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpperMatrix {
    pub values: Storage,
}

impl From<Storage> for UpperMatrix {
    fn from(values: Storage) -> Self {
        Self { values }
    }
}

#[derive(Debug)]
pub enum UpperMatrixError {
    MatrixNotTriangle,
}

impl UpperMatrix {
    pub fn from_mat(mat: &Matrix) -> Result<Self, UpperMatrixError> {
        for r in 1..3 {
            for c in 0..r {
                if mat.at(c, r) != 0.0 {
                    return Err(UpperMatrixError::MatrixNotTriangle);
                }
            }
        }
        Ok(Self {
            values: (
                mat.values[0],
                [mat.values[1][1], mat.values[1][2]],
                [mat.values[2][2]],
            ),
        })
    }

    pub fn scale(a: f32) -> Self {
        Self {
            values: ([a, 0., 0.], [a, 0.], [a]),
        }
    }

    pub fn swap(&mut self, other: &mut UpperMatrix) {
        swap(self, other);
    }

    pub fn at(&self, col: usize, row: usize) -> f32 {
        assert!(col < 3);
        assert!(row < 3);
        assert!(row <= col);
        match row {
            0 => self.values.0[col],
            1 => self.values.1[col - 1],
            2 => self.values.2[col - 2],
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn at_mut(&mut self, col: usize, row: usize) -> &mut f32 {
        assert!(col < 3);
        assert!(row < 3);
        assert!(row <= col);
        match row {
            0 => &mut self.values.0[col],
            1 => &mut self.values.1[col - 1],
            2 => &mut self.values.2[col - 2],
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn set(&mut self, col: usize, row: usize, val: f32) {
        *self.at_mut(col, row) = val;
    }

    /// `M*v` where `M` is self and `v` is interpreted as `vT`
    pub fn right_prod(&self, v: [f32; 3]) -> [f32; 3] {
        let mut res = [0.0; 3];
        for r in 0..3 {
            res[0] += v[r] * self.values.0[r];
        }
        for r in 1..3 {
            res[1] += v[r] * self.values.1[r - 1];
        }
        res[2] += v[2] * self.values.2[0];
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
            values: ([1., 0., x], [1., y], [1.]),
        }
    }
}

impl Mul<f32> for UpperMatrix {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f32> for UpperMatrix {
    fn mul_assign(&mut self, rhs: f32) {
        self.values.0.iter_mut().for_each(|x| *x *= rhs);
        self.values.1.iter_mut().for_each(|x| *x *= rhs);
        self.values.2[0] *= rhs;
    }
}
