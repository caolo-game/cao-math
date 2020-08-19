use super::mat3::Matrix;
use super::mat3_upper::UpperMatrix;
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use wasm_bindgen::prelude::*;

pub enum Payload {
    Matrix(Matrix),
    Upper(UpperMatrix),
}

#[wasm_bindgen(js_name=Mat3f)]
pub struct JsMatrix {
    #[wasm_bindgen(skip)]
    pub val: Payload,
}

#[wasm_bindgen(js_class=Mat3f)]
impl JsMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Payload::Matrix(Default::default());
        Self { val }
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        match &self.val {
            Payload::Matrix(mat) => mat.at(col, row),
            Payload::Upper(mat) => mat.at(col, row),
        }
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, a: f32) {
        match &mut self.val {
            Payload::Matrix(mat) => *mat.at_mut(col, row) = a,
            Payload::Upper(mat) => *mat.at_mut(col, row) = a,
        }
    }

    #[wasm_bindgen(js_name=leftProd)]
    /// Calculate the `a*M` product
    pub fn left_prod(&self, a: &Vec3) -> Vec3 {
        let a = match &self.val {
            Payload::Matrix(mat) => mat.left_prod(a.into()),
            Payload::Upper(_) => unreachable!(),
        };
        Vec3::from(a)
    }

    #[wasm_bindgen(js_name=rightProd)]
    /// Calculate the `M*a` product
    pub fn right_prod(&self, a: &Vec3) -> Vec3 {
        let a = match &self.val {
            Payload::Matrix(mat) => mat.right_prod(a.into()),
            Payload::Upper(mat) => mat.right_prod(a.into()),
        };
        Vec3::from(a)
    }

    #[wasm_bindgen(js_name=scaleMatrix)]
    pub fn scale(a: f32) -> Self {
        Matrix::scale(a).into()
    }

    #[wasm_bindgen(js_name=translateMatrix)]
    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    pub fn translate(t: &Vec2) -> Self {
        Matrix::translate(t.into()).into()
    }

    /// Multiply by scalar value
    #[wasm_bindgen(js_name=scalarMul)]
    pub fn scalar_mul(&mut self, f: f32) {
        match &mut self.val {
            Payload::Matrix(mat) => *mat *= f,
            Payload::Upper(mat) => *mat *= f,
        }
    }

    /// Multiply two matrices
    #[wasm_bindgen(js_name=matrixMul)]
    pub fn mul(&self, b: &JsMatrix) -> JsMatrix {
        let mut res = Matrix::default();
        match (&self.val, &b.val) {
            (Payload::Matrix(a), Payload::Matrix(b)) => {
                a.mat_mul(&b, &mut res);
            }
            _ => unimplemented!(),
        }
        res.into()
    }
}

impl From<Matrix> for JsMatrix {
    fn from(val: Matrix) -> Self {
        let val = Payload::Matrix(val);
        Self { val }
    }
}
