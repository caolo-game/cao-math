use super::mat3::Mat33;
use crate::vec::vec2::Vec2;
use crate::vec::vec3::Vec3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=Mat3f)]
pub struct JsMat33 {
    #[wasm_bindgen(skip)]
    pub val: Mat33,
}

#[wasm_bindgen(js_class=Mat3f)]
impl JsMat33 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let val = Default::default();
        Self { val }
    }

    #[wasm_bindgen]
    pub fn at(&self, col: usize, row: usize) -> f32 {
        self.val.at(col, row)
    }

    #[wasm_bindgen]
    pub fn set(&mut self, col: usize, row: usize, a: f32) {
        self.val.set(col, row, a);
    }

    #[wasm_bindgen(js_name=leftProd)]
    /// Calculate the `a*M` product
    pub fn left_prod(&self, a: &Vec3) -> Vec3 {
        let a = self.val.left_prod(a.into());
        Vec3::from(a)
    }

    #[wasm_bindgen(js_name=rightProd)]
    /// Calculate the `M*a` product
    pub fn right_prod(&self, a: &Vec3) -> Vec3 {
        let a = self.val.right_prod(a.into());
        Vec3::from(a)
    }

    #[wasm_bindgen(js_name=scaleMat33)]
    pub fn scale(a: f32) -> Self {
        Mat33::scale(a).into()
    }

    #[wasm_bindgen(js_name=translateMat33)]
    /// Creates a matrix for the given translation `t`
    /// Where `b = M*a` equals `a+t`
    pub fn translate(t: &Vec2) -> Self {
        Mat33::translate(t.into()).into()
    }

    /// Multiply by scalar value
    #[wasm_bindgen(js_name=scalarMul)]
    pub fn scalar_mul(&mut self, f: f32) {
        self.val *= f;
    }

    /// Multiply two matrices
    #[wasm_bindgen(js_name=matrixMul)]
    pub fn mul(&self, b: &JsMat33) -> JsMat33 {
        let mut res = Mat33::default();
        self.val.mat_mul(&b.val, &mut res);
        res.into()
    }
}

impl From<Mat33> for JsMat33 {
    fn from(val: Mat33) -> Self {
        Self { val }
    }
}
